// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file).

use accesskit::{ActionHandler, ActionRequest, TreeUpdate};
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    os::raw::c_void
};
use windows::Win32::{
    Foundation::*,
    UI::WindowsAndMessaging::*
};

mod platform_impl;

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

pub struct Adapter {
    adapter: platform_impl::Adapter,
}

impl Adapter {
    pub fn new(
        dse_ptr: *mut c_void,
        hwnd: HWND,
        source: Box<dyn FnOnce() -> TreeUpdate + Send>,
        action_handler: ActionHandlerCallback,
    ) -> Self {
        let action_handler = GDActionHandler {
            callback: action_handler,
            dse: dse_ptr,
        };
        let adapter = platform_impl::Adapter::new(hwnd, source, Box::new(action_handler));
        Self { adapter }
    }

    pub fn update(&self, update: TreeUpdate) {
        self.adapter.update(update)
    }

    pub fn update_if_active(&self, updater: impl FnOnce() -> TreeUpdate) {
        self.adapter.update_if_active(updater)
    }
}

fn tree_update_from_json(json: *const c_char) -> Option<TreeUpdate> {
    let json = unsafe { CStr::from_ptr(json).to_str() }.ok()?;
    serde_json::from_str::<TreeUpdate>(json).ok()
}

const PROP_NAME: &str = "AccessKitGodotPlugin";

#[no_mangle]
extern fn accesskit_init(
    dse: *mut c_void,
    hwnd: HWND,
    action_handler: ActionHandlerCallback,
    initial_tree_update: *const c_char
) -> bool {
    let initial_tree_update = match tree_update_from_json(initial_tree_update) {
        Some(tree_update) => tree_update,
        _ => return false
    };
    let adapter = Box::new(Adapter::new(
        dse,
        hwnd,
        Box::new(move || initial_tree_update),
        action_handler,
    ));
    let ptr = Box::into_raw(adapter);
    if unsafe { SetPropW(hwnd, PROP_NAME, HANDLE(ptr as _)).as_bool() } {
        true
    } else {
        false
    }
}

#[no_mangle]
extern fn accesskit_push_update(_dse: *mut c_void, hwnd: HWND, tree_update: *const c_char) -> bool {
    let tree_update = match tree_update_from_json(tree_update) {
        Some(tree_update) => tree_update,
        _ => return false
    };
    let handle = unsafe { GetPropW(hwnd, PROP_NAME) };
    let adapter = unsafe { Box::from_raw(handle.0 as *mut Adapter) };
    adapter.update(tree_update);
    Box::into_raw(adapter);
    true
}

#[no_mangle]
extern fn accesskit_destroy(_dse: *mut c_void, hwnd: HWND) {
    let handle = unsafe { RemovePropW(hwnd, PROP_NAME) }.unwrap();
    unsafe { Box::from_raw(handle.0 as *mut Adapter) };
}
