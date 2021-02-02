use crate::rendertools::*;
use crate::inputhandler::*;
use crate::WindowCanvas;
use crate::OsPackage;
use crate::FONT_NOTOSANS;
use crate::misc::*;

use std::io::prelude::*;
use std::ptr::{null, null_mut};

use std::f32::consts::PI;


use std::thread::sleep;
use std::time::{Instant, Duration};

use crate::lab_sims::*;

const GRID_SIZE: i32 = 20;
const MAX_VOLTAGE: f32 = 20.0;

const MENU_WIDTH : i32 = 100;
const MENU_HEIGHT: i32 = 100;

const EPSILON0 : f32 = 8.8541878128E-12;


//TODO
//+ allow voltage to be negative




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


#[derive(Copy, Clone, Debug, PartialEq)]
enum EqSourceTypes{
    None = 0,
    Disk = 1,
    VertBar = 2,
    HorzBar = 3,
}

struct ActivationPoint{
    _type: EqSourceTypes,
    x: i32,
    y: i32,
    voltage: f32,

    menu_on: bool,
    voltage_selected: bool,
    slider: HorizSlider,

}
impl ActivationPoint{
    fn new()->ActivationPoint{
        ActivationPoint{
            _type: EqSourceTypes::None,
            x: 0i32,
            y: 0i32,
            voltage: 0f32,
  
            menu_on: false,
            voltage_selected: false,
            slider: HorizSlider::new(),
        }
    }
}


struct Point{
    x: i32,
    y: i32,
}
pub struct EQ_Storage{
    init: bool,
    activation_points: Vec<ActivationPoint>,
    arr_panels: Vec<Panel>,
    things_to_draw: Vec<Vec<Point>>,
    are_we_drawing: bool,
}

impl EQ_Storage{
    pub fn new()->EQ_Storage{
        EQ_Storage{
            init: false,
            activation_points: Vec::new(),
            arr_panels: Vec::new(),
            things_to_draw: Vec::new(),
            are_we_drawing: false,
        }
    }
}



