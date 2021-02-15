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


#[cfg(target_os = "linux")]
pub mod dynamic_lib_loading{
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::ffi::CString; 
use std::ptr;
use std::os::raw::{c_int, c_char, c_void};

//
//This is a lib of dlopen and dlclose using rust
//Comments copied from the following source files
// /usr/include/dlfcn.h
// https://www.unvanquished.net/~modi/code/include/x86_64-linux-gnu/bits/dlfcn.h.html

    /* These are the possible values for the REQUEST argument to `dlinfo'.  */
    enum DL_INFO{
        /* Treat ARG as `lmid_t *'; store namespace ID for HANDLE there.  */
        RTLD_DI_LMID = 1,

        /* Treat ARG as `struct link_map **';
           store the `struct link_map *' for HANDLE there.  */
        RTLD_DI_LINKMAP = 2,

        RTLD_DI_CONFIGADDR = 3,	/* Unsupported, defined by Solaris.  */

        /* Treat ARG as `Dl_serinfo *' (see below), and fill in to describe the
           directories that will be searched for dependencies of this object.
           RTLD_DI_SERINFOSIZE fills in just the `dls_cnt' and `dls_size'
           entries to indicate the size of the buffer that must be passed to
           RTLD_DI_SERINFO to fill in the full information.  */
        RTLD_DI_SERINFO = 4,
        RTLD_DI_SERINFOSIZE = 5,

        /* Treat ARG as `char *', and store there the directory name used to
           expand $ORIGIN in this shared object's dependency file names.  */
        RTLD_DI_ORIGIN = 6,

        RTLD_DI_PROFILENAME = 7,	/* Unsupported, defined by Solaris.  */
        RTLD_DI_PROFILEOUT = 8,	/* Unsupported, defined by Solaris.  */

        /* Treat ARG as `size_t *', and store there the TLS module ID
           of this object's PT_TLS segment, as used in TLS relocations;
           store zero if this object does not define a PT_TLS segment.  */
        RTLD_DI_TLS_MODID = 9,

        /* Treat ARG as `void **', and store there a pointer to the calling
           thread's TLS block corresponding to this object's PT_TLS segment.
           Store a null pointer if this object does not define a PT_TLS
           segment, or if the calling thread has not allocated a block for it.  */
        RTLD_DI_TLS_DATA = 10,

        //RTLD_DI_MAX = 10
    }

/* The MODE argument to `dlopen' contains one of the following: */
    
    pub const RTLD_LAZY         : i32 =   0x00001;        /* Lazy function call binding.  */
    pub const RTLD_NOW          : i32 =   0x00002;        /* Immediate function call binding.  */
    pub const RTLD_BINDING_MASK : i32 =   0x3    ;    /* Mask of binding time value.  */
    pub const RTLD_NOLOAD       : i32 =   0x00004;        /* Do not load the object.  */
    pub const RTLD_DEEPBIND     : i32 =   0x00008;        /* Use deep binding.  */
    /* If the following bit is set in the MODE argument to `dlopen',
     *    the symbols of the loaded object and its dependencies are made
     *       visible as if the object were linked directly into the program.  */
    pub const RTLD_GLOBAL       : i32 =  0x00100;
    /* Unix98 demands the following flag which is the inverse to RTLD_GLOBAL.
     *    The implementation does this by default and so we can define the
     *       value to zero.  */
    pub const RTLD_LOCAL       : i32 = 0;
    /* Do not delete object when closed.  */
    pub const RTLD_NODELETE    : i32 = 0x01000;

    struct Dl_info{
      dli_fname: *mut c_char,	/* File name of defining object.  */
      dli_fbase: *mut c_void,	/* Load address of that object.  */
      dli_sname: *mut c_char,	/* Name of nearest symbol.  */
      dli_saddr: *mut c_void,	/* Exact value of nearest symbol.  */
      //dlerror
    }
    /* This is the type of elements in `Dl_serinfo', below.
       The `dls_name' member points to space in the buffer passed to `dlinfo'.  */
    struct Dl_serpath
    {
      dls_name: *mut c_char,		/* Name of library search path directory.  */
      dls_flags: u32,	/* Indicates where this directory came from. */
    }

    /* This is the structure that must be passed (by reference) to `dlinfo' for
       the RTLD_DI_SERINFO and RTLD_DI_SERINFOSIZE requests.  */
    struct Dl_serinfo
    {
      dls_size: usize,		/* Size in bytes of the whole buffer.  */
      dls_cnt: u32,		/* Number of elements in `dls_serpath'.  */
      dls_serpath: [Dl_serpath;1],	/* Actually longer, dls_cnt elements.  */
    } 

    //TODO
    //Think about changing from c_int to i32 or something
    extern "C" {
        pub fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
        pub fn dlsym(lib_handle: *mut c_void, name: *const c_char) -> *mut c_void;
        pub fn dlclose(lib_handle: *mut c_void) -> c_int;
        pub fn dlinfo(lib_handle: *mut c_void, request: c_int, info: *mut c_void) -> c_int;
        pub fn dlerror() -> *mut c_char;
    }

    pub struct DyLib(*mut c_void);

    pub fn open_lib( lib_path: &str, flag: i32 )->Result<DyLib, String>{unsafe{

        //TODO
        //Get enums dlopen uses
        let shared_lib_handle = dlopen(CString::new(lib_path).unwrap().as_ptr(), flag as c_int);
        if shared_lib_handle.is_null(){
            println!("{:?}", get_error());
            Err(format!("Shared lib is null! {}  Check file path/name.", lib_path))
        }
        else{
            Ok( DyLib(shared_lib_handle) )
        }

    }}

    //Example
    //let function : fn()->i32= transmute_copy((dlsym(shared_lib_handle, CString::new(name).unwrap().as_ptr()) as *mut ()).as_mut());
    pub fn get_fn( shared_lib_handle: &DyLib, name: &str)-> Result<*mut (), String>{ unsafe{
        let _fn = dlsym(shared_lib_handle.0, CString::new(name).unwrap().as_ptr());
        if _fn.is_null() {
           Err("Function name could not be found.".to_string()) 
        }
        else{
            Ok(_fn as *mut () )
        }
    }}

    pub fn get_error()->String{unsafe{
        let error = dlerror();
        if error.is_null(){
            return "No Error".to_string();
        }
        else{
            CString::from_raw(error).into_string().unwrap()
        }
    }}

    pub fn close_lib(shared_lib_handle: &DyLib){unsafe{
        if dlclose(shared_lib_handle.0) != 0{
            println!("Could not properly close shared library.");
        }
    }}
}










