// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file).

use accesskit::{ActionHandler, TreeUpdate};
use windows::Win32::Foundation::HWND;

pub struct Adapter;

impl Adapter {
    pub fn new(
        _hwnd: HWND,
        _source: Box<dyn FnOnce() -> TreeUpdate>,
        _action_handler: Box<dyn ActionHandler>,
    ) -> Self {
        Self {}
    }

    pub fn update(&self, _update: TreeUpdate) {}

    pub fn update_if_active(&self, _updater: impl FnOnce() -> TreeUpdate) {}
}

pub(crate) struct MainThreadCallbackSender;

impl MainThreadCallbackSender {
    pub(crate) fn send(&self, f: impl FnOnce()) {
        f()
    }
}

pub(crate) struct MainThreadCallbackReceiver;

pub(crate) fn main_thread_callback_channel() -> (MainThreadCallbackSender, MainThreadCallbackReceiver) {
    (MainThreadCallbackSender {}, MainThreadCallbackReceiver {})
}
