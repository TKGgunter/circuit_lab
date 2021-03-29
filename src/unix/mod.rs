//! This module contains the main event loop for the linux operating system.
//!
//! This application, on linux, uses x11 for window management.  This module includes basic
//! input event gathering, keyboard and mouse events.  The primary functions of 
//! interest are `make_window` and `update_screen`. 
//! `make_window` contains the event loop and `update_screen` updates the x11 
//! buffer with the contents of our backbuffer.
//!


#![cfg(target_os = "linux")]
#![allow(warnings, unused)]

use crate::lab_sims::*;
use crate::ui_tools::ui_test;

use crate::x11::xlib;
use x11::xlib::*;/*{XOpenDisplay, XDefaultScreen, XBlackPixel, XWhitePixel, XNextEvent,
                XCreateSimpleWindow, XSetStandardProperties, XSelectInput, XEvent,
                XSetBackground, XSetForeground, XClearWindow, XInternAtom, XMapRaised, 
                XSetWMProtocols, XFreeGC, XDestroyWindow, XCloseDisplay, XDestroyImage,
                XDefaultRootWindow, ExposureMask, ButtonPressMask, KeyPressMask, XPending,
                XCreateGC, XCreatePixmapFromBitmapData, XCopyPlane, XFlush, XSync,
                XCreateImage, XDefaultVisual, ZPixmap, XDefaultDepth, XPutImage, XImage};*/

use std::ptr::{null, null_mut};
use std::time::{Duration, Instant};
use std::thread::sleep;

use crate::rendertools::*;
use crate::{WindowCanvas, WindowInfo,
            GLOBAL_BACKBUFFER, OsPackage,
            GLOBAL_WINDOWINFO, inputhandler, SETICON};

use inputhandler::*;
use crate::misc::*;

#[macro_use]
use crate::{timeit, DEBUG_timeit};
use crate::debug_tools::*;



/// This function creates a new x11 image and draws that image in the given display window.
/// The resulting x11 image is then destroyed.
fn update_screen(buffer: &mut [u8], dis: *mut xlib::Display, visual: *mut xlib::Visual, win: u64, depth: u32, gc: xlib::GC, window_width: u32, window_height: u32){unsafe{


    let image = XCreateImage(dis, visual, depth, ZPixmap, 0, buffer.as_mut_ptr() as *mut _, window_width, window_height, 32, 0); 
    let mut _image = (image as *mut XImage).as_mut().unwrap();

    XPutImage(dis, win, gc, image, 0, 0, 0, 0, window_width, window_height);

    XSync(dis, 0);
    _image.data = null_mut();
    XDestroyImage(image);
}}

/// This function is supposed to set the icon for the application. It currently does not work.
///
fn _set_icon( dis: *mut x11::xlib::_XDisplay, win: x11::xlib::Window, bmp: &TGBitmap ){unsafe{
    use std::ffi::CString;

    let net_wm_icon = x11::xlib::XInternAtom(dis, CString::new("_NET_WM_ICON").expect("net_wm_icon").into_raw(), x11::xlib::False);
    let cardinal    = x11::xlib::XInternAtom(dis, CString::new("CARDINAL").expect("cardinal").into_raw(), x11::xlib::False);

    let width  =  bmp.width;
    let height =  bmp.height;
    let mut _buffer = Vec::with_capacity((2+width*height) as usize);

    _buffer.push( width );
    _buffer.push( height );

    let buffer = bmp.rgba.as_ptr();

    for i in (0..height).rev(){
        for j in 0..width{

            let a = *buffer.offset((4*(i * height + j) + 3) as isize) as u32;
            let r = *buffer.offset((4*(i * height + j) + 2) as isize) as u32;
            let g = *buffer.offset((4*(i * height + j) + 1) as isize) as u32;
            let b = *buffer.offset((4*(i * height + j) + 0) as isize) as u32;

            //uint8_t a = ((uint8_t*)&pixel)[0];
            //uint8_t r = ((uint8_t*)&pixel)[1];
            //uint8_t g = ((uint8_t*)&pixel)[2];
            //uint8_t b = ((uint8_t*)&pixel)[3];

            //((uint8_t*)&_pixel)[0] = r;
            //((uint8_t*)&_pixel)[1] = g;
            //((uint8_t*)&_pixel)[2] = b;
            //((uint8_t*)&_pixel)[3] = a;

            let _pixel : i32 = std::mem::transmute(0x00000000 + (a << 24) + (b << 16) + (g << 8) + r);
            _buffer.push(_pixel);
        }
    }
    let length = 2 + width * height;

    let _cp = x11::xlib::XChangeProperty(dis, win, net_wm_icon, cardinal, 32,  x11::xlib::PropModeReplace, _buffer.as_ptr() as *mut u8, length);
    let _mw = x11::xlib::XMapWindow(dis, win);
}}




