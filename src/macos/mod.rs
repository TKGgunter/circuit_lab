#![cfg(target_os = "macos")]
#![allow(warnings, unused)]

use crate::lab_sims::*;

use crate::cocoa;
use crate::objc;
 
use objc::runtime::{Object, Sel};

use cocoa::base::{selector, nil, NO, YES, id};
use cocoa::quartzcore::CALayer;
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSAutoreleasePool, NSProcessInfo,
                        NSString, NSDefaultRunLoopMode, NSData};
use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular, NSWindow, NSEventMask,
                    NSBackingStoreBuffered, NSMenu, NSMenuItem, NSWindowStyleMask, NSColor, NSView, NSEvent,
                    NSRunningApplication, NSApplicationActivateIgnoringOtherApps, NSImage, NSEventType};


use std::{thread, time};
use std::ptr::{null, null_mut};
use std::fs::File;
use std::io::prelude::*;


use crate::rendertools::*;
use crate::{WindowCanvas, WindowInfo,
            GLOBAL_BACKBUFFER, OsPackage,
            GLOBAL_WINDOWINFO, inputhandler};
use inputhandler::*;
use crate::misc::*;

/////////////
//TODO
// + do the many little things
//  + handle minize 
//  + handle resize
//  + handle textinput
//  + handle keyboard input


struct MacBackbuffer{
    backbuffer : *mut Object,
    image : *mut Object,
}

static mut BACKBUFFER : MacBackbuffer = MacBackbuffer{
                                          backbuffer : null_mut(),
                                          image : null_mut(),
                                        };

static mut BUFFER : Vec<u8> = Vec::new();

    //NOTE this was used to check loading the file worked can be deleted if you want
    //let bmp_file = NSString::alloc(nil).init_str("test.bmp");
    //let mut nsimage = NSImage::alloc(nil).initWithContentsOfFile_(bmp_file);


fn update_window<T: NSWindow + std::marker::Copy>(bitmap: &mut WindowCanvas, window: &T ){unsafe{
//TODO
//This method is leaking memory. I think I'm doing everything right and this is cocoa thing but who knows.

    let bitmapWidth  = window.contentView().bounds().size.width as usize;
    let bitmapHeight = window.contentView().bounds().size.height as usize;
    let bytesPerPixel = 4;
    let pitch = bitmapWidth * bytesPerPixel;
    let device_color_space = NSString::alloc(nil).init_str("NSDeviceRGBColorSpace");


    if BACKBUFFER.backbuffer != null_mut() {
        let _ : id = msg_send![BACKBUFFER.image, removeRepresentation: BACKBUFFER.backbuffer];
        let _ : id = msg_send![BACKBUFFER.backbuffer, dealloc]; 
    } 

    let count = bitmap.w as usize * 4;
    let mut temp = vec![0u8; count]; //TODO do not allocate this all the time
    for i in 0..bitmap.h as isize/2{

        let ptr_offset_high = bitmap.w as isize  * i * 4;
        let ptr_offset_low = (bitmap.h as isize  - i - 1)* bitmap.w as isize  * 4;
        std::ptr::copy_nonoverlapping(bitmap.buffer.offset( ptr_offset_low ) as *const u8, temp.as_mut_ptr(), count);
        std::ptr::copy_nonoverlapping(bitmap.buffer.offset( ptr_offset_high), bitmap.buffer.offset(ptr_offset_low), count);
        std::ptr::copy_nonoverlapping(temp.as_ptr(), bitmap.buffer.offset(ptr_offset_high) as *mut u8, count);
    }

    BACKBUFFER.backbuffer = msg_send![class!(NSBitmapImageRep), alloc];
    BACKBUFFER.backbuffer = msg_send![ BACKBUFFER.backbuffer, initWithBitmapDataPlanes: &bitmap.buffer as * const _//temp as * const _ 
                                                             pixelsWide: bitmapWidth
                                                             pixelsHigh: bitmapHeight
                                                             bitsPerSample: 8usize
                                                             samplesPerPixel: 4usize
                                                             hasAlpha: YES
                                                             isPlanar: NO
                                                             colorSpaceName: device_color_space
                                                             bytesPerRow: pitch
                                                             bitsPerPixel: bytesPerPixel * 8usize
                                                             ];


    let image_size = NSSize::new(bitmapWidth as f64, bitmapHeight as f64);
    if BACKBUFFER.image != null_mut() {
        let _ : id = msg_send![window.contentView().layer(), setContents: nil];
        let _ : id = msg_send![BACKBUFFER.image, recache]; 
    } else {
        BACKBUFFER.image = NSImage::initWithSize_(NSImage::alloc(BACKBUFFER.image), image_size );
    }


    let _ : id = msg_send![BACKBUFFER.image, addRepresentation: BACKBUFFER.backbuffer];
    let _ : id = msg_send![window.contentView().layer(), setContents: BACKBUFFER.image];

}}


