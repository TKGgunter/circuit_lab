//! This module contains a set of performance debugging tools, akin to telemetry. 
//!
//! To assist in the optimization of the program the following tools are provided.
//! The macros `timeit!` and `DEBUG_timeit!` should be used when timing a block of code.
//! `timeit!` is a macro that will print the elapsed time and cycle counts at the end of the code
//! block.
//! `DEBUG_timeit!` is a macro that should be used in conjunction with a statically initialized
//! `DebugStruct`, `reset_frame_debugging` and `draw_debuginfo`.  `draw_debuginfo` will render debug data to the given
//! canvas. `DebugStruct` collects timings across a specified frame such that `draw_debuginfo` can deliver averaged statistics.
//!
//! When utilizing the module past the following to the file of interest.
//! ```
//!#[macro_use]
//!use crate::{timeit, DEBUG_timeit};
//!use crate::debug_tools::*;
//! ```



use std::collections::HashMap;
use std::time::{Instant, Duration};

use crate::misc::StopWatch;
use crate::WindowCanvas;
use crate::inputhandler::*;
use crate::rendertools::*;



pub const MAX_AVG_N : usize = 3;

#[derive(Copy, Clone)]
pub struct CountsAndDuration{
    pub count: usize,
    pub duration: Duration,
}

pub struct DebugStruct{
    pub instant: StopWatch,
    pub count: usize,
    pub count_per_frame: usize,
    pub cpu_counts: u64,
    pub durations: Duration,
    pub durations_per_frame: [CountsAndDuration; MAX_AVG_N],
    
    pub _cycles: u64,
}
impl DebugStruct{
    pub fn new()->DebugStruct{
        DebugStruct{
            instant: StopWatch::new(), //replace with stopwatch
            count: 0,
            count_per_frame: 0,
            cpu_counts: 0,
            durations: Duration::new(0,0),
            durations_per_frame: [CountsAndDuration{ count: 0, duration: Duration::new(0,0) }; MAX_AVG_N],

            _cycles: 0
        }
    }
}

#[cfg(target_arch = "x86")]
pub fn get_clock_cycle()->u64{unsafe{
    let mut rt = 0;
    rt = core::arch::x86::_rdtsc();
    return rt;
}}


#[cfg(target_arch = "x86_64")]
pub fn get_clock_cycle()->u64{unsafe{
    let mut rt = 0;
    rt = core::arch::x86_64::_rdtsc();
    return rt;
}}

pub struct DebugRenderStruct{
    bkg_color : [f32; 4],
    font_color: [f32; 4],
    font_size: f32,
    x: i32, 
    y: i32,
    width : i32,
    height: i32,
}

pub static mut GLOBAL_DEBUG_TIMEIT : Option<HashMap<String, DebugStruct>> = None;
pub static mut GLOBAL_DEBUG_COUNT : usize = 0;
pub static mut GLOBAL_DEBUG_RENDER : DebugRenderStruct = DebugRenderStruct{ bkg_color: [0f32, 0f32, 0f32, 0.5],
                                                                        font_color: [1f32, 1f32, 1f32, 1f32],
                                                                        font_size: 22f32,
                                                                        x: 0, y: 0, width: 100, height: 100};



/// This function initializes the debug render.
///
/// If this function is set with `None` the render will be set with defaults.
pub fn init_debugging( rect: Option<[i32; 4]>){unsafe{
    GLOBAL_DEBUG_TIMEIT = Some(HashMap::new());

    match rect {
        Some(_rect)=>{
            let [x, y, w, h] = _rect;
            GLOBAL_DEBUG_RENDER.x = x;
            GLOBAL_DEBUG_RENDER.y = y;
            GLOBAL_DEBUG_RENDER.width = w;
            GLOBAL_DEBUG_RENDER.height = h;
        },
        None=>{}
    }

}}

/// This function resets the timing and counting information carried by a global struct.
///
/// This function should be set at the frame end, the end of `circuit_sim` as to accurately
/// calculate count and timing averages.
pub fn reset_frame_debugging(){unsafe{
    GLOBAL_DEBUG_COUNT += 1;

    let db = GLOBAL_DEBUG_TIMEIT.as_mut().expect("GLOBAL_DEBUG not init.");
    for (k, v) in db.iter_mut(){
        v.count_per_frame = 0;
        v.durations_per_frame[GLOBAL_DEBUG_COUNT % MAX_AVG_N].duration = Duration::new(0, 0);
        v.durations_per_frame[GLOBAL_DEBUG_COUNT % MAX_AVG_N].count = 0;
    }
}}

/// This macro will print elapsed time and clock cycles over the course of a code block.
///
/// ## Example
/// ```
/// timeit!{{
///     //Do some work
/// }}
///
/// timeit!{"name of block", {
///     //Do some work
///     //The name of timeit block will be printed before results.
/// }}
/// ```
#[macro_export]
macro_rules! timeit{
    ($x:block) => 
    { 
        let DEBUG_time = std::time::Instant::now(); 
        let DEBUG_cycles = get_clock_cycle();
        $x;
        let DEBUG_cycles = get_clock_cycle() - DEBUG_cycles;
        let DEBUG_duration = DEBUG_time.elapsed();
        println!("Debug timeit : {:?} {}", DEBUG_duration, DEBUG_cycles);
    };
    ($s:expr , $x:block)=>{
        let DEBUG_time = std::time::Instant::now(); 
        let DEBUG_cycles = get_clock_cycle();
        $x;
        let DEBUG_cycles = get_clock_cycle() - DEBUG_cycles;
        let DEBUG_duration = DEBUG_time.elapsed();
        println!("Debug timeit | {} : {:?} {}", $s, DEBUG_duration, DEBUG_cycles);
    };
}