/// This is the primary function for the application. The event/render loop occurs here.
///
pub fn make_window() {unsafe{

    //NOTE
    //Standard x11 window initialization occurs here.
    let window_width  = 1000;
    let window_height = 550;

    let dis    = XOpenDisplay(null());
    let screen = XDefaultScreen(dis);
    let black  = XBlackPixel(dis, screen);
    let white  = XWhitePixel(dis, screen);

    let win = XCreateSimpleWindow(dis, XDefaultRootWindow(dis), 0, 0,
                                  window_width, window_height, 5, black,
                                  black);


    use std::ffi::CString; 
    XSetStandardProperties(dis,win,CString::new("CircuitLab").unwrap().into_raw(),
                           CString::new("Temp v01").unwrap().into_raw(),
                           0,null_mut(),0,null_mut());
    XSelectInput(dis, win, ExposureMask|ButtonPressMask|KeyPressMask|KeyReleaseMask);
    let gc=XCreateGC(dis, win, 0, null_mut());        



    XSetBackground(dis, gc, 0);
    XSetForeground(dis, gc, white);
    XClearWindow(dis, win);
    XMapRaised(dis, win);

    let mut wm_delete_window = XInternAtom(dis, CString::new("WM_DELETE_WINDOW").unwrap().into_raw(),
                                           0/*False*/);
    XSetWMProtocols(dis, win, &mut wm_delete_window as *mut _, 1);



    let mut presentation_buffer = vec![0u8; (4*window_width*window_height) as usize];
    let mut bmp_buffer = vec![0u8; (4*window_width*window_height) as usize];


    let visual = XDefaultVisual(dis, 0);
    let depth = XDefaultDepth(dis, screen) as u32;

    XFlush(dis);


    unsafe{
        //NOTE
        //Setting up GLOBAL_BACKBUFFER dimensions and dpi properties.

        GLOBAL_BACKBUFFER.info.width = window_width as i32;
        GLOBAL_BACKBUFFER.info.height = window_height as i32;
        GLOBAL_BACKBUFFER.info.planes = 1;
        
        GLOBAL_BACKBUFFER.w = window_width as i32;
        GLOBAL_BACKBUFFER.h = window_height as i32;
        GLOBAL_BACKBUFFER.buffer = bmp_buffer.as_mut_ptr() as *mut _;

        GLOBAL_WINDOWINFO.w = GLOBAL_BACKBUFFER.w;
        GLOBAL_WINDOWINFO.h = GLOBAL_BACKBUFFER.h;

        GLOBAL_BACKBUFFER.display_width    =  XDisplayWidth(dis, 0); 
        GLOBAL_BACKBUFFER.display_width_mm = XDisplayWidthMM(dis, 0);

        GLOBAL_BACKBUFFER.display_height = XDisplayHeight(dis, 0); 
        GLOBAL_BACKBUFFER.display_height_mm = XDisplayHeightMM(dis, 0);

        {
            let x_mm = GLOBAL_BACKBUFFER.display_width_mm as f32;
            let x = GLOBAL_BACKBUFFER.display_width as f32;

            let y_mm = GLOBAL_BACKBUFFER.display_height_mm as f32;
            let y = GLOBAL_BACKBUFFER.display_height as f32;

            if x >= 1f32 && y >= 1f32 { 
                GLOBAL_BACKBUFFER.dpmm = (x.powi(2) + y.powi(2)).sqrt() / (x_mm.powi(2) + y_mm.powi(2)).sqrt();
            } else {
                GLOBAL_BACKBUFFER.dpmm = DPMM_SCALE; 
            }
        }
    }




    let mut mouseinfo   = MouseInfo::new();
    let mut textinfo = TextInfo{character: Vec::with_capacity(10), timing:Vec::new()};
    let mut keyboardinfo = KeyboardInfo{key: Vec::new(), status:Vec::new()};

    let mut ls_app_storage = LS_AppStorage::new();

    let mut stopwatch = StopWatch::new();
    let mut stopwatch_lbutton = StopWatch::new();
    let mut old_window_info = GLOBAL_WINDOWINFO;

    let mut exe_path = std::env::current_exe().expect("could not find the exe path");
    let in_target_path = exe_path.to_string_lossy().contains("target/release");


     
    init_debugging( Some([0, 0, 600, 500]) );


    let mut exit = false;
    loop{

        match &SETICON {
            Some(bmp)=>{
                _set_icon(dis, win, bmp);
            },
            None=>{}
        }
        SETICON = None;

        {
            let max_len = (GLOBAL_BACKBUFFER.h * GLOBAL_BACKBUFFER.w * 4) as usize;
            let count = (GLOBAL_BACKBUFFER.w * 4) as usize;
            for i in 0..GLOBAL_BACKBUFFER.h as usize {unsafe{

                let mut ptr_bmp = bmp_buffer.as_mut_ptr().offset( (i*count) as isize );
                let mut ptr_pre = presentation_buffer.as_mut_ptr().offset( (max_len - (i+1)*count ) as isize);

                std::ptr::copy_nonoverlapping(ptr_bmp, ptr_pre, count);

            }}
        }
        
        update_screen(&mut presentation_buffer[..], dis, visual, win, depth, gc, GLOBAL_BACKBUFFER.w as _, GLOBAL_BACKBUFFER.h as _);

        {//TODO change window size if application asks
            if GLOBAL_WINDOWINFO.w != old_window_info.w
            || GLOBAL_WINDOWINFO.h != old_window_info.h{

                XResizeWindow(dis, win, GLOBAL_WINDOWINFO.w as _, GLOBAL_WINDOWINFO.h as _);
            }
        }

	let mut window_struct = XWindowAttributes{
	    x: 0,
	    y: 0,
	    width:  0,
	    height: 0,
	    border_width: 0,
	    depth: 0,
	    visual: null_mut(),
	    root: 0,
	    class: 0,
	    bit_gravity: 0,
	    win_gravity: 0,
	    backing_store: 0,
	    backing_planes: 0,
	    backing_pixel: 0,
	    save_under: 0,
	    colormap: 0,
	    map_installed: 0,
	    map_state: 0,
	    all_event_masks: 0,
	    your_event_mask: 0,
	    do_not_propagate_mask: 0,
	    override_redirect: 0,
	    screen: null_mut(),
	};
	XGetWindowAttributes(dis, win, &mut window_struct as *mut _);
        GLOBAL_WINDOWINFO.w = window_struct.width;	
        GLOBAL_WINDOWINFO.h = window_struct.height;	

        old_window_info = GLOBAL_WINDOWINFO;

	if window_struct.width != GLOBAL_BACKBUFFER.w 
	|| window_struct.height != GLOBAL_BACKBUFFER.h{

            GLOBAL_BACKBUFFER.w = window_struct.width;	
            GLOBAL_BACKBUFFER.h = window_struct.height;	
            GLOBAL_BACKBUFFER.info.width = GLOBAL_BACKBUFFER.w;
            GLOBAL_BACKBUFFER.info.height = GLOBAL_BACKBUFFER.h;
            
            let size = (4 * GLOBAL_BACKBUFFER.w * GLOBAL_BACKBUFFER.h) as usize;

            bmp_buffer.resize(size, 0);
            presentation_buffer.resize(size, 0);

            GLOBAL_BACKBUFFER.buffer = bmp_buffer.as_mut_ptr() as *mut _;
	}



        keyboardinfo.key.clear();
        keyboardinfo.status.clear();

        textinfo.character.clear();
        textinfo.timing.clear();


        mouseinfo.old_lbutton = mouseinfo.lbutton;
        mouseinfo.old_rbutton = mouseinfo.rbutton;
        mouseinfo.lbutton = ButtonStatus::Up;
        mouseinfo.rbutton = ButtonStatus::Up;
        mouseinfo.double_lbutton = false;

        mouseinfo.delta_x = mouseinfo.x;
        mouseinfo.delta_y = mouseinfo.y;
        let mut x  : i32 = 0;
        let mut y  : i32 = 0;
        let mut _x : i32 = 0;
        let mut _y : i32 = 0;
        {
            let mut mask = 0u32;

            let mut _w0 : Window = 0;
            let mut _w1 : Window = 0;

            XQueryPointer(dis, win, &mut _w0 as *mut _, &mut _w1 as *mut _, 
                          &mut x as *mut _, &mut y as *mut _, &mut _x as *mut _, 
                          &mut _y as *mut _, &mut mask as *mut _);

            if mask&256 == 256{//Left click TODO check with mouse
                mouseinfo.lbutton = ButtonStatus::Down;
            } else if mask&1024 == 1024 {//Right click TODO check with mouse
                mouseinfo.rbutton = ButtonStatus::Down;
            }

            if mouseinfo.lbutton == ButtonStatus::Up
            && mouseinfo.old_lbutton == ButtonStatus::Down{
                stopwatch_lbutton.reset_lap_timer();
            }
            if mouseinfo.lbutton == ButtonStatus::Down
            && mouseinfo.old_lbutton == ButtonStatus::Up
            && stopwatch_lbutton.lap_time().as_millis() <= 450 {
                mouseinfo.double_lbutton = true;
            }


            mouseinfo.x = _x;
            mouseinfo.y = GLOBAL_BACKBUFFER.h as i32 - _y;
        }
        
        
        let mut text_key : KeySym = 0;
        let mut text = [0u8; 4];

        let mut event = XEvent{type_: 0};
        while XPending(dis) !=0 {
            XNextEvent(dis, &mut event as *mut _);
            match event.type_ {
                KeyPress=>{
                    if XLookupString(&mut event.key as *mut _, 
                                    text.as_mut_ptr() as *mut _, 4, 
                                    &mut text_key as *mut _, null_mut()) == 1{
                        if text[0] == 27
                        && in_target_path {//ESC text code 
                            exit = true;
                        }
                        
                        let temp_str =  std::str::from_utf8(&text).expect("Good string");
                        for (i_ts, it_ts) in temp_str.chars().enumerate(){
                            if i_ts > 0 && it_ts == '\0' {}
                            else { 
                                textinfo.character.push(it_ts);
                            }
                        }
                    }
                    let _key = XLookupKeysym(&mut event.key as *mut _, 0);
                    keyboardinfo.update_keyboardinfo_linux(_key, true);
                },
                KeyRelease=>{
                    let _key = XLookupKeysym(&mut event.key as *mut _, 0);
                    let mut peek_event = XEvent{type_: 0};
                    if XEventsQueued(dis, 1) > 0 {
                    XPeekEvent(dis, &mut peek_event as *mut _);
                        if peek_event.type_ == KeyPress 
                        && XLookupKeysym(&mut peek_event.key as *mut _, 0) == _key{}
                        else {
                            keyboardinfo.update_keyboardinfo_linux(_key, false);//TODO status should not be a bool it is not enough information
                        }
                    } else {
                        keyboardinfo.update_keyboardinfo_linux(_key, false);//TODO status should not be a bool it is not enough information
                    }
                },
                ClientMessage=>{
                    if event.client_message.data.get_long(0) == wm_delete_window as i64{
                        exit = true;
                    }
                },
                _=>{
                }
            }
        }

        mouseinfo.delta_x = mouseinfo.x - mouseinfo.delta_x;
        mouseinfo.delta_y = mouseinfo.y - mouseinfo.delta_y;

        if circuit_sim(&mut OsPackage{window_canvas: &mut GLOBAL_BACKBUFFER, window_info: &mut GLOBAL_WINDOWINFO},
                    &mut ls_app_storage, &keyboardinfo, &textinfo, &mouseinfo) != 0 { break; }

        let delta_time = stopwatch.lap_time();
        draw_string(&mut GLOBAL_BACKBUFFER, &format!("{:#.3?}", delta_time), 0, GLOBAL_BACKBUFFER.h-30, C4_WHITE, 26.0);//TODO we should avg things so we no flicker
        stopwatch.reset_lap_timer();

        draw_debuginfo(&mut GLOBAL_BACKBUFFER);
        reset_frame_debugging();
        
        if exit {
            break;
        }
    }

    XFreeGC(dis, gc);
    XDestroyWindow(dis, win);
    XCloseDisplay(dis);	
}}
