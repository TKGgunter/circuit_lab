#![cfg(target_os = "windows")]



extern crate winapi;
use winapi::um::winuser as user32;
use winapi::um::wingdi as gdi32;

use winapi::shared::windef::{HWND, RECT, HDC};//, HWND__, HDC__}; TODO up for removal
use winapi::um::wingdi::{BITMAPINFO, BITMAPINFOHEADER, SRCCOPY, RGBQUAD};


use std::ptr::{null, null_mut};
use std::iter::once;
use std::mem;


use crate::{WindowCanvas,
            GLOBAL_BACKBUFFER,
            GLOBAL_WINDOWINFO,
            OsPackage};

use crate::lab_sims::*;
use crate::SETICON;


use crate::inputhandler::*;
use crate::rendertools;
use rendertools::*;


use crate::misc::*;




#[inline]
fn new_rgbquad()->RGBQUAD{
     RGBQUAD{
        rgbBlue: 0,
        rgbGreen: 0,
        rgbRed: 0,
        rgbReserved: 0,
     }
}

pub fn make_window(){unsafe{
    //NOTE
    //This entire function was developed with the help of handmade hero eps 1-3
    //https://www.youtube.com/user/handmadeheroarchive
    use winapi::um::libloaderapi as kernel32;

    use user32::{RegisterClassW, CreateWindowExW, TranslateMessage, DispatchMessageW, PeekMessageW, LoadCursorW};
    use winapi::um::winuser::{ WS_EX_ACCEPTFILES, WNDCLASSW, CW_USEDEFAULT, WS_OVERLAPPEDWINDOW, WS_VISIBLE, MSG, IDC_ARROW};
    use winapi::shared::windef::POINT;


    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;







    let instance = kernel32::GetModuleHandleW(null());
    //let mut app_storage = AppStorage::new();
    let mut ls_app_storage = LS_AppStorage::new();
    let mut stopwatch = StopWatch::new();


    //NOTE
    //https://docs.microsoft.com/en-us/windows/desktop/winmsg/window-class-styles
    //
    // + 0x0020 allocates a unique device context for each window in class
    // + 0x0001 redraws window if resize or window movement vertical
    // + 0x0002 redraws window if resize or window movement horizontal 
    let windows_string: Vec<u16> = OsStr::new("XCopyFGCWindowClass").encode_wide().chain(once(0)).collect();
    let windowclass = WNDCLASSW{style: 0x0020u32 | 0x0001u32 | 0x0002u32 | winapi::um::winuser::CS_DBLCLKS,
            lpfnWndProc: Some(window_callback),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: instance,
            hIcon: null_mut(),
            hCursor: LoadCursorW(null_mut(), IDC_ARROW),
            hbrBackground: null_mut(),
            lpszMenuName: null(),
            lpszClassName: windows_string.as_ptr()};

    let rt_registarclassw = RegisterClassW(&windowclass as *const WNDCLASSW);

    if rt_registarclassw == 0 { panic!("Error occurred when attempting to registar class window!"); }
    if rt_registarclassw != 0 {
        let windows_string: Vec<u16> = OsStr::new("XCopyFGC Window").encode_wide().chain(once(0)).collect();
        //NOTE
        //https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-createwindowexa
        //
        //         https://docs.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles
        //WS_EX_ACCEPTFILES           The window accepts drag-drop files. 
        //
        //         https://docs.microsoft.com/en-us/windows/win32/winmsg/window-styles
        //WS_OVERLAPPEDWINDOW         (WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX) The window is an overlapped window. 
        //WS_VISIBLE                  The window is initially visible.
        let window_handle = CreateWindowExW(
                          WS_EX_ACCEPTFILES ,
                          windowclass.lpszClassName,
                          windows_string.as_ptr(),
                          WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                          CW_USEDEFAULT,
                          CW_USEDEFAULT,
                          1000,
                          550,
                          null_mut(),
                          null_mut(),
                          instance,
                          null_mut());



        //let iDpi = user32::GetDpiForSystem(); TODO doesn't work



        if window_handle == null_mut(){ panic!("Error Window_handle did not alloc!"); }
        if window_handle != null_mut(){

            let mut mouseinfo = MouseInfo::new();
            let mut textinfo = TextInfo{character: Vec::with_capacity(10), timing:Vec::new()};
            let mut keyboardinfo = KeyboardInfo{key: Vec::new(), status:Vec::new()};
            let mut old_window_info = GLOBAL_WINDOWINFO;

            let mut frame_counter = 0;
            'a : loop {
                let mut message = MSG{ hwnd: null_mut(), message: 0, wParam: 0, lParam: 0, time: 0, pt: POINT{x:0, y:0} };


                keyboardinfo.key.clear();
                keyboardinfo.status.clear();
                textinfo.character.clear();

                mouseinfo.wheel_delta = 0;
                mouseinfo.old_lbutton = mouseinfo.lbutton.clone(); //TODO(9/1/2020) keep an eye on this. It may cause an input bug for app_main.
                mouseinfo.old_rbutton = mouseinfo.rbutton.clone(); //TODO(9/1/2020) keep an eye on this. It may cause an input bug for app_main.

                //NOTE
                //https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-peekmessagew
                //
                //0x0001 removes message from queue
                mouseinfo.delta_x = mouseinfo.x;
                mouseinfo.delta_y = mouseinfo.y;
                let mut mouse_delta_updated = false;
                let mut n_messages = 0;
                while PeekMessageW(&mut message as *mut MSG, null_mut(), 0, 0, 0x0001) > 0{
                    {//NOTE: Handle mouse events
                        //Convert to the correct coordinates


                        mouseinfo.x = message.pt.x - GLOBAL_WINDOWINFO.x - 10;
                        mouseinfo.y = GLOBAL_WINDOWINFO.h - ( message.pt.y - GLOBAL_WINDOWINFO.y) - 10;

                        if message.message == winapi::um::winuser::WM_MOUSEMOVE{// n_messages == 0 {
                            mouse_delta_updated = true;
                        }
                        

                        //TODO not working .... why?
                        if message.message == winapi::um::winuser::WM_LBUTTONDBLCLK{ mouseinfo.double_lbutton = true; }
                        else { mouseinfo.double_lbutton = false; }
                        /////////////////////

                        if message.message == winapi::um::winuser::WM_LBUTTONDOWN{ mouseinfo.lbutton = ButtonStatus::Down;  }
                        else if message.message == winapi::um::winuser::WM_LBUTTONUP{ mouseinfo.lbutton = ButtonStatus::Up; }

                        if message.message == winapi::um::winuser::WM_RBUTTONDOWN{ mouseinfo.rbutton = ButtonStatus::Down;  }
                        else if message.message == winapi::um::winuser::WM_RBUTTONUP{ mouseinfo.rbutton = ButtonStatus::Up; }
                        //else { mouseinfo.lbutton = ButtonStatus::Up; }//TODO Not sure what this fixed but keep an eye on this

                        //Mouse Wheel stuffs
                        if message.message == winapi::um::winuser::WM_MOUSEWHEEL{
                            let delta_wheel = winapi::um::winuser::GET_WHEEL_DELTA_WPARAM(message.wParam) as i16;
                            mouseinfo.wheel += delta_wheel as isize /120;
                            mouseinfo.wheel_delta = delta_wheel as i32 /120;
                        }
                        else{
                        }

                    }
                    {//Handle text events
                        if message.message == winapi::um::winuser::WM_CHAR{
                            //NOTE
                            //This only handles ascii characters
                            if textinfo.character.len() < 50 { textinfo.character.push(message.wParam as u8 as char); }
                        }else{
                            //NOTE hold over from a breaking change 5/26/2020 
                            //textinfo.character = '\0';
                        }
                    }
                    {//Handle keyboard events
                        keyboardinfo.update_keyboardinfo_windows(&message);
                    }
                    if message.message == winapi::um::winuser::WM_QUIT{
                        break 'a;
                    }
                    else if message.message == winapi::um::winuser::WM_KEYDOWN && message.wParam == winapi::um::winuser::VK_ESCAPE as usize{
                        break 'a;
                    }

                    //NOTE Thoth Gunter 02/11/2020
                    //https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage
                    //https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessage
                    //
                    //Places messages back on queue not sure why we are doing this
                    //Should check out when I have more time.
                    TranslateMessage(&mut message as *mut MSG);
                    DispatchMessageW(&mut message as *mut MSG);
                    n_messages += 1;
                }
                if !mouse_delta_updated  {
                    mouseinfo.delta_x = 0;
                    mouseinfo.delta_y = 0;
                } else{
                    mouseinfo.delta_x = mouseinfo.x - mouseinfo.delta_x;
                    mouseinfo.delta_y = mouseinfo.y - mouseinfo.delta_y;
                }

                render_default_to_buffer(&mut GLOBAL_BACKBUFFER, None);

                match &SETICON {
                    Some(bmp)=>{
                        println!("Attempting to changing app icon... not completed.");
                    },
                    None=>{}
                }
                SETICON = None;
                

                if circuit_sim(&mut OsPackage{window_canvas: &mut GLOBAL_BACKBUFFER, window_info: &mut GLOBAL_WINDOWINFO},
                            &mut ls_app_storage, &keyboardinfo, &textinfo, &mouseinfo) != 0 { break 'a; }

                let delta_time = stopwatch.lap_time();
                let gb_height = GLOBAL_BACKBUFFER.h;
                //rendertools::draw_string(&mut GLOBAL_BACKBUFFER, &format!("{:?}", delta_time), 0, gb_height-27, rendertools::C4_WHITE, 26.0);//TODO
                stopwatch.reset_lap_timer();
                
                if old_window_info != GLOBAL_WINDOWINFO{//TODO resize
                    user32::SetWindowPos(window_handle, user32::HWND_TOP, 0, 0, GLOBAL_WINDOWINFO.w, GLOBAL_WINDOWINFO.h, user32::SWP_NOREDRAW | user32::SWP_NOREPOSITION | user32::SWP_NOZORDER);
                }

                //NOTE
                //Whats the difference between get client rect and get window rect
                //and why does client rect give bad x and y values
                let device_context = user32::GetDC(window_handle);
                let mut rect: RECT = RECT{ left: 0, top: 0, right: 0, bottom: 0};
                user32::GetClientRect(window_handle, &mut rect as *mut RECT);
                update_window(device_context, &GLOBAL_BACKBUFFER, 0, 0, rect.right-rect.left, rect.bottom-rect.top);

                if user32::GetWindowRect(window_handle, &mut rect) != 0{
                    GLOBAL_WINDOWINFO.x = rect.left;
                    GLOBAL_WINDOWINFO.y = rect.top;
                    GLOBAL_WINDOWINFO.w = rect.right - rect.left;
                    GLOBAL_WINDOWINFO.h = rect.bottom - rect.top;
                }
                old_window_info = GLOBAL_WINDOWINFO;

                user32::ReleaseDC(window_handle, device_context);
                frame_counter += 1;
                //TODO sleep?
            }
        } else{

        }
    } else{

    }
}}

