//! This module is contains structs and functions that handle event input.
//!
//! There are three basic structs defined by this module.
//! They are `KeyboardInfo`, `MouseInfo`, and `TextInfo`.
//! `KeyboardInfo` holds current keyboard events. 
//! `MouseInfo` holds current mouse events.
//! `TextInfo` holds the most recent character, as defined by the operating system.
//!



#![allow(unused)]

#[cfg(target_os = "windows")]
use winapi::um::winuser::*;//{MSG, VK_*};


#[cfg(target_os = "linux")]
use crate::x11::xlib::KeySym;
#[cfg(target_os = "linux")]
use crate::x11::keysym::*;


#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ButtonStatus{
    Up,
    Down
}
impl Default for ButtonStatus{
    fn default()->ButtonStatus{
        ButtonStatus::Up
    }
}


/// MouseInfo is designed to interface with a standard two button plus middle wheel mouse. 
///
/// MouseInfo is to be filled out by the operating system specific modules.
/// A basic left click example is given below.
/// ## Example
/// ```
/// ...
/// let mut mouseinfo =  MouseInfo::new();
/// {//Get events
///     ...
/// }
///
/// let left_button_clicked = mouseinfo.lclicked();
/// if left_button_clicked {
///     //Do things when left button is clicked.
/// }
/// ```
#[derive(Debug)]
pub struct MouseInfo{
    pub x: i32,
    pub y: i32,
    pub delta_x: i32,
    pub delta_y: i32,

    pub lbutton: ButtonStatus,
    pub old_lbutton: ButtonStatus,

    pub double_lbutton: bool,

    pub rbutton: ButtonStatus,
    pub old_rbutton: ButtonStatus,

    pub wheel: isize,
    pub wheel_delta: i32,
}
impl MouseInfo{
    pub fn new()->MouseInfo{
        MouseInfo{
            x: 0,
            y: 0,
            delta_x: 0,
            delta_y: 0,

            lbutton: ButtonStatus::Up,
            old_lbutton: ButtonStatus::Up,

            double_lbutton: false,

            rbutton: ButtonStatus::Up,
            old_rbutton: ButtonStatus::Up,

            wheel: 0,
            wheel_delta: 0,
        }
    }

    pub fn lclicked(&self)->bool{
        return self.lbutton == ButtonStatus::Up && self.old_lbutton == ButtonStatus::Down ;
    }

    pub fn rclicked(&self)->bool{
        return self.rbutton == ButtonStatus::Up && self.old_rbutton == ButtonStatus::Down ;
    }
}



#[derive(PartialEq, Debug)]
pub enum KeyboardEnum{
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    N1, N2, N3, N4, N5, N6, N7, N8, N9, N0,
    Pad1, Pad2, Pad3, Pad4, Pad5, Pad6, Pad7, Pad8, Pad9, Pad0,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    Lctrl, Rctrl, Ctrl,
    Lshift, Rshift, Shift,
    _Fn,
    Cmd,
    Tab,
    Space,
    Rightarrow,
    Leftarrow,
    Uparrow,
    Downarrow,
    Enter,
    Delete,
    Backspace,
    Default
}


/// TextInfo is contains utf-8 compatible character and associated time steps.
///
/// TextInfo is to be filled by operating system modules.
pub struct TextInfo{
    pub character: Vec<char>,
    pub timing: Vec<i32>
}

/// KeyboardInfo is contains keyboard button status.
///
/// KeyboardInfo is to be filled by operating system modules.
pub struct KeyboardInfo{
    pub key: Vec<KeyboardEnum>,
    pub status: Vec<ButtonStatus>,

}

macro_rules! update_down{
    ($bool:tt, $m:tt, $x:expr, $y:tt, $keyboardinfo:tt) => {
        if $m == $x {
            $keyboardinfo.key.push($y);
            if $bool {
                $keyboardinfo.status.push(ButtonStatus::Down);
            } else {
                $keyboardinfo.status.push(ButtonStatus::Up);
            }
        }
    }
}



