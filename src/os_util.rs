#![cfg(target_os = "windows")]

///////////////////////////
//TODO
//Move out of here
//NOTE
//I wish this didn't have to be a global static but the other way didn't seem to work
use winapi::shared::windef::HWND;
static mut NAMES: Option<Vec::<[char; 128]>> = None;

use winapi::um::winuser::{GetWindowTextA, EnumWindows, GetForegroundWindow};
unsafe extern "system" fn list_windows_callback( window_handle: HWND, params: isize )->i32{

    let mut arr = ['\0'; 128];
    let mut _arr = [0i8; 128];
    GetWindowTextA( window_handle, _arr.as_mut_ptr(), 128);

    for i in 0.._arr.len(){
        arr[i] = _arr[i] as u8 as char;
    }
    if arr[0] == '\0' {
        return 1;
    }
    let names = NAMES.as_mut().unwrap();
    for i in 0..names.len(){
        for j in 0..arr.len(){
            if arr[j] != names[i][j]{
                break;
            }
            return 1;
        }
    }

    names.push( arr );
    return 1;
}

pub fn list_windows(){unsafe{
    NAMES = Some(Vec::new());
    EnumWindows( Some(list_windows_callback), 0);
    let names = NAMES.as_mut().unwrap();
    println!("TODO main.rs list_windows");
    //for i in 0..names.len(){
    //    println!("{:?}", &names[i][..20]);
    //}
}}


pub fn get_foreground_window_name()->String{unsafe{
    let hwnd = GetForegroundWindow(); 
    let mut _arr = [0i8; 128];
    let len = GetWindowTextA( hwnd, _arr.as_mut_ptr(), 128);

    let mut string = String::new();
    for i in 0..len as usize{
        if i >= _arr.len() { break; } 
        string.push(_arr[i] as u8 as char);
    }

    return string;
}}



pub mod screen_capture{
use crate::rendertools::*;

use winapi::shared::windef::{RECT, HDC, HWND__, HDC__};
use std::ptr::{null_mut};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt; //For encode wide
use winapi::um::wingdi::{BITMAP, BITMAPINFO, BITMAPINFOHEADER, SRCCOPY, RGBQUAD};
use winapi::um::wingdi as gdi32;
use gdi32::CreateCompatibleDC;
use winapi::um::winuser as user32;
use std::mem;


    fn new_rgbquad()->RGBQUAD{
         RGBQUAD{
            rgbBlue: 0,
            rgbGreen: 0,
            rgbRed: 0,
            rgbReserved: 0,
         }
    }


    pub struct WindowHandleDC{
        pub window_handle : *mut HWND__,
        pub window_dc     : *mut HDC__,
    }
    pub fn load_handle_dc(window_name: &str, )->WindowHandleDC{ unsafe{
        use std::iter::once;
        use user32::{FindWindowW, GetWindowDC};

        let windows_string: Vec<u16> = OsStr::new(window_name).encode_wide().chain(once(0)).collect();

        let handle = FindWindowW(null_mut(), windows_string.as_ptr());
        let handle_dc = WindowHandleDC{ window_handle: handle,
                        window_dc: GetWindowDC(handle)};

        return handle_dc;
    }}


