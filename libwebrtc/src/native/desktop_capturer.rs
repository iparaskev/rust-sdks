// Copyright 2025 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::str;

use cxx::UniquePtr;
use webrtc_sys::desktop_capturer::{self as sys_dc, ffi::new_desktop_capturer};

pub struct DesktopCapturer {
    pub(crate) sys_handle: UniquePtr<sys_dc::ffi::DesktopCapturer>,
}

impl DesktopCapturer {
    pub fn new<T>(callback: T) -> Self
        where T: Fn(DesktopFrame) + Send + 'static {
        let callback = DesktopCallback::new(callback);
        let callback_wrapper = sys_dc::DesktopCapturerCallbackWrapper::new(Box::new(callback));
        let sys_handle = new_desktop_capturer(Box::new(callback_wrapper));
        Self { sys_handle }
    }

    pub fn capture_frame(&self) {
        self.sys_handle.capture_frame();
    }
}

pub struct DesktopFrame {
    pub(crate) sys_handle: UniquePtr<sys_dc::ffi::DesktopFrame>,
}

impl DesktopFrame {
    pub fn new(sys_handle: UniquePtr<sys_dc::ffi::DesktopFrame>) -> Self {
        Self { sys_handle }
    }

    pub fn width(&self) -> i32 {
        self.sys_handle.width()
    }

    pub fn height(&self) -> i32 {
        self.sys_handle.height()
    }

    pub fn stride(&self) -> u32 {
        self.sys_handle.stride() as u32
    }

    pub fn data(&self) -> &[u8] {
        let data = self.sys_handle.data();
        unsafe {std::slice::from_raw_parts(data, self.stride() as usize * self.height() as usize)}
    }
}

pub struct DesktopCallback<T: Fn(DesktopFrame) + Send> {
    callback: T,
}

impl<T> DesktopCallback<T>
    where T: Fn(DesktopFrame) + Send {
    pub fn new(callback: T) -> Self {
        Self { callback }
    }
}

impl<T> sys_dc::DesktopCapturerCallback for DesktopCallback<T>
    where T: Fn(DesktopFrame) + Send {
    fn on_capture_result(&self, frame: UniquePtr<sys_dc::ffi::DesktopFrame>) {
        (self.callback)(DesktopFrame::new(frame));
    }
}