extern fn on_enter_fullscreen(this: &Object, _cmd: Sel, _notification: id) {unsafe{
    let window: id = *this.get_ivar("window");
    window.setToolbar_(nil);
    RUNNING = false; //TODO
}}

extern fn will_close(this: &Object, _cmd: Sel,  _notification: id) {unsafe{
    RUNNING = false;
}}


extern fn did_move(this: &Object, _cmd: Sel, _notification: id) {unsafe{
}}

extern fn did_resize(this: &Object, _cmd: Sel,  _notification: id) {unsafe{
    

}}
 



static mut RUNNING : bool = true;
pub fn make_window() {unsafe{

    let app = NSApp();
    app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
    app.activateIgnoringOtherApps_(YES); 

    // create Menu Bar
    //let menubar = NSMenu::new(nil).autorelease();
    //let app_menu_item = NSMenuItem::new(nil).autorelease();
    //menubar.addItem_(app_menu_item);
    //app.setMainMenu_(menubar);

    //// create Application menu
    //let app_menu = NSMenu::new(nil).autorelease();
    //let quit_prefix = NSString::alloc(nil).init_str("Quit ");
    //let quit_title =
    //    quit_prefix.stringByAppendingString_(NSProcessInfo::processInfo(nil).processName());
    //let quit_action = selector("terminate:");
    //let quit_key = NSString::alloc(nil).init_str("q");
    //let quit_item = NSMenuItem::alloc(nil)
    //    .initWithTitle_action_keyEquivalent_(quit_title, quit_action, quit_key)
    //    .autorelease();
    //app_menu.addItem_(quit_item);
    //app_menu_item.setSubmenu_(app_menu);



    let init_frame =  NSRect::new(NSPoint::new(0., 0.), NSSize::new(1000., 550.));
    GLOBAL_WINDOWINFO.x = 0;
    GLOBAL_WINDOWINFO.y = 0;
    GLOBAL_WINDOWINFO.w = 1000;
    GLOBAL_WINDOWINFO.h = 500;

    // create Window
    let window = NSWindow::alloc(nil)
        .initWithContentRect_styleMask_backing_defer_( init_frame,
                                                      NSWindowStyleMask::NSTitledWindowMask|
                                                      NSWindowStyleMask::NSClosableWindowMask|
                                                      NSWindowStyleMask::NSResizableWindowMask |
                                                      NSWindowStyleMask::NSMiniaturizableWindowMask,
                                                      NSBackingStoreBuffered,
                                                      NO);
    let title = NSString::alloc(nil).init_str("Program 01");
    window.setTitle_(title);
    let bkg_color = NSWindow::backgroundColor(window);
    window.setBackgroundColor_(NSColor::colorWithRed_green_blue_alpha_(bkg_color, 0.01, 0.01, 0.01, 1.0)  );
    window.makeKeyAndOrderFront_(nil);
    window.contentView().setWantsLayer(YES);


///////////////////
//TODO
//remember me to resize from the app
//    window.setContentSize_(NSSize::new(500., 500.));
///////////////////


    window.setDelegate_(delegate!("MyWindowDelegate", 
    {
        window: id = window,
        //(onWindowWillEnterFullscreen:) => on_enter_fullscreen as extern fn(&Object, Sel, id), // Declare function(s)
        (windowWillClose:) => will_close as extern fn(&Object, Sel, id),
        (windowDidMove:) => did_move as extern fn(&Object, Sel, id),
        (windowDidResize:) => did_resize as extern fn(&Object, Sel, id)
    }));


    let bitmapWidth  = window.contentView().bounds().size.width as usize;
    let bitmapHeight = window.contentView().bounds().size.height as usize;
    let bytesPerPixel = 4;
    let pitch = bitmapWidth * bytesPerPixel;
    BUFFER = vec![0u8; pitch * bitmapHeight];


    GLOBAL_BACKBUFFER.w = bitmapWidth as i32;
    GLOBAL_BACKBUFFER.h = bitmapHeight as i32;
    GLOBAL_BACKBUFFER.info.width = GLOBAL_BACKBUFFER.w;
    GLOBAL_BACKBUFFER.info.height = GLOBAL_BACKBUFFER.h;
    
    GLOBAL_BACKBUFFER.buffer = BUFFER.as_mut_ptr() as *mut _; 




    let now = time::Instant::now();
    let mut elapsed = now.elapsed();

    let mut mouseinfo = MouseInfo::new();
    let mut textinfo = TextInfo{character: Vec::with_capacity(10), timing:Vec::new()};
    let mut keyboardinfo = KeyboardInfo{key: Vec::new(), status:Vec::new()};



    let mut ls_app_storage = LS_AppStorage::new();

//TODO check for frame work

    let mut orig_exe_path = std::env::current_exe().expect("could not find the exe path").to_str().unwrap().to_string();
    let mut _exe_path = std::env::current_exe().expect("could not find the exe path").to_str().unwrap().to_string();
    let mut exe_path = std::env::current_exe().expect("could not find the exe path");
    exe_path.pop();
    if !exe_path.to_string_lossy().contains("target/release"){
        if exe_path.to_string_lossy().contains("Contents/MacOS"){
            exe_path.pop();
            exe_path.pop();
            exe_path.pop();
            println!("{:?}", exe_path);
            _exe_path.clear();
            std::env::set_current_dir(exe_path).expect("could not do the thing");
             _exe_path = std::env::current_dir().expect("could not find the exe path").to_str().unwrap().to_string();
        } else {
            std::env::set_current_dir( exe_path );
        }
    }

    #[derive(Clone, Copy, Debug)]
    struct SpecialKey{
        ctrl : ButtonStatus,
        shift: ButtonStatus,
        _fn  : ButtonStatus,
        cmd  : ButtonStatus,
    }
    let mut specialkeys = SpecialKey{
                            ctrl : ButtonStatus::Up,
                            shift: ButtonStatus::Up,
                            _fn  : ButtonStatus::Up,
                            cmd  : ButtonStatus::Up,
                         };
    let mut old_specialkeys = specialkeys.clone();
    let mut stopwatch_lbutton = StopWatch::new();


    let mut old_windowinfo = WindowInfo{x:0, y:0, w:1000, h:550};
    loop{
        if RUNNING == false { 
            window.close();
            break; 
        }

        //TODO
        //window.setContentSize_(cocoa::foundation::NSSize::new(1500., 750.));
        if old_windowinfo != GLOBAL_WINDOWINFO{
            old_windowinfo = GLOBAL_WINDOWINFO;
            window.setContentSize_(cocoa::foundation::NSSize::new(GLOBAL_WINDOWINFO.w as _, GLOBAL_WINDOWINFO.h as _));
        }

        let _w = window.contentView().bounds().size.width as _;
        let _h = window.contentView().bounds().size.height as _;

        if _w != GLOBAL_BACKBUFFER.w 
        || _h != GLOBAL_BACKBUFFER.h{
            let new_size = _w * _h * 4;
            GLOBAL_BACKBUFFER.w  = _w;
            GLOBAL_BACKBUFFER.h = _h;

            BUFFER.resize(new_size as usize, 0);
            GLOBAL_BACKBUFFER.buffer = BUFFER.as_mut_ptr() as *mut _;
            
        }

        for i in 0..GLOBAL_BACKBUFFER.w * GLOBAL_BACKBUFFER.h{
            let _i = i * 4;
            let r = *(GLOBAL_BACKBUFFER.buffer as *mut u8).offset(_i as isize);
            *(GLOBAL_BACKBUFFER.buffer as *mut u8).offset(_i as isize) = *(GLOBAL_BACKBUFFER.buffer as *mut u8).offset(_i as isize + 2);
            *(GLOBAL_BACKBUFFER.buffer as *mut u8).offset(_i as isize + 2) = r;
            *(GLOBAL_BACKBUFFER.buffer as *mut u8).offset(_i as isize + 3) =  255; //* mult as u8;
        } 
        update_window( &mut GLOBAL_BACKBUFFER, &window );

        let ten_millis = time::Duration::from_millis(10);
        //thread::sleep(ten_millis);
        let temp =  now.elapsed() - elapsed;



        keyboardinfo.key.clear();
        keyboardinfo.status.clear();
        textinfo.character.clear();

        mouseinfo.wheel_delta = 0;
        mouseinfo.old_lbutton = mouseinfo.lbutton.clone(); //TODO(9/1/2020) keep an eye on this. It may cause an input bug for app_main.
        mouseinfo.old_rbutton = mouseinfo.rbutton.clone(); //TODO(9/1/2020) keep an eye on this. It may cause an input bug for app_main.

        let temp_rect = NSWindow::frame(window);
        GLOBAL_WINDOWINFO.x = temp_rect.origin.x.round() as i32;
        GLOBAL_WINDOWINFO.y = temp_rect.origin.y.round() as i32;
        GLOBAL_WINDOWINFO.w = window.contentView().bounds().size.width as _;
        GLOBAL_WINDOWINFO.h = window.contentView().bounds().size.height as _; //temp_rect.size.height.round() as i32;

        mouseinfo.delta_x = mouseinfo.x;
        mouseinfo.delta_y = mouseinfo.y;

        old_specialkeys = specialkeys.clone();
        mouseinfo.double_lbutton = false;

        let mut i = 0;
        loop  {
            i += 1;
            let mut event = NSApp().nextEventMatchingMask_untilDate_inMode_dequeue_( NSEventMask::all().bits(), nil, NSDefaultRunLoopMode, YES);
            if event == nil { break; }
            let (x, y) =  (NSEvent::mouseLocation(event).x, NSEvent::mouseLocation(event).y);
            mouseinfo.x = x.round() as i32 - GLOBAL_WINDOWINFO.x;
            mouseinfo.y = y.round() as i32 - GLOBAL_WINDOWINFO.y;

            

            use cocoa::appkit::NSEventModifierFlags;
            if i == 1{
                if event.modifierFlags().contains( NSEventModifierFlags::NSShiftKeyMask ){
                    specialkeys.shift = ButtonStatus::Down;
                } else {
                    specialkeys.shift = ButtonStatus::Up;
                } 
                if event.modifierFlags().contains( NSEventModifierFlags::NSControlKeyMask ){
                    specialkeys.ctrl = ButtonStatus::Down;
                } else {
                    specialkeys.ctrl = ButtonStatus::Up;
                }
                if event.modifierFlags().contains(  NSEventModifierFlags::NSCommandKeyMask ){
                    specialkeys.cmd = ButtonStatus::Down;
                } else{
                }
                if event.modifierFlags().contains(  NSEventModifierFlags::NSFunctionKeyMask ){
                    specialkeys._fn = ButtonStatus::Down;
                }else{
                    specialkeys._fn = ButtonStatus::Up;
                }
            }


            let mut keydown = 0;
            let mut keycode = std::usize::MAX;

            match event.eventType(){
                NSEventType::NSMouseMoved => {
                    //TODO doesn't work as expected
                    //mouseinfo.delta_x = NSEvent::deltaX(event).round() as i32;
                    //mouseinfo.delta_y = NSEvent::deltaY(event).round() as i32;
                },

                NSEventType::NSLeftMouseDown => {
                    mouseinfo.lbutton = ButtonStatus::Down;
                },
                NSEventType::NSLeftMouseUp => {
                    mouseinfo.lbutton = ButtonStatus::Up;
                },
                NSEventType::NSRightMouseDown => {
                    mouseinfo.rbutton = ButtonStatus::Down;
                },
                NSEventType::NSRightMouseUp => {
                    mouseinfo.rbutton = ButtonStatus::Up;
                },
                NSEventType::NSKeyDown => {
                    keydown = 1;
                    if  NSEvent::keyCode(event) == 53 {
                        RUNNING = false;
                    }
                    keycode = NSEvent::keyCode(event) as usize;

                    //TODO we are not properly handleing utf8
                    // std::mem::transmute::<u32, char>(*NSString::UTF8String(NSEvent::characters(event) ).offset(0) as u32);
                    //TODO this prob should not be done on a keydown event
                    use std::ffi::CString;
                    if textinfo.character.len() < 50 { textinfo.character.push(*NSString::UTF8String(NSEvent::characters(event) ) as u8 as char); }
                },
                NSEventType::NSKeyUp => {
                    keydown = 2;
                    keycode = NSEvent::keyCode(event) as usize;
                },
                NSEventType::NSFlagsChanged => {
                },
                _=>{}
            }
            {//Handle keyboard events
                let keyinfo = KeyMessageMacOS{ keydown: keydown, keycode: keycode};
                keyboardinfo.update_keyboardinfo_macos(keyinfo);


            }
            let _ : id = msg_send![NSApp(), sendEvent: event];
        }
        mouseinfo.delta_x = mouseinfo.x - mouseinfo.delta_x;
        mouseinfo.delta_y = mouseinfo.y - mouseinfo.delta_y;

        if specialkeys.ctrl != old_specialkeys.ctrl {
            keyboardinfo.key.push(KeyboardEnum::Ctrl);
            keyboardinfo.status.push(specialkeys.ctrl);
        }
        if specialkeys.shift != old_specialkeys.shift {
            keyboardinfo.key.push(KeyboardEnum::Shift);
            keyboardinfo.status.push(specialkeys.shift);
        }
        if specialkeys._fn != old_specialkeys._fn {
            keyboardinfo.key.push(KeyboardEnum::_Fn);
            keyboardinfo.status.push(specialkeys._fn);
        }
        if specialkeys.cmd != old_specialkeys.cmd {
            keyboardinfo.key.push(KeyboardEnum::Cmd);
            keyboardinfo.status.push(specialkeys.cmd);
        }

        if mouseinfo.lbutton == ButtonStatus::Up
        && mouseinfo.old_lbutton == ButtonStatus::Down{
            stopwatch_lbutton.reset_lap_timer();
        }
        if mouseinfo.lbutton == ButtonStatus::Down
        && mouseinfo.old_lbutton == ButtonStatus::Up
        && stopwatch_lbutton.lap_time().as_millis() <= 500 {
            mouseinfo.double_lbutton = true;
        }

        
        if circuit_sim(&mut OsPackage{window_canvas: &mut GLOBAL_BACKBUFFER, window_info: &mut GLOBAL_WINDOWINFO},
                    &mut ls_app_storage, &keyboardinfo, &textinfo, &mouseinfo) != 0 { break; }
        //if cloud_game(&mut OsPackage{window_canvas: &mut GLOBAL_BACKBUFFER, window_info: &mut GLOBAL_WINDOWINFO},
        //            &mut cg_app_storage, &keyboardinfo, &textinfo, &mouseinfo) != 0 { break; }

        draw_string(&mut GLOBAL_BACKBUFFER, &orig_exe_path, 150, 150, C4_WHITE, 30f32);
        draw_string(&mut GLOBAL_BACKBUFFER, &_exe_path, 150, 150-30, C4_WHITE, 30f32);
        elapsed = now.elapsed();


    }

}}