extern "system" fn window_callback(window: HWND, message: u32, w_param: usize, l_param: isize )->isize{unsafe{

    use user32::{DefWindowProcA, BeginPaint, EndPaint, PostQuitMessage, GetClientRect};
    use winapi::um::winuser::{WM_SIZE, WM_DESTROY, WM_CLOSE, WM_ACTIVATEAPP, WM_PAINT, PAINTSTRUCT};

    let mut rt = 0;
    match message{
        WM_SIZE=>{
            let mut rect: RECT = RECT{ left: 0, top: 0, right: 0, bottom: 0};
            GetClientRect(window, &mut rect as *mut RECT);
            resize_drawsection(&mut GLOBAL_BACKBUFFER, rect.right - rect.left, rect.bottom - rect.top);
        },
        WM_DESTROY=>{
            PostQuitMessage(0);
        },
        WM_CLOSE=>{
            PostQuitMessage(0);
        },
        WM_ACTIVATEAPP=>{
        },
        WM_PAINT=>{
            let rect: RECT = RECT{ left: 0, top: 0, right: 0, bottom: 0};
            let mut canvas = PAINTSTRUCT{hdc: null_mut(), fErase: 0 , rcPaint: rect, fRestore: 0, fIncUpdate: 0, rgbReserved: [0;32]};
            BeginPaint(window, &mut canvas as *mut PAINTSTRUCT );
            {//TODO
             //will soon become my DrawRect function
                let x = canvas.rcPaint.left;
                let y = canvas.rcPaint.top;
                let w = canvas.rcPaint.right - canvas.rcPaint.left;
                let h = canvas.rcPaint.bottom - canvas.rcPaint.top;
                update_window(canvas.hdc, &GLOBAL_BACKBUFFER, x, y, w, h);
            }
            EndPaint(window, &mut canvas as *mut PAINTSTRUCT);
        },
        _=>{
            rt = DefWindowProcA(window, message, w_param, l_param);
        },
    }
    return rt;
}}



