use accesskit::{ActionHandler, ActionRequest, TreeUpdate};
#[cfg(any(target_os = "macos"))]
use accesskit_macos::SubclassingAdapter;
#[cfg(any(target_os = "linux"))]
use accesskit_unix::Adapter as SubclassingAdapter;
#[cfg(any(target_os = "windows"))]
use accesskit_windows::{SubclassingAdapter, HWND};
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    os::raw::c_void,
    ptr,
};

#[cfg(any(target_os = "windows"))]
type DisplayHandle = *mut c_void; // Unused, always nullptr.
#[cfg(any(target_os = "macos"))]
type DisplayHandle = *mut c_void; // Unused, always nullptr.
#[cfg(any(target_os = "linux"))]
type DisplayHandle = *mut c_void;

#[cfg(any(target_os = "windows"))]
type WindowHandle = HWND;
#[cfg(any(target_os = "macos"))]
type WindowHandle = *mut c_void;
#[cfg(any(target_os = "linux"))]
type WindowHandle = *mut c_void;

#[cfg(any(target_os = "windows"))] // Unused, always nullptr.
type ViewHandle = *mut c_void;
#[cfg(any(target_os = "macos"))]
type ViewHandle = *mut c_void;
#[cfg(any(target_os = "linux"))] // Unused, always nullptr.
type ViewHandle = *mut c_void;

type ActionHandlerCallback = extern "C" fn(*mut c_void, *const c_char);

struct GDActionHandler {
    callback: ActionHandlerCallback,
    dse: *mut c_void,
}

unsafe impl Send for GDActionHandler {}
unsafe impl Sync for GDActionHandler {}
impl ActionHandler for GDActionHandler {
    fn do_action(&self, request: ActionRequest) {
        let request = serde_json::to_string(&request).unwrap();
        let request = CString::new(request).unwrap();
        let callback = self.callback;
        let dse = self.dse;
        let ptr = request.as_ptr();
        callback(dse, ptr);
        std::mem::forget(request);
    }
}

fn tree_update_from_json(json: *const c_char) -> Option<TreeUpdate> {
    let json = unsafe { CStr::from_ptr(json).to_str() }.ok()?;
    serde_json::from_str::<TreeUpdate>(json).ok()
}

pub struct GDAdapter {
    adapter: SubclassingAdapter,
}

impl GDAdapter {
    pub fn new(
        dse_ptr: *mut c_void,
        #[allow(unused_variables)] disp_handle: DisplayHandle,
        #[allow(unused_variables)] wnd_handle: WindowHandle,
        #[allow(unused_variables)] view_handle: ViewHandle,
        source: Box<dyn FnOnce() -> TreeUpdate + Send>,
        action_handler: ActionHandlerCallback,
        #[allow(unused_variables)] app_name: String,
    ) -> Self {
        let action_handler = GDActionHandler {
            callback: action_handler,
            dse: dse_ptr,
        };
        #[cfg(target_os = "macos")]
        let adapter =
            unsafe { SubclassingAdapter::new(view_handle, source, Box::new(action_handler)) };
        #[cfg(target_os = "windows")]
        let adapter = SubclassingAdapter::new(wnd_handle, source, Box::new(action_handler));
        #[cfg(target_os = "linux")]
        let adapter =
            SubclassingAdapter::new(app_name, "Godot".to_string(), "4.0".to_string(), source, Box::new(action_handler)).unwrap();
        Self { adapter }
    }

    pub fn update(&self, update: TreeUpdate) {
        #[cfg(not(target_os = "linux"))]
        self.adapter.update(update).raise();

        #[cfg(target_os = "linux")]
        self.adapter.update(update);
    }
}

#[no_mangle]
extern "C" fn accesskit_init(
    dse: *mut c_void,
    disp_handle: DisplayHandle,
    wnd_handle: WindowHandle,
    view_handle: ViewHandle,
    action_handler: ActionHandlerCallback,
    initial_tree_update: *const c_char,
    app_name: *const c_char,
) -> *mut c_void {
    let tree_update = match tree_update_from_json(initial_tree_update) {
        Some(tree_update) => tree_update,
        _ => return ptr::null_mut(),
    };
    let app_name = unsafe { CStr::from_ptr(app_name).to_str() }.unwrap();
    let adapter = Box::new(GDAdapter::new(
        dse,
        disp_handle,
        wnd_handle,
        view_handle,
        Box::new(move || tree_update),
        action_handler,
        app_name.to_string(),
    ));
    Box::into_raw(adapter) as *mut c_void
}

#[no_mangle]
extern "C" fn accesskit_push_update(
    _dse: *mut c_void,
    adapter_ptr: *mut c_void,
    tree_update: *const c_char,
) -> bool {
    let tree_update = match tree_update_from_json(tree_update) {
        Some(tree_update) => tree_update,
        _ => return false,
    };
    let adapter = unsafe { Box::from_raw(adapter_ptr as *mut GDAdapter) };
    adapter.update(tree_update);
    Box::into_raw(adapter);
    true
}

#[no_mangle]
extern "C" fn accesskit_destroy(_dse: *mut c_void, adapter_ptr: *mut c_void) {
    unsafe { Box::from_raw(adapter_ptr as *mut GDAdapter) };
}