/// This macro will store elapsed time and clock cycles over the course of a code block.
/// 
/// ## Example
/// ```
///
/// DEBUG_timeit!{"name of block", {
///     //Do some work
///     //The name of timeit block will be set for results.
/// }}
/// ```
/// 
#[macro_export]
macro_rules! DEBUG_timeit{
    ($x:tt, $y:block)=>
    {unsafe{
        match GLOBAL_DEBUG_TIMEIT.as_mut(){
            Some(DEBUG_debug)=>
            {
                if DEBUG_debug.contains_key(&$x.to_string()){
                } else {
                    DEBUG_debug.insert($x.to_string(), 
                                 DebugStruct::new());//TODO should only be new it key does not exist 
                }
                let DEBUG_struct = DEBUG_debug.get_mut(&$x.to_string()).unwrap();
                DEBUG_struct.instant.reset();

                $y;

                DEBUG_struct.durations += DEBUG_struct.instant.get_time();
                DEBUG_struct.count += 1;

                DEBUG_struct.durations_per_frame[GLOBAL_DEBUG_COUNT % MAX_AVG_N].duration += DEBUG_struct.instant.get_time();
                DEBUG_struct.durations_per_frame[GLOBAL_DEBUG_COUNT % MAX_AVG_N].count += 1;
                DEBUG_struct.count_per_frame += 1;

            },
            None=>{}
        }
    }}
}



//pub fn update_debuginfo(keyboardinfo: &KeyboardInfo, textinfo: &TextInfo, mouseinfo: &MouseInfo){
//    //TODO 
//    //some form of interaction maybe
//}

/// Draws the debug info to the canvas provided. The results will be rendered across the entire
/// canvas.
pub fn draw_debuginfo(canvas: &mut WindowCanvas){unsafe{

    let _x = GLOBAL_DEBUG_RENDER.x;
    let _y = GLOBAL_DEBUG_RENDER.y;
    let w = GLOBAL_DEBUG_RENDER.width;
    let h = GLOBAL_DEBUG_RENDER.height;

    draw_rect(canvas, [_x, _y, w, h],  GLOBAL_DEBUG_RENDER.bkg_color, true);

    let mut x = _x + 0;
    let mut y = _y + h;

    y -= GLOBAL_DEBUG_RENDER.font_size as i32;//TODO this is trashy
    draw_string(canvas, "DEBUG:", _x, y, GLOBAL_DEBUG_RENDER.font_color, GLOBAL_DEBUG_RENDER.font_size);

    y -= 2*GLOBAL_DEBUG_RENDER.font_size as i32;//TODO this is trashy
    draw_string(canvas, "   tag       |  counts   |  counts_per_frame  |  avg_duration  |  tot_dur_frame", _x, y, GLOBAL_DEBUG_RENDER.font_color, GLOBAL_DEBUG_RENDER.font_size);
    y -= GLOBAL_DEBUG_RENDER.font_size as i32;//TODO this is trashy

    match GLOBAL_DEBUG_TIMEIT.as_mut(){
        Some(db)=>{
            for (k, v) in db.iter(){
                let mut _k = k.clone();
                _k.truncate(10);
                draw_string(canvas, &format!("{}", _k), x, y, GLOBAL_DEBUG_RENDER.font_color, GLOBAL_DEBUG_RENDER.font_size); //TAG
                draw_string(canvas, &format!("{:8}", v.count), x+75, y, GLOBAL_DEBUG_RENDER.font_color, GLOBAL_DEBUG_RENDER.font_size);//TOTAL Counts
                draw_string(canvas, &format!("{:>8}", v.count_per_frame), x+200, y, GLOBAL_DEBUG_RENDER.font_color, GLOBAL_DEBUG_RENDER.font_size);
                draw_string(canvas, &format!("{:>8.2?}", v.durations/v.count as u32), x+320, y, GLOBAL_DEBUG_RENDER.font_color, GLOBAL_DEBUG_RENDER.font_size);
                draw_string(canvas, &format!("{:>8.2?}", v.durations/v.count as u32 * v.count_per_frame as u32), x+420, y, GLOBAL_DEBUG_RENDER.font_color, GLOBAL_DEBUG_RENDER.font_size);
                y -= GLOBAL_DEBUG_RENDER.font_size as i32;
                if y < GLOBAL_DEBUG_RENDER.font_size as i32 { break; }
            }
        },
        None=>{
            draw_string(canvas, "Debug struct has not been init.", _x+10, _y, C4_RED, GLOBAL_DEBUG_RENDER.font_size);
        }
    }
}}



#[test]
fn debugging_test1(){
    fn add(x: f32, y: f32)->f32{ return x + y ; }
    let x = 10.0;
    timeit!{{
    add(x, 12.0);
    }}; 
}

#[test]
fn debugging_test2(){
    fn add(x: f32, y: f32)->f32{ return x + y ; }
    let x = 10.0;
    timeit!{{
    let a = add(x, 12.0);
    //let b = add(x, 12.0);
    }}; 
}


#[test]
fn debugging_test3(){
    fn add(x: f32, y: f32)->f32{ return x + y ; }
    init_debugging(None);


    let x = 10;
    DEBUG_timeit!( "debug testing",{
    let y = add(x as f32, 12.0);
    match x {
        10 => { let y = add(10.0, 11.0); },
        _=> {  }
    }
    }); 

    unsafe{
    println!("{:?}", GLOBAL_DEBUG_TIMEIT.as_mut().unwrap().len());
    }
}




