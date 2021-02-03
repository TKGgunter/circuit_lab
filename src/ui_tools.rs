use crate::{WindowCanvas, OsPackage};
use crate::rendertools::*;
use crate::inputhandler::*;
use crate::misc::*;
use std::ptr::{null, null_mut};

//NOTE this is not thread safe.
//Every thing is in global space, not too nice when working with canvases of multiple sizes

//TODO offsets are needed if drawing to something that is not the main canvas

static mut CANVAS : Option<&mut WindowCanvas> = None;
static mut BUTTON_BG_COLOR1 : [f32; 4] = C4_DGREY;
static mut BUTTON_BG_COLOR2 : [f32; 4] = C4_GREY;
static mut BUTTON_TEXT_COLOR  : [f32; 4] = C4_LGREY;
static mut BUTTON_TEXT_FONT_SIZE : f32 = 30f32;


#[derive(Default)]
pub struct ButtonResult{
    pub in_rect: bool,
    pub lclicked: bool,
    pub rect: [i32; 4]
}
pub fn set_canvas(wc: &'static mut WindowCanvas){unsafe{
    //TODO set offsets?
    CANVAS = Some(wc);
}}



pub fn set_button_bg_color1(c: [f32; 4]){unsafe{
    BUTTON_BG_COLOR1 = c;
}}
pub fn reset_button_bg_color1(){unsafe{
    BUTTON_BG_COLOR1 = C4_DGREY;
}}



pub fn set_button_bg_color2(c: [f32; 4]){unsafe{
    BUTTON_BG_COLOR2 = c;
}}
pub fn reset_button_bg_color2(){unsafe{
    BUTTON_BG_COLOR2 = C4_GREY;
}}



pub fn set_button_text_color(c: [f32; 4]){unsafe{
    BUTTON_TEXT_COLOR = c;
}}

pub fn reset_button_text_color(c: [f32; 4]){unsafe{
    BUTTON_TEXT_COLOR = C4_LGREY;
}}


pub fn calc_button_height(size: f32)->i32{
    let mut rt = size*1.4;
    return rt as i32;
}
pub fn basic_button( canvas: &mut WindowCanvas, text: &str, x: i32, y:i32, font_size: f32, mouseinfo: &MouseInfo )->ButtonResult{unsafe{
    let mut rt = ButtonResult::default();

    let mut color_bg = BUTTON_BG_COLOR1;

    //TODO
    //let text_len = (get_advance_string(text, BUTTON_TEXT_FONT_SIZE) as f32 * 1.10) as i32;
    let text_len = (get_advance_string(text, font_size) as f32 * 1.10) as i32;
    let button_height = calc_button_height(font_size);

    //let mut rect = [x - text_len / 2 - 6, 
    //                y-font_size as i32/2 - (button_height - font_size as i32) / 2, 
    //                text_len, 
    //                button_height];
    let mut rect = [x, 
                    y, 
                    text_len, 
                    button_height];

    rt.in_rect = in_rect(mouseinfo.x, mouseinfo.y, rect);
    if rt.in_rect {
        color_bg = BUTTON_BG_COLOR2;
        rt.lclicked = mouseinfo.lclicked();
    }

    draw_rect(canvas, rect, color_bg, true);
    draw_string(canvas, text, rect[0], rect[1], BUTTON_TEXT_COLOR, font_size);

    rt.rect = rect;
    return rt;
}}



pub fn gb_basic_button( text: &str, x: i32, y:i32, mouseinfo: &MouseInfo )->ButtonResult{unsafe{
    let mut rt = ButtonResult::default();
    if CANVAS.is_none() { 
        println!("Canvas has not been set");
        return rt; 
    }

    rt = basic_button(CANVAS.as_mut().unwrap(), text, x, y, BUTTON_TEXT_FONT_SIZE, mouseinfo);
    return rt;
}}


