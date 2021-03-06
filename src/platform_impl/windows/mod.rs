#![cfg(target_os = "windows")]

use winapi;
use winapi::shared::windef::HWND;
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM};

use std::fmt;

pub use self::event_loop::{EventLoop, EventLoopWindowTarget, EventLoopProxy};
pub use self::monitor::MonitorHandle;
pub use self::window::Window;

use window::Icon;
use event::DeviceId as RootDeviceId;

#[derive(Clone, Default)]
pub struct PlatformSpecificWindowBuilderAttributes {
    pub parent: Option<HWND>,
    pub taskbar_icon: Option<Icon>,
    pub no_redirection_bitmap: bool,
}

unsafe impl Send for PlatformSpecificWindowBuilderAttributes {}
unsafe impl Sync for PlatformSpecificWindowBuilderAttributes {}

// Cursor name in UTF-16. Used to set cursor in `WM_SETCURSOR`.
#[derive(Debug, Clone, Copy)]
pub struct Cursor(pub *const winapi::ctypes::wchar_t);
unsafe impl Send for Cursor {}
unsafe impl Sync for Cursor {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceId(u32);

impl DeviceId {
    pub unsafe fn dummy() -> Self {
        DeviceId(0)
    }
}

impl DeviceId {
    pub fn get_persistent_identifier(&self) -> Option<String> {
        if self.0 != 0 {
            raw_input::get_raw_input_device_name(self.0 as _)
        } else {
            None
        }
    }
}

// Constant device ID, to be removed when this backend is updated to report real device IDs.
const DEVICE_ID: RootDeviceId = RootDeviceId(DeviceId(0));

fn wrap_device_id(id: u32) -> RootDeviceId {
    RootDeviceId(DeviceId(id))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowId(HWND);
unsafe impl Send for WindowId {}
unsafe impl Sync for WindowId {}

impl WindowId {
    pub unsafe fn dummy() -> Self {
        use std::ptr::null_mut;

        WindowId(null_mut())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OsSpecificWindowEvent {
    pub(crate) message: UINT,
    pub(crate) wparam: WPARAM,
    pub(crate) lparam: LPARAM,
}

impl fmt::Debug for OsSpecificWindowEvent {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.pad("OsSpecificWindowEvent { .. }")
    }
}

mod dpi;
mod drop_handler;
mod event;
mod event_loop;
mod icon;
mod monitor;
mod raw_input;
mod util;
mod window;
mod window_state;
