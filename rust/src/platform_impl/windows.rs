// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file).

use accesskit::{ActionHandler, TreeUpdate};
use accesskit_windows::{Adapter as WindowsAdapter, SubclassingAdapter};
use lazy_static::lazy_static;
use windows::{core::*, Win32::{Foundation::*, System::{LibraryLoader::GetModuleHandleW}, UI::WindowsAndMessaging::*}};

pub struct Adapter {
    adapter: SubclassingAdapter,
}

impl Adapter {
    pub fn new(
        hwnd: HWND,
        source: Box<dyn FnOnce() -> TreeUpdate>,
        action_handler: Box<dyn ActionHandler>,
    ) -> Self {
        let adapter = WindowsAdapter::new(hwnd, source, action_handler);
        let adapter = SubclassingAdapter::new(adapter);
        Self { adapter }
    }

    pub fn update(&self, update: TreeUpdate) {
        self.adapter.update(update).raise();
    }

    pub fn update_if_active(&self, updater: impl FnOnce() -> TreeUpdate) {
        self.adapter.update_if_active(updater).raise();
    }
}

// The following is loosely based on EventLoopThreadExecutor in winit.

// Double-box because the inner box is fat, and we need a plain pointer.
type InnerBoxedCallback = Box<dyn FnOnce() + Send>;
type BoxedCallback = Box<InnerBoxedCallback>;

extern "system" fn callback_receiver_wnd_proc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match message as u32 {
        WM_USER => {
            let callback: BoxedCallback = unsafe { Box::from_raw(lparam.0 as *mut _) };
            callback();
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(window, message, wparam, lparam) },
    }
}

lazy_static! {
    static ref WIN32_INSTANCE: HINSTANCE = {
        unsafe { GetModuleHandleW(None) }.unwrap()
    };

    static ref DEFAULT_CURSOR: HCURSOR = {
        unsafe { LoadCursorW(None, IDC_ARROW) }.unwrap()
    };

    static ref CALLBACK_RECEIVER_WINDOW_CLASS_ATOM: u16 = {
        // The following is a combination of the implementation of
        // IntoParam<PWSTR> and the class registration function from winit.
        let class_name_wsz: Vec<_> = "AccessKitCallbackReceiver"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();

        let wc = WNDCLASSW {
            hCursor: *DEFAULT_CURSOR,
            hInstance: *WIN32_INSTANCE,
            lpszClassName: PCWSTR(class_name_wsz.as_ptr() as _),
            lpfnWndProc: Some(callback_receiver_wnd_proc),
            ..Default::default()
        };

        let atom = unsafe { RegisterClassW(&wc) };
        if atom == 0 {
            let result: windows::core::Result<()> = Err(Error::from_win32());
            result.unwrap();
        }
        atom
    };
}