static mut SLIDER_COLOR1 : [f32; 4] = C4_WHITE;
static mut SLIDER_COLOR2 : [f32; 4] = C4_LGREY;
pub fn set_slider_color1(c: [f32; 4]){unsafe{
    SLIDER_COLOR1 = c;
}}
pub fn set_slider_color2(c: [f32; 4]){unsafe{
    SLIDER_COLOR2 = c;
}}

#[derive(Default)]
pub struct SliderResult{
    pub ldown: bool,
    pub frac: f32,
}
pub fn basic_horizslider(canvas: &mut WindowCanvas, frac: f32, rect: [i32;4], mouseinfo: &MouseInfo)->SliderResult{
    let mut slider = HorizSlider::new();
    slider.x      = rect[0];
    slider.y      = rect[1];
    slider.width  = rect[2];
    slider.height = rect[3];

    slider.button_w = (slider.width / 20).max(10);
    slider.button_h = (slider.height * 10).min(50);
    slider.percentage = frac;

    slider.update( mouseinfo );
    unsafe{ slider.draw(canvas, SLIDER_COLOR1, SLIDER_COLOR2); }
    return SliderResult{ldown: mouseinfo.lbutton == ButtonStatus::Down, frac: slider.percentage};
}
pub fn gb_horizslider(frac: f32, rect: [i32;4], mouseinfo: &MouseInfo)->SliderResult{unsafe{
    if CANVAS.is_none() { 
        println!("Canvas has not been set");
        return SliderResult::default(); 
    }
    let canvas = CANVAS.as_mut().unwrap();
    return basic_horizslider(canvas, frac, rect, mouseinfo);
}}

pub struct HorizSlider{
    pub width : i32,
    pub height: i32,
    pub x     : i32,
    pub y     : i32,

    pub button_w: i32,
    pub button_h: i32,
    pub button_selected: bool, 
    pub percentage: f32,

}
impl HorizSlider{
    pub fn new()->HorizSlider{
        HorizSlider{
            width : 0i32,
            height: 0i32,
            x     : 0i32,
            y     : 0i32,

            button_w: 0i32,
            button_h: 0i32,
            button_selected: false, 
            percentage: 0.0f32,

        }
    }
    pub fn update(&mut self, mouseinfo: &MouseInfo){
        let x = self.x + ( self.width as f32 * self.percentage) as i32 - self.button_w/2;
        let y = self.y-self.button_h/2 + self.height/2;

        if in_rect(mouseinfo.x, mouseinfo.y, [x, y, self.button_w, self.button_h]){
            if mouseinfo.lbutton == ButtonStatus::Down{
                self.button_selected = true;
            } else {
                self.button_selected = false;
            }
        } else {
            if mouseinfo.lbutton == ButtonStatus::Down{
                self.button_selected = false;
            }
        }
        if self.button_selected {
            let per = (( mouseinfo.x - self.x ) as f32 / self.width as f32).max(0.0).min(1.0);
            self.percentage = per; 
        }
    }
    pub fn draw(&self, canvas: &mut WindowCanvas, bar_color: [f32; 4], button_color: [f32; 4]){
        draw_rect(canvas, [self.x, self.y, self.width, self.height], bar_color, true);

        let x = self.x + ( self.width as f32 * self.percentage) as i32 - self.button_w/2;
        let y = self.y-self.button_h/2 + self.height/2;

        draw_rect(canvas, [x, y, self.button_w, self.button_h], button_color, true);
    }
}



pub struct TextBoxResult{
    pub text_buffer: String,
    pub active: bool,
    pub cursor: usize
}

