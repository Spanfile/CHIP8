extern crate gdi32;
extern crate kernel32;
extern crate user32;
extern crate winapi;

use self::winapi::windef;
use self::winapi::wingdi;
use self::winapi::winuser;
use screen::Screen;
use std::ffi::OsStr;
use std::io::Error;
use std::iter::once;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null;
use std::ptr::null_mut;

pub struct Window {
    hwnd: windef::HWND,
    screen: Box<Screen>,
}

impl Default for Window {
    fn default() -> Window {
        Window {
            hwnd: null_mut(),
            screen: Default::default(),
        }
    }
}

impl Window {
    pub fn new(name: &str, title: &str, width: i32, height: i32, scale: i32) -> Window {
        let screen = Box::new(Screen::new(width, height, scale));
        Window {
            hwnd: match create_window(name, title, &screen, Window::window_proc) {
                Ok(hwnd) => hwnd,
                Err(error) => panic!("Window creation failed. {:?}", error),
            },
            screen,
        }
    }

    pub fn dispatch_messages(&self) -> bool {
        unsafe {
            let msg: winuser::LPMSG = mem::uninitialized();

            while user32::PeekMessageW(
                msg,                // lpMsg
                0 as winapi::HWND,  // hWnd
                0,                  // wMsgFilterMin
                0,                  // wMsgFilterMax
                winuser::PM_REMOVE, // wRemoveMsg
            ) != 0
            {
                if (*msg).message == winuser::WM_QUIT {
                    return false;
                }

                user32::TranslateMessage(msg as *const winuser::MSG);
                user32::DispatchMessageW(msg as *const winuser::MSG);
            }

            true
        }
    }

    pub fn clear(&mut self) {
        self.screen.clear();
        unsafe {
            user32::InvalidateRect(self.hwnd, null(), 0);
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {
        self.screen.set_pixel(x, y);
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

                let screen =
                    user32::GetWindowLongPtrA(h_wnd, winuser::GWLP_USERDATA) as *const Screen;

                for (i, pixel) in (*screen).buffer.iter().enumerate() {
                    if *pixel {
                        let x = i as i32 % (*screen).width;
                        let y = i as i32 / (*screen).width;
                        gdi32::Rectangle(hdc, x, y, x + (*screen).scale, y + (*screen).scale);
                    }
                }

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
}

fn create_window(
    name: &str,
    title: &str,
    screen: &Screen,
    window_proc: unsafe extern "system" fn(
        h_wnd: winapi::HWND,
        msg: winapi::UINT,
        w_param: winapi::WPARAM,
        l_param: winapi::LPARAM,
    ) -> winapi::LRESULT,
) -> Result<windef::HWND, Error> {
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

        let hwnd = user32::CreateWindowExW(
            winuser::WS_EX_OVERLAPPEDWINDOW,                    // dwExStyle
            name.as_ptr(),                                      // lpClassName
            title.as_ptr(),                                     // lpWindowName
            winuser::WS_OVERLAPPEDWINDOW | winuser::WS_VISIBLE, // dwStyle
            winuser::CW_USEDEFAULT,                             // x
            winuser::CW_USEDEFAULT,                             // y
            screen.width * screen.scale,                        // nWidth
            screen.height * screen.scale,                       // nHeight
            null_mut(),                                         // hWndParent
            null_mut(),                                         // hMenu
            h_instance,                                         // hInstance
            null_mut(),                                         // lpParam
        );

        user32::SetWindowLongPtrA(
            hwnd,
            winuser::GWLP_USERDATA,
            (screen as *const Screen) as i64,
        );

        if hwnd.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(hwnd)
        }
    }
}

fn winstr(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}