    pub fn screen_shot(handle_dc: &WindowHandleDC)->Option<TGBitmap>{unsafe{
        use gdi32::{CreateCompatibleBitmap, SelectObject, BitBlt, GetObjectW, GetDIBits};

        let mut rt = None;

        let mut rect: RECT = RECT{ left: 0, top: 0, right: 0, bottom: 0};
        if user32::GetWindowRect(handle_dc.window_handle, &mut rect) != 0{
        } else {
            println!("Coud not get window rect");
            return rt;
        }

        let w = rect.right - rect.left;
        let h = rect.bottom - rect.top;

        //TODO
        //remove for CreateDIBSection should make things faster
        //https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-createdibsection
        //https://www.experts-exchange.com/questions/26484815/Screen-capture-CreateDIBSection-doesn't-capture-LayeredWindow.html
        let bitmap_handle = CreateCompatibleBitmap( handle_dc.window_dc, w, h);

        if bitmap_handle == null_mut(){
            //TODO
            //after about 1-2min this breaks. We might not need to create so many compatiblebitmaps or handles?
            //we have some leak going on with windows 
            println!("bitmap was bad.");
            panic!();
            return rt;
        }

        let compat_dc = CreateCompatibleDC(handle_dc.window_dc);
        if compat_dc == null_mut(){
            let error = winapi::um::errhandlingapi::GetLastError();
            panic!("Could not create compatdc: {}", error);
        }

        {


            let oldBitmap = SelectObject(compat_dc, bitmap_handle as winapi::shared::windef::HGDIOBJ);
            if BitBlt(compat_dc as HDC, 0, 0, w, h, handle_dc.window_dc as HDC, 0, 0, SRCCOPY) == 0 {
                println!("BitBlt broke {:?}", line!());
            }

            //https://stackoverflow.com/questions/3291167/how-can-i-take-a-screenshot-in-a-windows-application
            //https://msdn.microsoft.com/en-us/library/windows/desktop/dd183402(v=vs.85).aspx
            //https://stackoverflow.com/questions/31302185/rust-ffi-casting-to-void-pointer
            let mut pixels = vec![0u8; (4*w*h) as usize];
            let mut bitmap = BITMAP{bmType: 0, bmWidth: 0, bmHeight: 0, bmWidthBytes: 0, bmPlanes: 0, bmBitsPixel: 0, bmBits: &mut pixels[0] as *mut u8 as *mut winapi::ctypes::c_void};

            GetObjectW(bitmap_handle as *mut winapi::ctypes::c_void, mem::size_of::<BITMAP>() as i32 , &mut bitmap as *mut BITMAP as *mut winapi::ctypes::c_void);

            let mut bitmap_info = BITMAPINFO{
                bmiHeader : BITMAPINFOHEADER{
                    biSize : mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth : bitmap.bmWidth,
                    biHeight : bitmap.bmHeight,
                    biPlanes : 1,
                    biBitCount : bitmap.bmBitsPixel,
                    biCompression : 0,//BI_RGB,
                    biSizeImage : ((w as u32 * bitmap.bmBitsPixel as u32 + 31) / 32) * 4 * h as u32,
                    biXPelsPerMeter: 1,
                    biYPelsPerMeter: 1,
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                bmiColors: [new_rgbquad()],
            };

            GetDIBits(handle_dc.window_dc, bitmap_handle, 0, bitmap.bmHeight as u32, &mut pixels[0] as *mut u8 as *mut winapi::ctypes::c_void, &mut bitmap_info as *mut BITMAPINFO, 0);
            SelectObject(compat_dc, oldBitmap);



            let header =  TGBitmapFileHeader{   type_: 0x4d42, //BM
                                                    size_:(mem::size_of::<TGBitmapFileHeader>() + mem::size_of::<TGBitmapHeaderInfo>() + 4 * pixels.len()) as u32,
                                                    reserved_1: 0,
                                                    reserved_2: 0,
                                                    off_bits: (mem::size_of::<TGBitmapFileHeader>() + mem::size_of::<TGBitmapHeaderInfo>()) as u32};

            //Redunant please fix
            let info = TGBitmapHeaderInfo{
                header_size:        mem::size_of::<TGBitmapHeaderInfo>() as u32,
                width:              bitmap.bmWidth,
                height:             bitmap.bmHeight,
                planes:             1,
                bit_per_pixel:      bitmap.bmBitsPixel,
                compression:        0,
                image_size:         bitmap_info.bmiHeader.biSizeImage,
                x_px_per_meter:     1,
                y_px_per_meter:     1,
                colors_used:        0,
                colors_important:   0,
            };

            rt = Some(TGBitmap{file_header: header, info_header: info, rgba: pixels, width: bitmap.bmWidth, height: bitmap.bmHeight});

        }

        let clean_delete = gdi32::DeleteDC(compat_dc as HDC);
        if clean_delete == 0 {
            panic!("Could not release memory: compat_dc");
        }
        let clean_delete = gdi32::DeleteObject(bitmap_handle as _);
        if clean_delete == 0 {
            panic!("Could not release memory: bitmap_handle");
        }
        return rt;
    }}


    pub fn found_window(name: &str)->bool{unsafe{
        use std::iter::once;
        use user32::FindWindowW;
    
        let mut rt = true;
        let windows_string: Vec<u16> = OsStr::new(name).encode_wide().chain(once(0)).collect();
        let window_hwnd = FindWindowW(null_mut(), windows_string.as_ptr());
    
        if window_hwnd == null_mut() {
            rt = false;
        }
        gdi32::DeleteDC(window_hwnd as HDC);
    
        return rt;
    }}

}