//remove n characters to a static mut
//remove time to a static mut
pub fn basic_textbox(canvas: &mut WindowCanvas, input: &str, xy: [i32;2], n_chars: i32, time: f32, active: bool, mouseinfo: &MouseInfo, textinfo: &TextInfo, keyboardinfo: &KeyboardInfo)->TextBoxResult{
    let mut textbox = TextBox::new();
    textbox.text_buffer = input.to_string();
    textbox.max_char = n_chars;
    textbox.active = active;
    textbox.max_render_length = get_advance_string(&"X".repeat(textbox.max_char as _), textbox.text_size );

    textbox.update(keyboardinfo, textinfo, mouseinfo);
    textbox.draw(canvas, time);

    return TextBoxResult{ text_buffer: textbox.text_buffer.clone(), active: textbox.active, cursor: textbox.text_cursor};
}


#[derive(Clone)]
pub struct TextBox{
    pub text_buffer: String,//TODO should this be a tiny string?
    pub text_cursor: usize,
    pub max_char: i32,
    pub max_render_length: i32,
    pub text_size: f32,
    pub x: i32,
    pub y: i32,
    pub text_color:[f32;4],
    pub bg_color:[f32;4],
    pub cursor_color:[f32;4],
    pub omega: f32,
    pub active: bool,
}
impl TextBox{
    pub fn new()->TextBox{
        TextBox{
            text_buffer: String::new(),
            text_cursor: 0,
            max_char: 30,
            max_render_length: 200,
            text_size: 24.0,
            x: 0,
            y: 0,
            text_color:[0.8;4],
            cursor_color:[0.8;4],
            bg_color:[1.0, 1.0, 1.0, 0.1],
            omega: 1.0f32,
            active: false,
        }
    }
    pub fn update(&mut self, keyboardinfo : &KeyboardInfo, textinfo: &TextInfo, mouseinfo: &MouseInfo){
        fn placeCursor(_self: &mut TextBox, mouseinfo: &MouseInfo){//Look for where to place cursor
            let mut position = 0;
            for (i, it) in _self.text_buffer.chars().enumerate() {
                //IF mouse is between old position and new position then we place cursor
                //behind the current character
                let adv = get_advance(it, _self.text_size);
                if i < _self.text_buffer.len() - 1{
                    if mouseinfo.x >= position + _self.x+2 && mouseinfo.x < position + adv + _self.x + 2 {
                        _self.text_cursor = i;
                        break;
                    }
                } else{
                    if mouseinfo.x >= position + _self.x+2 {
                        _self.text_cursor = i + 1;
                        break;
                    }
                }

                position += adv;
            }
        }


        if self.active == false {
            if in_rect(mouseinfo.x, mouseinfo.y,
               [self.x+4, self.y + 4, self.max_render_length , self.text_size as i32]) &&
               mouseinfo.lbutton == ButtonStatus::Down{
                self.active = true;

                placeCursor(self, mouseinfo);
            }
            return;
        }


        if  self.active {
            if in_rect(mouseinfo.x, mouseinfo.y,
                [self.x+4, self.y + 4, self.max_render_length , self.text_size as i32]) == false &&
                mouseinfo.lbutton == ButtonStatus::Down{
                self.active = false;
                return;
            } else { //IS THIS A GOOD ELSE STATEMENT I DON'T THINK THIS MAKES SENSE
                if in_rect(mouseinfo.x, mouseinfo.y,
                   [self.x+4, self.y + 4, self.max_render_length , self.text_size as i32]) &&
                   mouseinfo.lbutton == ButtonStatus::Down
                {//Look for where to place cursor
                    placeCursor(self, mouseinfo);

                }

                for i in 0..keyboardinfo.key.len(){
                    if  keyboardinfo.key[i] == KeyboardEnum::Enter &&
                       keyboardinfo.status[i] == ButtonStatus::Down {
                        self.active = false;
                        return;
                    }
                }
            }
        }

        for i in 0..keyboardinfo.key.len(){
            if keyboardinfo.status[i] == ButtonStatus::Down{
                if keyboardinfo.key[i] == KeyboardEnum::Leftarrow{
                    if self.text_cursor > 0 {
                        self.text_cursor -= 1;
                    }
                }
                if keyboardinfo.key[i] == KeyboardEnum::Rightarrow{
                    if (self.text_cursor as usize) < self.text_buffer.len() {
                        self.text_cursor += 1;
                    }
                }
                if keyboardinfo.key[i] == KeyboardEnum::Delete{
                    let _cursor = self.text_cursor;
                    if self.text_buffer.len() > _cursor {
                        self.text_buffer.remove(_cursor);
                    }
                }
            }
        }

        for character in &textinfo.character{
            let _cursor = self.text_cursor as usize;

            //NOTE character with u8 of 8 is the backspace code on windows
            let u8_char = *character as u8;
            if (u8_char == 8 )  && (self.text_buffer.len() > 0){
                self.text_buffer.remove(_cursor-1);
                self.text_cursor -= 1;
            } else if u8_char  >= 239 || u8_char == 127{
            //mac is 127 for delete
            } else {
                if self.text_buffer.len() < self.max_char as usize {
                    self.text_buffer.insert(_cursor, *character);
                    self.text_cursor += 1;
                }
            }
            if self.text_cursor as usize > self.text_buffer.len() {
                self.text_cursor = self.text_buffer.len();
            }
        }

    }
    pub fn draw(&self, canvas: &mut WindowCanvas, time: f32){
        draw_rect(canvas,
             [self.x+4, self.y + 4, self.max_render_length , self.text_size as i32],
             self.bg_color, true);
        draw_string(canvas, &self.text_buffer, self.x, self.y, self.text_color, self.text_size);


        if self.active {
            let mut adv = 0;
            let mut cursor_color = self.cursor_color;
            cursor_color[3] = cursor_color[3] * ( 0.5*(self.omega*time).cos() + 0.7).min(1.0);

            for (i, it) in self.text_buffer.chars().enumerate(){

                if i == self.text_cursor as usize {
                    draw_rect(canvas, [self.x+adv+4, self.y+4, 2, self.text_size as i32],
                         cursor_color, true);
                    break;
                }
                adv += get_advance(it, self.text_size);
            }
            if self.text_buffer.len() == 0 || self.text_cursor == self.text_buffer.len(){
                draw_rect(canvas, [self.x+adv+4, self.y+4, 2, self.text_size as i32],
                     cursor_color, true);
            }
        }
    }
}
/*
struct UiCanvas{
    elements: HashMap<TinyString, ENUM>,
    rect: [i32;4],
    block_w: i32,
    block_h: i32,
    fill_map: Vec<>,
    //blocks of unit size with array of bools that contain info on if block is filled or empty.
}
*/

