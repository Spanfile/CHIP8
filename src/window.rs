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
        // println!("dispatch");
        unsafe {
            let msg: winuser::LPMSG = mem::uninitialized();

            while user32::PeekMessageW(
                msg,                // lpMsg
                null_mut(),         // hWnd
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

    pub fn invalidate(&self) {
        unsafe {
            user32::InvalidateRect(self.hwnd, null(), 0);
        }
    }

    pub fn clear(&mut self) {
        self.screen.clear();
        self.invalidate();
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, on: bool) {
        self.screen.set_pixel(x, y, on);
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> bool {
        self.screen.get_pixel(x, y)
    }

    pub fn get_width(&self) -> i32 {
        self.screen.width
    }

    pub fn get_height(&self) -> i32 {
        self.screen.height
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
                // println!("paint");
                let hdc = user32::BeginPaint(h_wnd, &mut ps);
                let screen =
                    &*(user32::GetWindowLongPtrA(h_wnd, winuser::GWLP_USERDATA) as *const Screen);

                gdi32::SelectObject(hdc, gdi32::GetStockObject(wingdi::BLACK_BRUSH));
                gdi32::SelectObject(hdc, gdi32::GetStockObject(wingdi::BLACK_PEN));

                gdi32::Rectangle(
                    hdc,
                    0,
                    0,
                    screen.width * screen.scale,
                    screen.height * screen.scale,
                );

                gdi32::SelectObject(hdc, gdi32::GetStockObject(wingdi::WHITE_BRUSH));
                gdi32::SelectObject(hdc, gdi32::GetStockObject(wingdi::WHITE_PEN));

                for (i, pixel) in screen.buffer.iter().enumerate() {
                    if *pixel {
                        let x = (i as i32) % screen.width;
                        let y = (i as i32) / screen.width;
                        gdi32::Rectangle(
                            hdc,
                            x * screen.scale,
                            y * screen.scale,
                            (x + 1) * screen.scale,
                            (y + 1) * screen.scale,
                        );
                    }
                }

                user32::EndPaint(h_wnd, &ps as *const winuser::PAINTSTRUCT);
                0 as winapi::LRESULT
            }
            winuser::WM_DESTROY => {
                user32::PostQuitMessage(0);
                0 as winapi::LRESULT
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

        let class_style = winuser::CS_HREDRAW | winuser::CS_VREDRAW;
        let window_style = winuser::WS_CAPTION
            | winuser::WS_VISIBLE
            | winuser::WS_MINIMIZEBOX
            | winuser::WS_SYSMENU;
        let icon = user32::LoadIconW(null_mut(), winuser::IDI_APPLICATION);
        let cursor = user32::LoadCursorW(null_mut(), winuser::IDC_ARROW);
        // let background = gdi32::CreateSolidBrush(wingdi::RGB(0, 0, 0));

        let name = winstr(name);
        let title = winstr(title);

        let wndc = winuser::WNDCLASSEXW {
            cbSize: mem::size_of::<winuser::WNDCLASSEXW>() as u32,
            style: class_style,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: icon,
            hCursor: cursor,
            //hbrBackground: background,
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
            lpszClassName: name.as_ptr(),
            hIconSm: null_mut(),
        };

        user32::RegisterClassExW(&wndc);

        let mut client_rect: windef::RECT = mem::uninitialized();
        client_rect.left = 0;
        client_rect.top = 0;
        client_rect.right = screen.width * screen.scale;
        client_rect.bottom = screen.height * screen.scale;
        // println!(
        //     "{},{}, {} by {}",
        //     client_rect.left, client_rect.top, client_rect.right, client_rect.bottom
        // );
        user32::AdjustWindowRectEx(&mut client_rect, window_style, 0, 0);
        // println!(
        //     "{},{}, {} by {}",
        //     client_rect.left, client_rect.top, client_rect.right, client_rect.bottom
        // );

        let hwnd = user32::CreateWindowExW(
            winuser::WS_EX_OVERLAPPEDWINDOW,      // dwExStyle
            name.as_ptr(),                        // lpClassName
            title.as_ptr(),                       // lpWindowName
            window_style,                         // dwStyle
            winuser::CW_USEDEFAULT,               // x
            winuser::CW_USEDEFAULT,               // y
            client_rect.right - client_rect.left, // nWidth
            client_rect.bottom - client_rect.top, // nHeight
            null_mut(),                           // hWndParent
            null_mut(),                           // hMenu
            h_instance,                           // hInstance
            null_mut(),                           // lpParam
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
