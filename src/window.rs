extern crate gdi32;
extern crate kernel32;
extern crate user32;
extern crate winapi;

use self::winapi::wingdi;
use self::winapi::winuser;
use std::ffi::OsStr;
use std::io::Error;
use std::iter::once;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

pub struct Window {
    _handle: winapi::HWND,
}

impl Window {
    pub fn new(name: &str, title: &str, width: i32, height: i32) -> Window {
        Window {
            _handle: match create_window(name, title, width, height) {
                Ok(window_handle) => window_handle,
                Err(error) => panic!("Window creation failed. {:?}", error),
            },
        }
    }

    pub fn dispatch_messages(&self) -> bool {
        unsafe {
            let mut msg: winuser::MSG = mem::uninitialized();

            while user32::PeekMessageA(
                &mut msg as *mut winuser::MSG, // lpMsg
                0 as winapi::HWND,             // hWnd
                0,                             // wMsgFilterMin
                0,                             // wMsgFilterMax
                winuser::PM_REMOVE,            // wRemoveMsg
            ) != 0
            {
                if msg.message == winuser::WM_QUIT {
                    return false;
                }

                user32::TranslateMessage(&msg as *const winuser::MSG);
                user32::DispatchMessageW(&msg as *const winuser::MSG);
            }

            true
        }
    }
}

fn create_window(name: &str, title: &str, width: i32, height: i32) -> Result<winapi::HWND, Error> {
    unsafe {
        let h_instance = kernel32::GetCurrentProcess() as winapi::HINSTANCE;

        let style = winuser::CS_OWNDC | winuser::CS_HREDRAW | winuser::CS_VREDRAW;
        let icon = user32::LoadIconW(null_mut(), winuser::IDI_APPLICATION);
        let cursor = user32::LoadCursorW(null_mut(), winuser::IDC_ARROW);
        let background = gdi32::CreateSolidBrush(wingdi::RGB(0, 0, 0));

        let name = winstr(name);
        let title = winstr(title);

        let wndc = winuser::WNDCLASSW {
            style,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: icon,
            hCursor: cursor,
            hbrBackground: background,
            lpszMenuName: null_mut(),
            lpszClassName: name.as_ptr(),
        };

        user32::RegisterClassW(&wndc);

        let window_handle = user32::CreateWindowExW(
            winuser::WS_EX_OVERLAPPEDWINDOW,                    // dwExStyle
            name.as_ptr(),                                      // lpClassName
            title.as_ptr(),                                     // lpWindowName
            winuser::WS_OVERLAPPEDWINDOW | winuser::WS_VISIBLE, // dwStyle
            winuser::CW_USEDEFAULT,                             // x
            winuser::CW_USEDEFAULT,                             // y
            width,                                              // nWidth
            height,                                             // nHeight
            null_mut(),                                         // hWndParent
            null_mut(),                                         // hMenu
            h_instance,                                         // hInstance
            null_mut(),                                         // lpParam
        );

        if window_handle.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(window_handle)
        }
    }
}

fn winstr(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

unsafe extern "system" fn window_proc(
    h_wnd: winapi::HWND,
    msg: winapi::UINT,
    w_param: winapi::WPARAM,
    l_param: winapi::LPARAM,
) -> winapi::LRESULT {
    let mut ps: winuser::PAINTSTRUCT = mem::uninitialized();

    match msg {
        winuser::WM_PAINT => {
            let hdc = user32::BeginPaint(h_wnd, &mut ps);
            gdi32::SelectObject(hdc, gdi32::GetStockObject(wingdi::WHITE_BRUSH));

            gdi32::Rectangle(hdc, 20, 20, 40, 40);

            user32::EndPaint(h_wnd, &ps as *const winuser::PAINTSTRUCT);
            0
        }
        winuser::WM_DESTROY => {
            user32::PostQuitMessage(0);
            0
        }
        _ => user32::DefWindowProcW(h_wnd, msg, w_param, l_param),
    }
}