struct Storage{
    init: bool,
    char_arr: [char; 100],
    cursor: usize,
    active: bool,
    f: f32
}
static mut storage : Storage = Storage{ init: false, char_arr: ['\0';100], f: 0f32, cursor: 0, active: false};
//TODO
pub fn ui_test(os_package: &mut OsPackage, keyboardinfo: &KeyboardInfo, textinfo: &TextInfo, mouseinfo: &MouseInfo)->i32{unsafe{
    if storage.init == false {
        storage.init = true;
        unsafe{ set_canvas(&mut crate::GLOBAL_BACKBUFFER); }
        change_font(&crate::FONT_NOTOSANS);
    }
    let w = os_package.window_canvas.w;
    let h = os_package.window_canvas.h;
    draw_rect(os_package.window_canvas, [0, 0, w, h], C4_BLACK, true);

    if gb_basic_button("Name me!", 500, 500, mouseinfo).lclicked {
        draw_rect(os_package.window_canvas, [50i32; 4], C4_WHITE, true);
        println!("ASFASDF");
    }

    if basic_button( os_package.window_canvas, "Thoth", 500, 250, 24f32, mouseinfo).in_rect{
        println!("QRQEWF");
    }

    storage.f = gb_horizslider(storage.f, [10, 50, 100, 15], mouseinfo).frac;
//TODO do textbox test... but I think we are good.

    return 0;
}}



