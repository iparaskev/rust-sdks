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

use std::sync::{Arc, Mutex};

use crate::imp::desktop_capturer as imp_dc;
use tokio::sync::mpsc;

enum Message {
    StopCapture,
}

struct DesktopCapturerInner {
    handle: imp_dc::DesktopCapturer,
    tx: Option<mpsc::UnboundedSender<Message>>,
}

pub struct DesktopCapturer {
    inner: Arc<Mutex<DesktopCapturerInner>>,
}

impl DesktopCapturer {
    pub fn new<T>(callback: T) -> Self
    where
        T: Fn(DesktopFrame) + Send + 'static,
    {
        let inner_callback = move |frame: imp_dc::DesktopFrame| {
            callback(DesktopFrame::new(frame));
        };
        let inner = Arc::new(Mutex::new((DesktopCapturerInner {
            handle: imp_dc::DesktopCapturer::new(inner_callback),
            tx: None,
        })));
        Self { inner }
    }

    pub fn start_capture(&self) {
        let (tx, mut rx) = mpsc::unbounded_channel();
        {
            let mut inner = self.inner.lock().unwrap();
            inner.tx = Some(tx);
        }
        let inner = self.inner.clone();
        livekit_runtime::spawn(async move {
            loop {
                {
                    let inner = inner.lock().unwrap();
                    inner.handle.capture_frame();
                }

                match rx.try_recv() {
                    Ok(Message::StopCapture) => {
                        break;
                    }
                    Err(_) => {}
                }
            }
        });
    }

    pub fn stop_capture(&self) {
        let mut inner = self.inner.lock().unwrap();
        if let Some(tx) = &inner.tx {
            tx.send(Message::StopCapture).unwrap();
        }
        inner.tx.take();
    }
}

impl Drop for DesktopCapturer {
    fn drop(&mut self) {
        self.stop_capture();
    }
}

pub struct DesktopFrame {
    pub(crate) sys_handle: imp_dc::DesktopFrame,
}

impl DesktopFrame {
    pub fn new(sys_handle: imp_dc::DesktopFrame) -> Self {
        Self { sys_handle }
    }

    pub fn width(&self) -> i32 {
        self.sys_handle.width() as i32
    }
    pub fn height(&self) -> i32 {
        self.sys_handle.height() as i32
    }
    pub fn stride(&self) -> u32 {
        self.sys_handle.stride() as u32
    }
    pub fn data(&self) -> &[u8] {
        &self.sys_handle.data()
    }
}
