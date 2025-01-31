/*
 * Copyright 2025 LiveKit
 *
 * Licensed under the Apache License, Version 2.0 (the “License”);
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an “AS IS” BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#pragma once
#include <memory>

#include "modules/desktop_capture/desktop_capturer.h"
#include "rust/cxx.h"

namespace livekit {
class DesktopFrame;
class DesktopCapturer;
}  // namespace livekit

#include "webrtc-sys/src/desktop_capturer.rs.h"

namespace livekit {

class DesktopCapturer : public webrtc::DesktopCapturer::Callback {
 public:
  explicit DesktopCapturer(rust::Box<DesktopCapturerCallbackWrapper> callback);

  void OnCaptureResult(webrtc::DesktopCapturer::Result result,
                       std::unique_ptr<webrtc::DesktopFrame> frame) final;

  void capture_frame() const {
    capturer->CaptureFrame();
  };

 private:
  std::unique_ptr<webrtc::DesktopCapturer> capturer;
  rust::Box<DesktopCapturerCallbackWrapper> callback;
};

class DesktopFrame {
 public:
  DesktopFrame(std::unique_ptr<webrtc::DesktopFrame> frame) : frame(std::move(frame)) {};
  int32_t width() const {
    return frame->size().width();
  }

  int32_t height() const {
    return frame->size().height();
  }

  int32_t stride() const {
    return frame->stride();
  }

  const uint8_t* data() const {
    return frame->data();
  }

 private:
  std::unique_ptr<webrtc::DesktopFrame> frame;
};

static std::unique_ptr<DesktopCapturer> new_desktop_capturer(
    rust::Box<DesktopCapturerCallbackWrapper> callback) {
  return std::make_unique<DesktopCapturer>(std::move(callback));
}
}  // namespace livekit