struct Particle{
    charge: f32,
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct ElectricField{
    x: f32,
    y: f32,
}


#[inline]
fn calc_potential( p1 : &Particle, x: f32, y: f32)->f32{
    return 1.0/(4.0*PI*EPSILON0) * p1.charge / ( (p1.x - x).powi(2) + (p1.y - y).powi(2) ).sqrt();
}

#[inline]
fn calc_electric_field( p1 : &Particle, x: f32, y: f32)->ElectricField{

    let _x = p1.x - x;
    let _y = p1.y - y;
    let _h = (_x.powi(2) + _y.powi(2)).sqrt();


    let rt = 1.0/(4.0*PI*EPSILON0) * p1.charge /  ((p1.x - x).powi(2) + (p1.y - y).powi(2));
    return ElectricField{ 
                x: rt * _x/_h,
                y: rt * _y/_h,
            };
}

#[inline]
fn add_electric_field(e1: &ElectricField, e2: &ElectricField)->ElectricField{
    ElectricField{ x: e1.x + e2.x, y: e1.y + e2.y}
}

#[inline]
fn calc_radius(x1: f32, y1: f32, x2: f32, y2: f32, )->f32{
    return ((x1 - x2).powi(2) + (y1- y2).powi(2)).sqrt();
}

pub fn eq_potential_sim(os_package: &mut OsPackage, app_storage: &mut LS_AppStorage, keyboardinfo: &KeyboardInfo, textinfo: &TextInfo, mouseinfo: &MouseInfo)->i32{
    let possible_points = [(200, 250), (800, 250), (500, 250)];//TODO should use window dimensions maybe?

    if !app_storage.eq_storage.init{
        app_storage.eq_storage.init = true;

        for it in possible_points.iter(){
            let mut s = ActivationPoint::new();
            s.x = it.0;
            s.y = it.1;

            s.slider.width = MENU_HEIGHT-10;
            s.slider.height = 2;
            s.slider.x = s.x + 5;
            s.slider.y = s.y + 45;
            s.slider.button_w = 10;
            s.slider.button_h = 20;

            app_storage.eq_storage.activation_points.push( s );
        }
    }


    let window_w = os_package.window_canvas.w;
    let window_h = os_package.window_canvas.h;


    draw_rect(os_package.window_canvas, [0, 0, window_w, window_h], COLOR_BKG, true);
    draw_grid(os_package.window_canvas, GRID_SIZE);



    for it in app_storage.eq_storage.things_to_draw.iter(){
        for j in 0..it.len()-1{//TODO We should interp between points
            let jt = &it[j];
            let jt_p1 = &it[j+1];
            
            draw_line(os_package.window_canvas, jt.x, jt.y, jt_p1.x, jt_p1.y, 5, 5, C4_WHITE);
        }
    }
    //TODO this should be done some where else
    if mouseinfo.lbutton == ButtonStatus::Down 
    && app_storage.eq_storage.are_we_drawing == false{
        app_storage.eq_storage.are_we_drawing = true;
        app_storage.eq_storage.things_to_draw.push(Vec::new());
    } else if mouseinfo.lbutton == ButtonStatus::Up{
        app_storage.eq_storage.are_we_drawing = false;
    }

    if app_storage.eq_storage.are_we_drawing {
        let i = app_storage.eq_storage.things_to_draw.len() - 1;
        app_storage.eq_storage.things_to_draw[i].push(Point{ x: mouseinfo.x, y: mouseinfo.y});
    }
    for (i, it) in keyboardinfo.key.iter().enumerate() {
        if *it == KeyboardEnum::U
        && keyboardinfo.status[i] == ButtonStatus::Down {

            app_storage.eq_storage.things_to_draw.pop();
        }
    }


    draw_string(&mut os_package.window_canvas, "Equipotential Simulation", window_w/2-170, window_h-60, C4_WHITE, 46.0);
    
    let mut particles = vec![]; //TODO this should be a pre alloc buffer
   



    

    let mut mouse_in_charged_volume = false; 
    let mut mouse_in_volume_menu = false; 

    let mut mouse_input_ate = false;
    for it in app_storage.eq_storage.activation_points.iter_mut(){
        let menu_rect = [it.x, it.y, MENU_WIDTH, MENU_HEIGHT];
        match it._type{
            EqSourceTypes::None=>{
                if calc_radius( mouseinfo.x as f32, mouseinfo.y as f32, it.x as f32, it.y as f32) < 10.0{
                    draw_circle(os_package.window_canvas, it.x, it.y, 10.0, C4_WHITE);
                    if mouseinfo.lclicked() && (!in_rect(mouseinfo.x, mouseinfo.y, menu_rect) || it.menu_on == false){
                        it.menu_on = !it.menu_on;
                    }
                } else {
                    draw_circle(os_package.window_canvas, it.x, it.y, 10.0, C4_LGREY);
                }
            },
            EqSourceTypes::Disk=>{
                draw_circle(os_package.window_canvas, it.x, it.y, 50.0, C4_DGREY);
                particles.push(Particle{charge: it.voltage*(50.0*EPSILON0*PI*4.0), x: it.x as f32, y: it.y as f32});

                if calc_radius( mouseinfo.x as f32, mouseinfo.y as f32, it.x as f32, it.y as f32) < 50.0{
                    
                    if (!in_rect(mouseinfo.x, mouseinfo.y, menu_rect) || it.menu_on == false){
                        mouse_in_volume_menu = true;
                        if mouseinfo.lclicked(){
                            it.menu_on = !it.menu_on;
                        }
                    }
                }
            },
            EqSourceTypes::VertBar=>{
                let w = 50;
                let h = 350;
                draw_rect(os_package.window_canvas, [it.x - w/2, it.y - h/2, w, h], C4_DGREY, true);
                let charge = it.voltage*(EPSILON0*PI*4.0)/4.0; //TODO kinda trash
                for i in 0..100{
                  let x = (it.x + w/3) as f32;
                  let y = (it.y - h/2) as f32 + h as f32 * i as f32 / 100.0;

                  let mut _charge = charge;
                  if i == 0 || i == 99 {
                      _charge = charge * 10.0;
                  }
                  let p = Particle{charge: _charge, x: x, y: y}; //TODO
                  particles.push( p );
                }
                for i in 0..100{
                  let x = (it.x - w/3) as f32 ;
                  let y = (it.y - h/2) as f32 + h as f32 * i as f32 / 100.0;

                  let mut _charge = charge;
                  if i == 0 || i == 99 {
                      _charge = charge * 10.0;
                  }
                  let p = Particle{charge: _charge, x: x, y: y}; //TODO
                  particles.push( p );
                }

                if in_rect(mouseinfo.x, mouseinfo.y, [it.x - w/2, it.y - h/2, w, h]){

                    if (!in_rect(mouseinfo.x, mouseinfo.y, menu_rect) || it.menu_on == false){
                        mouse_in_volume_menu = true;
                        if mouseinfo.lclicked(){
                            it.menu_on = !it.menu_on;
                        }
                    }
                }
            },
            EqSourceTypes::HorzBar=>{
                let w = 150;
                let h = 50;
                let charge = it.voltage*(EPSILON0*PI*4.0)/4.0; //TODO kinda trash

                draw_rect(os_package.window_canvas, [it.x - w/2, it.y - h/2, w, h], C4_DGREY, true);
                for i in 0..100{
                  let x = (it.x - w/2) as f32 + w as f32 * i as f32 / 100.0;
                  let y = (it.y - h/2) as f32;
                  let p = Particle{charge: charge, x: x, y: y};
                  particles.push( p );
                }

                if in_rect(mouseinfo.x, mouseinfo.y, [it.x - w/2, it.y - h/2, w, h]){
                    if (!in_rect(mouseinfo.x, mouseinfo.y, menu_rect) || it.menu_on == false){
                        mouse_in_volume_menu = true;
                        if mouseinfo.lclicked(){
                            it.menu_on = !it.menu_on;
                        }
                    }
                }
            },
            _=>{}
        }

        if in_rect( mouseinfo.x, mouseinfo.y, menu_rect) 
        && it.menu_on{
            mouse_in_volume_menu = true;
        }

        if it.menu_on
        && !mouse_input_ate {

            //TODO make pretty
            draw_rect(os_package.window_canvas, menu_rect, C4_BLUE, true);

            if it._type != EqSourceTypes::None{
                draw_string(&mut os_package.window_canvas, &format!("Voltage: {:.2}", it.voltage), it.x, it.y+50, C4_WHITE, 25.0);

                it.slider.percentage = it.voltage / MAX_VOLTAGE + 0.5;

                it.slider.update(mouseinfo);
                it.voltage = (it.slider.percentage - 0.5) * MAX_VOLTAGE;

                it.slider.draw(&mut os_package.window_canvas, C4_WHITE, C4_WHITE);
            }

            draw_string(&mut os_package.window_canvas, "Next Type", it.x, it.y, C4_WHITE, 25.0);

            if in_rect(mouseinfo.x, mouseinfo.y, [menu_rect[0], menu_rect[1], menu_rect[2], 30]) 
            && mouseinfo.lclicked() {
                mouse_input_ate = true;

                //TODO could have hanlded this more elegantly. :(
                let temp = it._type as i32;
                if temp == EqSourceTypes::None as i32 {
                    it._type = EqSourceTypes::Disk;

                } else if temp == EqSourceTypes::Disk as i32 {
                    it._type = EqSourceTypes::VertBar;

                } else if temp == EqSourceTypes::VertBar as i32 {
                    it._type = EqSourceTypes::HorzBar;

                } else {
                    it._type = EqSourceTypes::None;

                } 
            }
        }
    }




    let mut potiential = 0.0;
    let mut efield = ElectricField{x: 0.0, y: 0.0};

    for it in particles.iter(){
        potiential += calc_potential(it, mouseinfo.x as f32, mouseinfo.y as f32);
        let temp_efield = calc_electric_field(it, mouseinfo.x as f32, mouseinfo.y as f32);
        efield = add_electric_field(&efield, &temp_efield);
    }

    if !mouse_in_charged_volume 
    && !mouse_in_volume_menu {
        draw_rect(os_package.window_canvas, [mouseinfo.x, mouseinfo.y, 400, 50], C4_BLACK, true);
        draw_string(os_package.window_canvas, &format!("V: {:.2}", potiential), mouseinfo.x, mouseinfo.y, C4_GREY, 24.0);

        let mut efield_string = "Electric Field:".to_string();
        efield_string += &format!(" x {:.2},  y {:.2}", efield.x, efield.y);

        draw_string(os_package.window_canvas, &efield_string, mouseinfo.x, mouseinfo.y+25, C4_GREY, 24.0);
    }





    //TODO 
    //{//TESTING
    //    println!("");
    //    draw_line(&mut os_package.window_canvas, );
    //    draw_line(&mut os_package.window_canvas,);
    //    draw_line(&mut os_package.window_canvas,);
    //    draw_line(&mut os_package.window_canvas,);
    //}






    let screenshot_rect = [ window_w - 60, window_h - 25, 20, 20];
    draw_bmp(&mut os_package.window_canvas, &app_storage.screenshot_icon_bmp, screenshot_rect[0], screenshot_rect[1], 0.98, Some(20), Some(20));
    

    if mouseinfo.lclicked()
    && in_rect(mouseinfo.x, mouseinfo.y, screenshot_rect) { //TODO 
    //if app_storage.menu_offscreen{
        let mut bmp = TGBitmap::new(os_package.window_canvas.w, os_package.window_canvas.h);


        bmp.file_header.off_bits = 54; //TODO
        bmp.file_header.size_ = (4*bmp.width * bmp.height) as u32 + bmp.file_header.off_bits;
        bmp.info_header.header_size = 40;
        bmp.info_header.compression = 0;
        bmp.info_header.image_size = (4*bmp.width * bmp.height) as u32;
        bmp.info_header.x_px_per_meter = 1;
        bmp.info_header.y_px_per_meter = 1;

        unsafe{
            let buffer = os_package.window_canvas.buffer as *const u8;
            for i in 0..(bmp.width*bmp.height) as usize{
                bmp.rgba[4*i + 0] = *buffer.offset(4*i  as isize + 0);
                bmp.rgba[4*i + 1] = *buffer.offset(4*i  as isize + 1);
                bmp.rgba[4*i + 2] = *buffer.offset(4*i  as isize + 2);
                bmp.rgba[4*i + 3] = *buffer.offset(4*i  as isize + 3);
            }
        }


        bmp.save_bmp("screenshot.bmp");//TODO search for conflicts
    }




    let sub_w = app_storage.menu_canvas.canvas.w;
    let sub_h = app_storage.menu_canvas.canvas.h;
    draw_rect(&mut app_storage.menu_canvas.canvas, [0, 0, sub_w, sub_h], COLOR_MENU_BKG, true);

    {//Moving subcanvas
        let mut x = window_w/2;
        for it in keyboardinfo.key.iter(){
            if *it == KeyboardEnum::A && app_storage.menu_move_activated_time == 0{ //TODO

                app_storage.menu_move_activated = true;
                app_storage.menu_move_activated_time =  app_storage.timer.elapsed().as_micros();
            }
        }

        let rect_main = [window_w-25, window_h-26, 20, 20];
        let rect_sub_  = [window_w/2+5, window_h-25, 20, 20]; //TODO be should to include subcanvas offset
        let rect_sub  = [5, sub_h-25, 20, 20]; //TODO be should to include subcanvas offset

        //draw_rect(&mut os_package.window_canvas, rect_main, C4_WHITE, true);
        //draw_rect(&mut app_storage.menu_canvas.canvas, rect_sub, C4_WHITE, true);

        let mut color = C4_WHITE;
        if app_storage.menu_offscreen {
            if in_rect(mouseinfo.x, mouseinfo.y, rect_main) {
                color = C4_LGREY;
                if mouseinfo.lbutton == ButtonStatus::Up && mouseinfo.old_lbutton == ButtonStatus::Down{

                    app_storage.menu_move_activated = true;
                    app_storage.menu_move_activated_time =  app_storage.timer.elapsed().as_micros();

                }
            }
        } else {
            if in_rect(mouseinfo.x, mouseinfo.y, rect_sub_) {  //TODO if subcanvas is on screen
                color = C4_LGREY;
                if mouseinfo.lbutton == ButtonStatus::Up && mouseinfo.old_lbutton == ButtonStatus::Down{

                    app_storage.menu_move_activated = true;
                    app_storage.menu_move_activated_time =  app_storage.timer.elapsed().as_micros();

                }
            }
        }

        for i in 1..4{
            let y = (5.0 * i as f32) as i32 + rect_main[1];
            let _rect = [rect_main[0], y, rect_main[2], 2];
            draw_rect(&mut os_package.window_canvas, _rect, color, true);
        }

        for i in 1..4{
            let y = (5.0 * i as f32) as i32 + rect_sub[1];
            let _rect = [rect_sub[0], y, rect_sub[2], 2];
            draw_rect(&mut app_storage.menu_canvas.canvas, _rect, color, true);
        }


        let delta_max_time = 0.2E6;
        if app_storage.menu_move_activated {

            let mut delta = (app_storage.timer.elapsed().as_micros() - app_storage.menu_move_activated_time) as f32;
            x = ((window_w/2) as f32 * ((delta_max_time + delta) / delta_max_time)) as i32; //TODO
            if app_storage.menu_offscreen {
                x = ((window_w as f32 - (delta/delta_max_time) * (window_w as f32 / 2.0)) as i32).max(window_w/2); //TODO
            }

            if delta.abs() > delta_max_time{
                app_storage.menu_move_activated = false;
                app_storage.menu_move_activated_time = 0;
                app_storage.menu_offscreen = !app_storage.menu_offscreen;  //TODO this is kinda trash will need to rewrite

            }
        }

        if app_storage.menu_offscreen && app_storage.menu_move_activated == false {
            x = window_w;
        }


        draw_subcanvas(&mut os_package.window_canvas, &app_storage.menu_canvas, x, 0, 1.0);
    }


    return 0;
}




fn draw_line(window_canvas: &mut WindowCanvas, x1: i32, y1: i32, x2: i32, y2: i32, w: i32, h: i32, color: [f32; 4]){
    let _x1 = x1.min(x2);
    let _x2 = x1.max(x2);
    let mut _y1 = 0;
    let mut _y2 = 0;

    if _x1 == x1 {
        _y1 = y1;
        _y2 = y2;
    } else {
        _y1 = y2;
        _y2 = y1;
    }


    let m =(_y2 - _y1) as f32 / ( _x2 - _x1) as f32 ;
    let c = _y1 as f32 - m * _x1 as f32;
    for i in _x1.._x2{
        let rect = [ i , (m*i as f32 + c).round() as i32, w, h];
        draw_rect(window_canvas, rect, color, true);
    } 

    {//TODO this is trash  we draw the same pixels multiple times
        let c = _x1 as f32 - 1.0/m * _y1 as f32;
        if _y1 < _y2 {
            for i in _y1.._y2{
                let rect = [ (1.0/m*i as f32 + c).round() as i32, i, w, h];
                draw_rect(window_canvas, rect, color, true);
            } 
        } else {
            for i in _y2.._y1{
                let rect = [ (1.0/m*i as f32 + c).round() as i32, i, w, h];
                draw_rect(window_canvas, rect, color, true);
            } 
        }
    }
}