fn resize_drawsection( canvas: &mut WindowCanvas, w: i32, h: i32){unsafe{
    use winapi::um::memoryapi::{VirtualAlloc, VirtualFree};
    use winapi::um::winnt::{MEM_COMMIT, PAGE_READWRITE, MEM_RELEASE};

    if w == 0 
    || h == 0 {
        return;
    }

    if canvas.buffer != null_mut(){
        VirtualFree(canvas.buffer as *mut winapi::ctypes::c_void, 0, MEM_RELEASE);
    }
    canvas.info = TGBitmapHeaderInfo{
          header_size : mem::size_of::<TGBitmapHeaderInfo>() as u32,
          width : w,
          height : h,
          planes : 1,
          bit_per_pixel : 32,
          compression : 0,//BI_RGB,
          image_size: 0,
          x_px_per_meter: 0,
          y_px_per_meter: 0,
          colors_used: 0,
          colors_important: 0,
    };
    canvas.w = w;
    canvas.h = h;

    canvas.buffer = VirtualAlloc(null_mut(), (w*h*32) as usize, MEM_COMMIT, PAGE_READWRITE) as *mut std::ffi::c_void;
    render_default_to_buffer( &mut GLOBAL_BACKBUFFER, None);
}}


fn update_window(device_context: HDC, canvas: &WindowCanvas, _x: i32, _y: i32, w: i32, h: i32 ){unsafe{
    use gdi32::StretchDIBits;
    let _w = canvas.w;
    let _h = canvas.h;

    let info = BITMAPINFO{
        bmiHeader : BITMAPINFOHEADER{
            biSize : mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth : w,
            biHeight : h,
            biPlanes : 1,
            biBitCount : 32,
            biCompression : 0,//BI_RGB,
            biSizeImage : 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: [new_rgbquad()],
    };

    StretchDIBits(device_context, 0, 0, w, h, 0, 0, _w, _h, canvas.buffer as *const winapi::ctypes::c_void, &info as *const BITMAPINFO, 0, SRCCOPY);
}}

pub fn render_default_to_buffer( canvas: &mut WindowCanvas, default_color: Option<[u8;4]>){unsafe{
    let buffer = canvas.buffer as *mut u32;
    let w = canvas.w;
    let h = canvas.h;

    let mut r = 100;
    let mut g = 50;
    let mut b = 50;
    match default_color{
        Some(arr) =>{
            r = arr[0] as u32;
            g = arr[1] as u32;
            b = arr[2] as u32;
        },
        None =>{
        }
    }
    //TODO speedup
    for i in 0..(w*h) as isize {
        *buffer.offset(i) = 0x00000000 + (r << 16) +  (g << 8)  + b;
    }
}}