pub struct KeyMessageMacOS{
    pub keycode: usize,
    pub keydown: usize, //1 is down, 2 is up, zero is bull
}



impl KeyboardInfo{
    pub fn is_key_released(&self, key: KeyboardEnum)->bool{
        for (i, it) in self.key.iter().enumerate(){
            if *it == key
            && self.status[i] == ButtonStatus::Up{
                return true;
            }
        }

        return false;
    }


    pub fn is_key_pressed(&self, key: KeyboardEnum)->bool{
        for (i, it) in self.key.iter().enumerate(){
            if *it == key
            && self.status[i] == ButtonStatus::Down{
                return true;
            }
        }
        return false;
    }

    #[cfg(target_os = "windows")]
    pub fn update_keyboardinfo_windows(&mut self, message: &MSG){
        //NOTE
        //Becareful with KeyboardEnum! There are many single character values in the enum

        use KeyboardEnum::*;
        use ButtonStatus::*;
        if message.message == WM_KEYDOWN || message.message == WM_KEYUP{
            
            let is_down = message.message == WM_KEYDOWN;
            let message_wparam = message.wParam;

            update_down!(is_down, message_wparam, VK_LEFT as usize , Leftarrow, self);
            update_down!(is_down, message_wparam, VK_RIGHT as usize , Rightarrow, self);
            update_down!(is_down, message_wparam, VK_UP as usize , Uparrow, self);
            update_down!(is_down, message_wparam, VK_DOWN as usize , Downarrow, self);

            update_down!(is_down, message_wparam, VK_SPACE as usize , Space, self);
            update_down!(is_down, message_wparam, VK_RETURN as usize , Enter, self);
            update_down!(is_down, message_wparam, VK_DELETE as usize , Delete, self);

            update_down!(is_down, message_wparam, VK_LSHIFT as usize , Lshift, self); //NOTE doesn't work(not with my keyboard atleast) 10/23/2020
            update_down!(is_down, message_wparam, VK_RSHIFT as usize , Rshift, self); //NOTE doesn't work(not with my keyboard atleast) 10/23/2020
            update_down!(is_down, message_wparam, VK_SHIFT as usize , Shift, self);

            update_down!(is_down, message_wparam, VK_LCONTROL as usize , Lctrl, self); //NOTE doesn't work(not with my keyboard atleast) 10/23/2020
            update_down!(is_down, message_wparam, VK_RCONTROL as usize , Rctrl, self); //NOTE doesn't work(not with my keyboard atleast) 10/23/2020
            update_down!(is_down, message_wparam, VK_CONTROL as usize , Ctrl, self);

            update_down!(is_down, message_wparam, VK_TAB as usize , Tab, self);

            update_down!(is_down, message_wparam, VK_F1 as usize , F1, self);
            update_down!(is_down, message_wparam, VK_F2 as usize , F2, self);
            update_down!(is_down, message_wparam, VK_F3 as usize , F3, self);
            update_down!(is_down, message_wparam, VK_F4 as usize , F4, self);
            update_down!(is_down, message_wparam, VK_F5 as usize , F5, self);
            update_down!(is_down, message_wparam, VK_F6 as usize , F6, self);
            update_down!(is_down, message_wparam, VK_F7 as usize , F7, self);
            update_down!(is_down, message_wparam, VK_F8 as usize , F8, self);
            update_down!(is_down, message_wparam, VK_F9 as usize , F9, self);
            update_down!(is_down, message_wparam, VK_F10 as usize , F10, self);//NOTE doesn't seem to work
            update_down!(is_down, message_wparam, VK_F11 as usize , F11, self);//NOTE doesn't seem to work
            update_down!(is_down, message_wparam, VK_F12 as usize , F12, self);//NOTE doesn't seem to work

            update_down!(is_down, message_wparam, VK_NUMPAD0 as usize , Pad0, self);
            update_down!(is_down, message_wparam, VK_NUMPAD1 as usize , Pad1, self);
            update_down!(is_down, message_wparam, VK_NUMPAD2 as usize , Pad2, self);
            update_down!(is_down, message_wparam, VK_NUMPAD3 as usize , Pad3, self);
            update_down!(is_down, message_wparam, VK_NUMPAD4 as usize , Pad4, self);
            update_down!(is_down, message_wparam, VK_NUMPAD5 as usize , Pad5, self);
            update_down!(is_down, message_wparam, VK_NUMPAD6 as usize , Pad6, self);
            update_down!(is_down, message_wparam, VK_NUMPAD7 as usize , Pad7, self);
            update_down!(is_down, message_wparam, VK_NUMPAD8 as usize , Pad8, self);
            update_down!(is_down, message_wparam, VK_NUMPAD9 as usize , Pad9, self);

            update_down!(is_down, message_wparam, 0x30usize , N0, self);
            update_down!(is_down, message_wparam, 0x31usize , N1, self);
            update_down!(is_down, message_wparam, 0x32usize , N2, self);
            update_down!(is_down, message_wparam, 0x33usize , N3, self);
            update_down!(is_down, message_wparam, 0x34usize , N4, self);
            update_down!(is_down, message_wparam, 0x35usize , N5, self);
            update_down!(is_down, message_wparam, 0x36usize , N6, self);
            update_down!(is_down, message_wparam, 0x37usize , N7, self);
            update_down!(is_down, message_wparam, 0x38usize , N8, self);
            update_down!(is_down, message_wparam, 0x39usize , N9, self);

            update_down!(is_down, message_wparam, 0x41usize , A, self);
            update_down!(is_down, message_wparam, 0x42usize , B, self);
            update_down!(is_down, message_wparam, 0x43usize , C, self);
            update_down!(is_down, message_wparam, 0x44usize , D, self);
            update_down!(is_down, message_wparam, 0x45usize , E, self);
            update_down!(is_down, message_wparam, 0x46usize , F, self);
            update_down!(is_down, message_wparam, 0x47usize , G, self);
            update_down!(is_down, message_wparam, 0x48usize , H, self);
            update_down!(is_down, message_wparam, 0x49usize , I, self);
            update_down!(is_down, message_wparam, 0x4Ausize , J, self);
            update_down!(is_down, message_wparam, 0x4Busize , K, self);
            update_down!(is_down, message_wparam, 0x4Cusize , L, self);
            update_down!(is_down, message_wparam, 0x4Dusize , M, self);
            update_down!(is_down, message_wparam, 0x4Eusize , N, self);
            update_down!(is_down, message_wparam, 0x4Fusize , O, self);
            update_down!(is_down, message_wparam, 0x50usize , P, self);
            update_down!(is_down, message_wparam, 0x51usize , Q, self);
            update_down!(is_down, message_wparam, 0x52usize , R, self);
            update_down!(is_down, message_wparam, 0x53usize , S, self);
            update_down!(is_down, message_wparam, 0x54usize , T, self);
            update_down!(is_down, message_wparam, 0x55usize , U, self);
            update_down!(is_down, message_wparam, 0x56usize , V, self);
            update_down!(is_down, message_wparam, 0x57usize , W, self);
            update_down!(is_down, message_wparam, 0x58usize , X, self);
            update_down!(is_down, message_wparam, 0x59usize , Y, self);
            update_down!(is_down, message_wparam, 0x5Ausize , Z, self);
        }
    }

