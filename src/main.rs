#![allow(warnings)]  //TODO

extern crate stb_tt_sys;
extern crate stb_image_sys;
extern crate miniz;


use std::ptr::{null, null_mut};
use std::iter::once;
use std::mem;


mod inputhandler;
use inputhandler::*;

mod ui_tools;



mod lab_sims;
use lab_sims::*;


mod rendertools;
use rendertools::{TGBitmapHeaderInfo, TGBitmap };


mod os_util;

mod debug_tools;

mod misc;
use misc::*;


#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows::*;


#[cfg(target_os = "macos")]
#[macro_use] 
extern crate cocoa;
#[cfg(target_os = "macos")]
#[macro_use] 
extern crate objc;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos::*;

#[cfg(target_os = "linux")]
extern crate x11;
#[cfg(target_os = "linux")]
mod unix;
#[cfg(target_os = "linux")]
use unix::make_window;

mod eq_potential;



const FONT_NOTOSANS : &[u8] = std::include_bytes!("../assets/NotoSans-Regular.ttf");//TODO better pathing maybe
const FONT_NOTOSANS_BOLD : &[u8] = std::include_bytes!("../assets/NotoSans-Bold.ttf");//TODO better pathing maybe



#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct WindowInfo{
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

pub struct WindowCanvas{
    pub info : TGBitmapHeaderInfo,
    pub w: i32,
    pub h: i32,
    pub buffer: *mut std::ffi::c_void
}

static mut GLOBAL_WINDOWINFO : WindowInfo = WindowInfo{ x: 0, y: 0, w: 0, h: 0};
static mut GLOBAL_BACKBUFFER : WindowCanvas = WindowCanvas{
    info : TGBitmapHeaderInfo{
          header_size : 0,
          width : 0,
          height : 0,
          planes : 0,
          bit_per_pixel : 0,
          compression : 0,//BI_RGB,
          image_size: 0,
          x_px_per_meter: 0,
          y_px_per_meter: 0,
          colors_used: 0,
          colors_important: 0,
      },
    w : 0,
    h : 0,
    buffer : null_mut(),
};

pub struct OsPackage{
    pub window_canvas: &'static mut WindowCanvas, 
    pub window_info:  &'static mut WindowInfo,
}


pub static mut SETICON : Option<TGBitmap> = None;
pub fn set_icon( _bmp: &TGBitmap){unsafe{
    let mut v = vec![0; _bmp.rgba.len()];
    v.copy_from_slice(&_bmp.rgba);

    let bmp = TGBitmap{
        file_header: _bmp.file_header,
        info_header: _bmp.info_header,
        height: _bmp.height,
        width: _bmp.width,
        rgba: v,
    };
    SETICON = Some( bmp );   
}}


#[cfg(target_os = "windows")]
mod udp{
    use crate::dynamic_lib_loading::*;

    static mut UDP_LIB : Option<DyLib> = None;
    static mut GET_UDP_STATS : Option<fn (*mut _MIB_UDPSTATS)->u32> = None;

    #[derive(Debug, Default)] #[allow(non_camel_case_types)]
    pub struct _MIB_UDPSTATS {
        //NOTE
        //InDatagrams and OutDatagrams
        //are a cumulative accounting not based on when GetUdpStatistics is last called
        pub dwInDatagrams : u32,
        pub dwNoPorts     : u32,
        pub dwInErrors    : u32,
        pub dwOutDatagrams: u32,
        pub dwNumAddrs    : u32,
    }

    pub fn load_udplib()->Option<()>{unsafe{
        let lib = open_lib( "Iphlpapi.dll", 0 );
        match lib {
            Ok(_lib)=>{
                UDP_LIB = Some(_lib);
                let get_udp_stats = get_fn( UDP_LIB.as_ref().unwrap(), "GetUdpStatistics" );
                GET_UDP_STATS = std::mem::transmute(get_udp_stats.unwrap());
                return Some(());
            },
            Err(_)=>{
                return None;
            }
        }

        return Some(());
    }}

    pub fn get_udp_stats(stats: &mut _MIB_UDPSTATS)->Result<u32, String>{unsafe{
        match GET_UDP_STATS{
            None=>{ return Err("GET_UDP_STATS not set up".to_string()); }
            Some(_fn)=>{ return Ok(_fn( stats as *mut _)); }
        }
    }}

}


fn main() {

    make_window();
    //pause();
}






//TODO
//move some where else
#[cfg(target_os = "windows")]
pub mod dynamic_lib_loading{
use std::os::raw::{c_int, c_void};

    extern "C" {
        fn LoadLibraryA( path: *const i8 ) -> *mut c_void;
        fn GetProcAddress( lib: *mut c_void, name: *const i8 ) -> *mut c_void;
        fn FreeLibrary( lib: *mut c_void ) -> c_int;
        fn GetLastError() -> u32;
    }

    //TODO
    //This is temporary should be replaced by windows enums
    pub const RTLD_LAZY         : i32 =   0x00001;        /* Lazy function call binding.  */

    pub struct DyLib(*mut c_void);

    pub fn open_lib( lib_path: &str, _flag: i32 )->Result<DyLib, String>{unsafe{
        let _path = lib_path.to_string() + "\0";
        let lib = LoadLibraryA( _path.as_ptr() as *const i8);
        if lib.is_null(){
            let s = format!("Could not open lib \n{:?}\n\n For more info => https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes", GetLastError());
            return Err(s);
        }

        Ok(DyLib(lib as *mut c_void))
    }}

    pub fn get_fn( shared_lib_handle: &DyLib, name: &str)-> Result<*mut (), String>{ unsafe{
        let fn_name = name.to_string() + "\0";
        let function = GetProcAddress(shared_lib_handle.0 as _, fn_name.as_ptr() as *const i8) as *mut ();
        if function.is_null(){
            let s = format!("Could not get function \n{:?}", GetLastError());
            return Err(s);
        }

        Ok(function)
    }}

    pub fn get_error()->String{
        "Windows version has not been implemented".to_string()
    }

    pub fn close_lib(shared_lib_handle: &DyLib){unsafe{
        if FreeLibrary(shared_lib_handle.0 as _) == 0{
            println!("Could not properly close shared library.");
            println!("{}", format!("{:?}", GetLastError()));
        }
    }}
}