    #[cfg(target_os = "macos")]
    pub fn update_keyboardinfo_macos(&mut self, message: KeyMessageMacOS){ //TODO should have a special struct for this so that it's easier to fix
        //NOTE
        //Becareful with KeyboardEnum! There are many single character values in the enum

        use KeyboardEnum::*;
        use ButtonStatus::*;
        if message.keydown == 1 || message.keydown == 2{
                            //if message.wParam == winapi::um::winuser::VK_LEFT as usize{
            let is_down = message.keydown == 1;
            let message_wparam = message.keycode;

            update_down!(is_down, message_wparam, 0x7B , Leftarrow, self);
            update_down!(is_down, message_wparam, 0x7C , Rightarrow, self);
            update_down!(is_down, message_wparam, 0x7E , Uparrow, self);
            update_down!(is_down, message_wparam, 0x7D , Downarrow, self);

            update_down!(is_down, message_wparam, 0x31 , Space, self);
            update_down!(is_down, message_wparam, 0x24 , Enter, self);
            update_down!(is_down, message_wparam, 117 , Delete, self);
            update_down!(is_down, message_wparam, 51 , Backspace, self);

            update_down!(is_down, message_wparam, 0x38 , Shift, self);
            //update_down!(is_down, message_wparam, VK_LSHIFT as usize , Lshift, self);
            //update_down!(is_down, message_wparam, VK_RSHIFT as usize , Rshift, self);

            update_down!(is_down, message_wparam, 0x3B , Ctrl, self);
            //update_down!(is_down, message_wparam, VK_LCONTROL as usize , Lctrl, self);
            //update_down!(is_down, message_wparam, VK_RCONTROL as usize , Rctrl, self);


            update_down!(is_down, message_wparam, 0x30 , Tab, self);


            //update_down!(is_down, message_wparam, VK_F1 as usize , F1, self);
            //update_down!(is_down, message_wparam, VK_F2 as usize , F2, self);
            //update_down!(is_down, message_wparam, VK_F3 as usize , F3, self);
            //update_down!(is_down, message_wparam, VK_F4 as usize , F4, self);
            //update_down!(is_down, message_wparam, VK_F5 as usize , F5, self);
            //update_down!(is_down, message_wparam, VK_F6 as usize , F6, self);
            //update_down!(is_down, message_wparam, VK_F7 as usize , F7, self);
            //update_down!(is_down, message_wparam, VK_F8 as usize , F8, self);
            //update_down!(is_down, message_wparam, VK_F9 as usize , F9, self);
            //update_down!(is_down, message_wparam, VK_F10 as usize , F10, self);
            //update_down!(is_down, message_wparam, VK_F11 as usize , F11, self);
            //update_down!(is_down, message_wparam, VK_F12 as usize , F12, self);

            //update_down!(is_down, message_wparam, VK_NUMPAD0 as usize , Pad0, self);
            //update_down!(is_down, message_wparam, VK_NUMPAD1 as usize , Pad1, self);
            //update_down!(is_down, message_wparam, VK_NUMPAD2 as usize , Pad2, self);
            //update_down!(is_down, message_wparam, VK_NUMPAD3 as usize , Pad3, self);
            //update_down!(is_down, message_wparam, VK_NUMPAD4 as usize , Pad4, self);
            //update_down!(is_down, message_wparam, VK_NUMPAD5 as usize , Pad5, self);
            //update_down!(is_down, message_wparam, VK_NUMPAD6 as usize , Pad6, self);
            //update_down!(is_down, message_wparam, VK_NUMPAD7 as usize , Pad7, self);
            //update_down!(is_down, message_wparam, VK_NUMPAD8 as usize , Pad8, self);
            //update_down!(is_down, message_wparam, VK_NUMPAD9 as usize , Pad9, self);

            //update_down!(is_down, message_wparam, 0x30usize , N0, self);
            //update_down!(is_down, message_wparam, 0x31usize , N1, self);
            //update_down!(is_down, message_wparam, 0x32usize , N2, self);
            //update_down!(is_down, message_wparam, 0x33usize , N3, self);
            //update_down!(is_down, message_wparam, 0x34usize , N4, self);
            //update_down!(is_down, message_wparam, 0x35usize , N5, self);
            //update_down!(is_down, message_wparam, 0x36usize , N6, self);
            //update_down!(is_down, message_wparam, 0x37usize , N7, self);
            //update_down!(is_down, message_wparam, 0x38usize , N8, self);
            //update_down!(is_down, message_wparam, 0x39usize , N9, self);

            update_down!(is_down, message_wparam, 0x00usize , A, self);
            update_down!(is_down, message_wparam, 0x0Busize , B, self);
            update_down!(is_down, message_wparam, 0x08usize , C, self);
            update_down!(is_down, message_wparam, 0x02usize , D, self);
            update_down!(is_down, message_wparam, 0x0Eusize , E, self);
            update_down!(is_down, message_wparam, 0x03usize , F, self);
            update_down!(is_down, message_wparam, 0x05usize , G, self);
            update_down!(is_down, message_wparam, 0x04usize , H, self);
            update_down!(is_down, message_wparam, 0x22usize , I, self);
            update_down!(is_down, message_wparam, 0x26usize , J, self);
            update_down!(is_down, message_wparam, 0x28usize , K, self);
            update_down!(is_down, message_wparam, 0x25usize , L, self);
            update_down!(is_down, message_wparam, 0x2Eusize , M, self);
            update_down!(is_down, message_wparam, 0x2Dusize , N, self);
            update_down!(is_down, message_wparam, 0x1Fusize , O, self);
            update_down!(is_down, message_wparam, 0x23usize , P, self);
            update_down!(is_down, message_wparam, 0x0Cusize , Q, self);
            update_down!(is_down, message_wparam, 0x0Fusize , R, self);
            update_down!(is_down, message_wparam, 0x01usize , S, self);
            update_down!(is_down, message_wparam, 0x11usize , T, self);
            update_down!(is_down, message_wparam, 0x20usize , U, self);
            update_down!(is_down, message_wparam, 0x09usize , V, self);
            update_down!(is_down, message_wparam, 0x0Dusize , W, self);
            update_down!(is_down, message_wparam, 0x07usize , X, self);
            update_down!(is_down, message_wparam, 0x10usize , Y, self);
            update_down!(is_down, message_wparam, 0x06usize , Z, self);

        }
    }




    #[cfg(target_os = "linux")]
    pub fn update_keyboardinfo_linux(&mut self, message: KeySym, keystatus: bool){ //TODO should have a special struct for this so that it's easier to fix
        //NOTE
        //Becareful with KeyboardEnum! There are many single character values in the enum

        use KeyboardEnum::*;
        use ButtonStatus::*;
                            
        let is_down = keystatus;
        let message_wparam = message;

        update_down!(is_down, message_wparam, XK_Left as u64, Leftarrow, self);
        update_down!(is_down, message_wparam, XK_Right as u64 , Rightarrow, self);
        update_down!(is_down, message_wparam, XK_Up as u64 , Uparrow, self);
        update_down!(is_down, message_wparam, XK_Down as u64 , Downarrow, self);

        update_down!(is_down, message_wparam, XK_space as u64 , Space, self);
        update_down!(is_down, message_wparam, XK_Return as u64 , Enter, self);
        update_down!(is_down, message_wparam, XK_Delete as u64 , Delete, self);

        update_down!(is_down, message_wparam, XK_Shift_L as u64 , Lshift, self);
        update_down!(is_down, message_wparam, XK_Shift_R as u64 , Rshift, self);
        update_down!(is_down, message_wparam, XK_Shift_L as u64 , Shift, self);
        update_down!(is_down, message_wparam, XK_Shift_R as u64 , Shift, self);

        update_down!(is_down, message_wparam, XK_Control_L as u64 , Lctrl, self);
        update_down!(is_down, message_wparam, XK_Control_R as u64 , Rctrl, self);
        update_down!(is_down, message_wparam, XK_Control_L as u64 , Ctrl, self);
        update_down!(is_down, message_wparam, XK_Control_R as u64 , Ctrl, self);


        update_down!(is_down, message_wparam, XK_Tab as u64 , Tab, self);


        update_down!(is_down, message_wparam, XK_F1 as u64 , F1, self);
        update_down!(is_down, message_wparam, XK_F2 as u64 , F2, self);
        update_down!(is_down, message_wparam, XK_F3 as u64 , F3, self);
        update_down!(is_down, message_wparam, XK_F4 as u64 , F4, self);
        update_down!(is_down, message_wparam, XK_F5 as u64 , F5, self);
        update_down!(is_down, message_wparam, XK_F6 as u64 , F6, self);
        update_down!(is_down, message_wparam, XK_F7 as u64 , F7, self);
        //update_down!(is_down, message_wparam, VK_F8 as usize , F8, self);
        //update_down!(is_down, message_wparam, VK_F9 as usize , F9, self);
        //update_down!(is_down, message_wparam, VK_F10 as usize , F10, self);
        //update_down!(is_down, message_wparam, VK_F11 as usize , F11, self);
        //update_down!(is_down, message_wparam, VK_F12 as usize , F12, self);

        //update_down!(is_down, message_wparam, VK_NUMPAD0 as usize , Pad0, self);
        //update_down!(is_down, message_wparam, VK_NUMPAD1 as usize , Pad1, self);
        //update_down!(is_down, message_wparam, VK_NUMPAD2 as usize , Pad2, self);
        //update_down!(is_down, message_wparam, VK_NUMPAD3 as usize , Pad3, self);
        //update_down!(is_down, message_wparam, VK_NUMPAD4 as usize , Pad4, self);
        //update_down!(is_down, message_wparam, VK_NUMPAD5 as usize , Pad5, self);
        //update_down!(is_down, message_wparam, VK_NUMPAD6 as usize , Pad6, self);
        //update_down!(is_down, message_wparam, VK_NUMPAD7 as usize , Pad7, self);
        //update_down!(is_down, message_wparam, VK_NUMPAD8 as usize , Pad8, self);
        //update_down!(is_down, message_wparam, VK_NUMPAD9 as usize , Pad9, self);

        //update_down!(is_down, message_wparam, 0x30usize , N0, self);
        //update_down!(is_down, message_wparam, 0x31usize , N1, self);
        //update_down!(is_down, message_wparam, 0x32usize , N2, self);
        //update_down!(is_down, message_wparam, 0x33usize , N3, self);
        //update_down!(is_down, message_wparam, 0x34usize , N4, self);
        //update_down!(is_down, message_wparam, 0x35usize , N5, self);
        //update_down!(is_down, message_wparam, 0x36usize , N6, self);
        //update_down!(is_down, message_wparam, 0x37usize , N7, self);
        //update_down!(is_down, message_wparam, 0x38usize , N8, self);
        //update_down!(is_down, message_wparam, 0x39usize , N9, self);

        update_down!(is_down, message_wparam, XK_a as u64 , A, self);
        update_down!(is_down, message_wparam, XK_b as u64 , B, self);
        update_down!(is_down, message_wparam, XK_c as u64 , C, self);
        update_down!(is_down, message_wparam, XK_d as u64 , D, self);
        update_down!(is_down, message_wparam, XK_e as u64 , E, self);
        update_down!(is_down, message_wparam, XK_f as u64 , F, self);
        update_down!(is_down, message_wparam, XK_g as u64 , G, self);
        update_down!(is_down, message_wparam, XK_h as u64 , H, self);
        update_down!(is_down, message_wparam, XK_i as u64 , I, self);
        update_down!(is_down, message_wparam, XK_j as u64 , J, self);
        update_down!(is_down, message_wparam, XK_k as u64 , K, self);
        update_down!(is_down, message_wparam, XK_l as u64 , L, self);
        update_down!(is_down, message_wparam, XK_m as u64 , M, self);
        update_down!(is_down, message_wparam, XK_n as u64 , N, self);
        update_down!(is_down, message_wparam, XK_o as u64 , O, self);
        update_down!(is_down, message_wparam, XK_p as u64 , P, self);
        update_down!(is_down, message_wparam, XK_q as u64 , Q, self);
        update_down!(is_down, message_wparam, XK_r as u64 , R, self);
        update_down!(is_down, message_wparam, XK_s as u64 , S, self);
        update_down!(is_down, message_wparam, XK_t as u64 , T, self);
        update_down!(is_down, message_wparam, XK_u as u64 , U, self);
        update_down!(is_down, message_wparam, XK_v as u64 , V, self);
        update_down!(is_down, message_wparam, XK_w as u64 , W, self);
        update_down!(is_down, message_wparam, XK_x as u64 , X, self);
        update_down!(is_down, message_wparam, XK_y as u64 , Y, self);
        update_down!(is_down, message_wparam, XK_z as u64 , Z, self);

    }

}
