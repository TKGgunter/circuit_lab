
//! This module contains the set of software rendering tools used for this application.
//! Text rendering, simple shape, and bitmap rendering is provided here.
//!
//! This module contains functions that draws directly to the provided canvas.
//! All draw calls are done directly using the cpu.  If you wish to use the gpu for rendering
//! you will need to set it up yourself then paint it to the canvas.
//!
//! 
//!
//! The interface is pixel based, with the origin at bottom left corner of the canvas.
//! The example below shows how to draw a rectangle to WindowCanvas provided outside of this snips
//! context. `C4_BLACK` is a const provided by the module for convenience. 
//! This example will draw a black rectangle, who's bottom left corner will inhabit canvas coordinate of
//! (10, 10). The rectangle will be 50 pixels by 50 pixels.
//! # Examples
//! ```
//! pub fn example_program(os_package: &mut OsPackage, keyboardinfo: &KeyboardInfo, 
//!                                                    textinfo:     &TextInfo, 
//!                                                    mouseinfo:    &MouseInfo){
//!     let canvas = &mut os_package.window_canvas;
//!     let rect = [10, 10, 50, 50];
//!     draw_rect(canvas, rect, C4_BLACK, true);
//! }
//! ```
//! 
//! Images can be drawn in a similar fashion.  `draw_bmp`, or `draw_stbi_image` are given for this
//! purpose.  `draw_bmp` is used when working with TGBitmap structs.  TGBitmaps are this frame
//! works bitmap structure.  It must be noted that not all bitmap type are supported in with this
//! format. `draw_stbi_image` should be used when working with most files as most files are
//! supported through the stbi_image library. 
//!  
//!

#![allow(unused)]

#[macro_use]
use crate::{timeit, DEBUG_timeit};
use crate::debug_tools::*;

use crate::WindowCanvas;
use crate::{null, null_mut};
use crate::stb_tt_sys::*;
pub use crate::stb_image_sys::*;


use std::fs::File;
use std::io::prelude::*;



pub const C4_WHITE  :[f32;4] = [1.0, 1.0, 1.0, 1.0];
pub const C4_BLACK  :[f32;4] = [0.0, 0.0, 0.0, 1.0];

pub const C4_GREY   :[f32;4] = [0.5, 0.5, 0.5, 1.0];
pub const C4_LGREY  :[f32;4] = [0.8, 0.8, 0.8, 1.0];
pub const C4_DGREY  :[f32;4] = [0.2, 0.2, 0.2, 1.0];

pub const C4_BLUE   :[f32;4] = [0.0, 0.0, 1.0, 1.0];
pub const C4_RED    :[f32;4] = [1.0, 0.0, 0.0, 1.0];
pub const C4_GREEN  :[f32;4] = [0.0, 1.0, 0.0, 1.0];

pub const C4_YELLOW  :[f32;4] = [1.0, 1.0, 0.0, 1.0];
pub const C4_MAGENTA :[f32;4] = [1.0, 0.0, 1.0, 1.0];
pub const C4_CYAN    :[f32;4] = [0.0, 1.0, 1.0, 1.0];


pub const C4_PURPLE :[f32;4] = [0.5, 0.0, 0.5, 1.0];
pub const C4_CREAM  :[f32;4] = [1.0, 0.99, 0.82, 1.0];
pub const C4_DGREEN  :[f32;4] = [0.0, 0.39, 0.0, 1.0];
pub const C4_MGREEN  :[f32;4] = [0.0, 0.5, 0.0, 1.0];

pub const C3_WHITE  :[f32;3] = [1.0, 1.0, 1.0];
pub const C3_BLACK  :[f32;3] = [0.0, 0.0, 0.0];

pub const C3_GREY   :[f32;3] = [0.5, 0.5, 0.5];
pub const C3_LGREY  :[f32;3] = [0.8, 0.8, 0.8];
pub const C3_DGREY  :[f32;3] = [0.2, 0.2, 0.2];

pub const C3_BLUE   :[f32;3] = [0.0, 0.0, 1.0];
pub const C3_RED    :[f32;3] = [1.0, 0.0, 0.0];
pub const C3_GREEN  :[f32;3] = [0.0, 1.0, 0.0];

pub const C3_YELLOW :[f32;3] = [1.0, 1.0, 0.0];
pub const C3_PURPLE :[f32;3] = [1.0, 0.0, 1.0];
pub const C3_CYAN   :[f32;3] = [0.0, 1.0, 1.0];
pub const C3_CREAM  :[f32;3] = [1.0, 0.99, 0.82];
pub const C3_DGREEN :[f32;3] = [0.0, 0.39, 0.0];
pub const C3_MGREEN :[f32;3] = [0.0, 0.6, 0.0];


pub const DPMM_SCALE : f32 = 3.8;
pub const DPMM_TOLERANCE : f32 = 0.2;


use std::collections::HashMap;
static mut GLOBAL_FONTINFO    : stbtt_fontinfo  = new_stbtt_fontinfo();
static mut FONT_BUFFER        : Option<Vec<u8>> = Some(Vec::new());
static mut FONT_GLYPH_HASHMAP : Option<HashMap<usize, HashMap<(char, u32), Vec<u8>>>> = None;
static mut CURRENT_KEY        : usize = 0;


///Returns an array of length 4 that is the composite of the input array and alpha.
///
/// # Example
/// ```
/// let v = [1f32, 0.5f32, 0.1f32];
/// let a = 0.5f32;
///
/// let v_a = [1f32, 0.5f32, 0.1f32, 0.5f32];
/// assert_eq!(c3_to_c4(v, a), v_a);
/// ```
#[inline]
pub fn c3_to_c4(c3: [f32; 3], alpha: f32)->[f32; 4]{
    [c3[0], c3[1], c3[2], alpha]
}


/// Updates the static font buffer using the buffer provided.
/// Returns a result indicating if the function succeeded. 
/// Note: This is not thread safe. Additionally, if the user 
/// constantly un(re)loading the same font files the user will
/// incur performance penalties. 
pub fn change_font(buffer: &[u8])->Result<(), &str>{unsafe{

    let font_glypth_hashmap = match FONT_GLYPH_HASHMAP.as_mut(){
        Some(fgh)=>{
            fgh
        },
        None=>{
            FONT_GLYPH_HASHMAP = Some( HashMap::with_capacity(10) );
            FONT_GLYPH_HASHMAP.as_mut().unwrap()
        },
    };


    let key = buffer.as_ptr() as usize;
    CURRENT_KEY = key; 
    if !font_glypth_hashmap.contains_key( &key ){
        font_glypth_hashmap.insert( key, HashMap::with_capacity(100) ); 
    }
    //NOTE TKG. We are using the original buffer pointer to hash. If the user continuously reloads from
    //disk this will will become a problem.



    let font_buffer_ref = match FONT_BUFFER.as_mut(){
        Some(fb)=>{
            fb.clear();
            fb
        },
        None=>{
            FONT_BUFFER = Some(Vec::with_capacity(buffer.len()));
            FONT_BUFFER.as_mut().unwrap()
        }
    };

    font_buffer_ref.extend_from_slice(buffer);

    if stbtt_InitFont(&mut GLOBAL_FONTINFO as *mut stbtt_fontinfo, font_buffer_ref.as_ptr(), 0) == 0{
        println!("font was not able to load.");
        return Err("Font was not able to be loaded.");
    }
    return Ok(());
}}



/// Returns the pixel width of the character.
pub fn get_advance(character: char, size: f32)->i32{unsafe{
    if GLOBAL_FONTINFO.data == null_mut() {
        println!("Global font has not been set.");
        return -1;
    }
    let mut adv = 0;
    let scale = stbtt_ScaleForPixelHeight(&GLOBAL_FONTINFO as *const stbtt_fontinfo, size);
    let glyph_index = stbtt_FindGlyphIndex(&GLOBAL_FONTINFO as *const stbtt_fontinfo, character as i32);
    

    stbtt_GetGlyphHMetrics(&GLOBAL_FONTINFO as *const stbtt_fontinfo, glyph_index, &mut adv as *mut i32, null_mut());
    return (adv as f32 * scale) as i32;
}}




/// Returns the pixel width of the provided string.
pub fn get_advance_string( string: &str, size: f32 )->i32{
    let mut offset = 0;
    for it in string.chars(){
        offset += get_advance(it, size);
    }
    return offset;
}




/// Draws the provided character to the canvas. `size` is rounded to the nearest integer. 
/// Returns character width in pixels.
pub fn draw_char( canvas: &mut WindowCanvas, character: char, mut x: i32, mut y: i32,
             color: [f32; 4], mut size: f32 )->i32{unsafe{

    //NOTE Check that globalfontinfo has been set
    if GLOBAL_FONTINFO.data == null_mut() {
        println!("Global font has not been set.");
        return -1;
    }

    let canvas_dpmm = if canvas.dpmm == 0f32 { DPMM_SCALE } else { canvas.dpmm };

    let mut dpmm_ratio = 1f32;//canvas_dpmm / DPMM_SCALE;
    if (1f32 - dpmm_ratio).abs() < DPMM_TOLERANCE { 
        dpmm_ratio = 1f32;
    }

    size = size.round();
    let dpmm_size = (dpmm_ratio * size).round();

    x = (dpmm_ratio * x as f32).round() as _;
    y = (dpmm_ratio * y as f32).round() as _;


    //Construct a char buffer
    let mut char_buffer;
    let cwidth;
    let cheight;
    let scale;
    {
        let mut x0 = 0i32;
        let mut x1 = 0i32;
        let mut y0 = 0i32;
        let mut y1 = 0i32;
        let mut ascent = 0;
        let mut descent = 0;

        stbtt_GetFontVMetrics(&mut GLOBAL_FONTINFO as *mut stbtt_fontinfo,
                              &mut ascent as *mut i32,
                              &mut descent as *mut i32, null_mut());
        scale = stbtt_ScaleForPixelHeight(&GLOBAL_FONTINFO as *const stbtt_fontinfo, dpmm_size);
        let baseline = (ascent as f32 * scale ) as i32;

        cwidth = (scale * (ascent - descent) as f32 ) as usize + 4; //NOTE buffer term should be reduced.
        cheight = (scale * (ascent - descent) as f32 ) as usize + 4;//NOTE buffer term should be reduced.


        let glyph_index = stbtt_FindGlyphIndex(&GLOBAL_FONTINFO as *const stbtt_fontinfo, character as i32);
        
        char_buffer = match &mut FONT_GLYPH_HASHMAP{
            Some(fbh)=>{ 
                match fbh.get_mut(&CURRENT_KEY){
                    Some(font)=>{
                        match font.get(&(character, size as u32)){
                            Some(gi)=>
                            {
                            //NOTE
                            //Sets the previously rendered monotone character bmp.
                                gi
                            },
                            None=>{
                            //NOTE
                            //Generates a monotone character bmp and stores it in the hashmap.
                            //The results of the render are returned.
                                let mut _char_buffer = vec![0u8; cwidth * cheight];
                                stbtt_GetGlyphBitmapBoxSubpixel(&GLOBAL_FONTINFO as *const stbtt_fontinfo, glyph_index, scale, scale, 0.0,0.0,
                                                                        &mut x0 as *mut i32,
                                                                        &mut y0 as *mut i32,
                                                                        &mut x1 as *mut i32,
                                                                        &mut y1 as *mut i32);
                                stbtt_MakeGlyphBitmapSubpixel( &GLOBAL_FONTINFO as *const stbtt_fontinfo,
                                                               &mut _char_buffer[cwidth*(baseline + y0) as usize + (5 + x0) as usize ] as *mut u8,
                                                               x1-x0+2, y1-y0, cwidth as i32, scale, scale,0.0, 0.0, glyph_index);
                                font.insert((character, size as u32), _char_buffer);
                                font.get(&(character, size as u32)).as_ref().unwrap()
                            }
                        }

                    },
                    None=>{
                        panic!("FONT_GLYPH_HASHMAP does not recognize font."); 
                    }
                }
            },
            None=>{ panic!("FONT_GLYPH_HASHMAP has not been init."); }
        };

    }

    //NOTE
    //The character will not render if invisible.
    
    if character as u8 > 0x20{   //render char_buffer to main_buffer
        let buffer = canvas.buffer as *mut u32;
        let gwidth = canvas.w as isize;
        let gheight = canvas.h as isize;
        let offset = (x as isize + y as isize * gwidth);

        let instance = std::time::Instant::now(); 
        let width_mod_4 = cwidth % 4;

        let a = color[3];
        let orig_r = (color[0] * a);
        let orig_g = (color[1] * a);
        let orig_b = (color[2] * a);


        //NOTE simd feature.
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        //if false { if is_x86_feature_detected!("sse2") {unsafe{
        if true { if is_x86_feature_detected!("sse2") {unsafe{
//panic!("TODO {}  {}", buffer as usize , buffer as usize & 15);

//timeit!{{
            #[cfg(target_arch = "x86_64")]
            use std::arch::x86_64::*;
            #[cfg(target_arch = "x86")]
            use std::arch::x864::*;

            let orig_r = _mm_set1_ps(orig_r);
            let orig_g = _mm_set1_ps(orig_g);
            let orig_b = _mm_set1_ps(orig_b);

            let mut simd_dst_r = _mm_set1_ps(0f32);
            let mut simd_dst_g = _mm_set1_ps(0f32);
            let mut simd_dst_b = _mm_set1_ps(0f32);

            let mut simd_r = _mm_set1_ps(0f32);
            let mut simd_g = _mm_set1_ps(0f32);
            let mut simd_b = _mm_set1_ps(0f32);
            let mut simd_a = _mm_set1_ps(a);

            let mut simd_invert_255 = _mm_set1_ps(1./255.);
            let mut simd_one = _mm_set1_ps(1f32);

            let mut simd_tmp_r = _mm_set1_ps(0f32);
            let mut simd_tmp_g = _mm_set1_ps(0f32);
            let mut simd_tmp_b = _mm_set1_ps(0f32);

            let mut simd_dst = _mm_set1_epi32(0);


            let simd_mask_rule_char_length = _mm_set1_epi32(char_buffer.len() as i32);
            let simd_mask_rule_width       = _mm_set1_epi32(gwidth as i32);

            let mut text_alpha = [0f32;4];

            for i in 0..cheight as isize{
                if i + y as isize  > gheight {continue;}
                if i + y as isize  <= 0 {continue;}

                for j in (0..cwidth as isize).step_by(4){

                    
                    let mut mask = _mm_set1_epi32(0);
                    let simd_buffer = buffer.offset( (j as isize + i*gwidth + offset) as isize ) as *mut _;
                    simd_dst = _mm_loadu_si128(simd_buffer);


                    //NOTE
                    //simd mask implementation is WAY slower than looped equivilent and I'm not
                    //sure why. There are three compare operations and two ands. Compared to the 12
                    //compares and 4 sets done in a loop one would think simd would be faster. More
                    //exploration is required. 
                    
                    //let j_plus_x = j as i32 + x;
                    //let simd_mask_rule_gindex = _mm_set_epi32(j_plus_x + 3,
                    //                                          j_plus_x + 2,
                    //                                          j_plus_x + 1,
                    //                                          j_plus_x + 0);

                    //let char_index_i32 = j as i32 + cwidth as i32 * (cheight as i32 - 1 - i as i32);
                    //let simd_mask_rule_cindex = _mm_set_epi32(char_index_i32 + 3, 
                    //                                          char_index_i32 + 2,
                    //                                          char_index_i32 + 1,
                    //                                          char_index_i32 + 0);

                    //mask = _mm_cmplt_epi32(simd_mask_rule_gindex, simd_mask_rule_width);
                    //mask = _mm_and_si128( mask, _mm_cmpgt_epi32(simd_mask_rule_gindex, _mm_setzero_si128()) );
                    //mask = _mm_and_si128( mask, _mm_cmplt_epi32(simd_mask_rule_cindex, simd_mask_rule_char_length) );

                    for _j in 0..4{

                        if j + _j + x as isize  > gwidth {continue;}
                        if j + _j + x as isize  <= 0 {continue;}

                        let _j = _j as isize;
                        let dst_rgb = buffer.offset( (j+_j + i*gwidth + offset) as isize);
                        let _j = _j as usize;
                        let dst_r : &mut [f32; 4] = std::mem::transmute(&mut simd_dst_r);
                        let dst_g : &mut [f32; 4] = std::mem::transmute(&mut simd_dst_g);
                        let dst_b : &mut [f32; 4] = std::mem::transmute(&mut simd_dst_b);

                        dst_r[_j] = *(dst_rgb as *const u8).offset(2) as f32;
                        dst_g[_j] = *(dst_rgb as *const u8).offset(1) as f32;
                        dst_b[_j] = *(dst_rgb as *const u8).offset(0) as f32;


                        let _j = _j as usize;
                        if (j as usize + _j) + cwidth * (cheight - 1 - i as usize) >= char_buffer.len() { continue; }

                        text_alpha[_j] = char_buffer[(j as usize +_j) as usize + cwidth * (cheight - 1 - i as usize)] as f32;
                        let r : &mut [f32; 4] = std::mem::transmute(&mut simd_r);
                        let g : &mut [f32; 4] = std::mem::transmute(&mut simd_g);
                        let b : &mut [f32; 4] = std::mem::transmute(&mut simd_b);
                        r[_j] = text_alpha[_j];
                        g[_j] = text_alpha[_j];
                        b[_j] = text_alpha[_j];

                        let _mask: &mut [u32; 4] = std::mem::transmute(&mut mask);
                        _mask[_j] = 0xFF_FF_FF_FF;

                    }
                    

                    simd_tmp_r = _mm_mul_ps(simd_r, simd_invert_255);
                    simd_tmp_g = _mm_mul_ps(simd_g, simd_invert_255);
                    simd_tmp_b = _mm_mul_ps(simd_b, simd_invert_255);

                    simd_tmp_r = _mm_mul_ps(simd_tmp_r, simd_a);
                    simd_tmp_g = _mm_mul_ps(simd_tmp_g, simd_a);
                    simd_tmp_b = _mm_mul_ps(simd_tmp_b, simd_a);

                    simd_tmp_r = _mm_sub_ps(simd_one, simd_tmp_r);
                    simd_tmp_g = _mm_sub_ps(simd_one, simd_tmp_g);
                    simd_tmp_b = _mm_sub_ps(simd_one, simd_tmp_b);

                    simd_dst_r = _mm_mul_ps(simd_tmp_r, simd_dst_r);
                    simd_dst_g = _mm_mul_ps(simd_tmp_g, simd_dst_g);
                    simd_dst_b = _mm_mul_ps(simd_tmp_b, simd_dst_b);

                    simd_r = _mm_mul_ps(simd_r, orig_r);
                    simd_g = _mm_mul_ps(simd_g, orig_g);
                    simd_b = _mm_mul_ps(simd_b, orig_b);

                    simd_dst_r = _mm_add_ps(simd_r, simd_dst_r);
                    simd_dst_g = _mm_add_ps(simd_g, simd_dst_g);
                    simd_dst_b = _mm_add_ps(simd_b, simd_dst_b);

                    //NOTE converting from float to int for color channels
                    let mut simd_dst_r_u32 = _mm_cvtps_epi32(simd_dst_r);
                    simd_dst_r_u32 = _mm_slli_epi32(simd_dst_r_u32, 16);

                    let mut simd_dst_g_u32 = _mm_cvtps_epi32(simd_dst_g);
                    simd_dst_g_u32 = _mm_slli_epi32(simd_dst_g_u32, 8);

                    let mut simd_dst_b_u32 = _mm_cvtps_epi32(simd_dst_b);


                    //NOTE combining color channels
                    let mut simd_rgba = _mm_or_si128(_mm_or_si128(simd_dst_r_u32, simd_dst_g_u32), simd_dst_b_u32);


                    //NOTE applying pixel mask.
                    simd_rgba = _mm_or_si128( _mm_and_si128(mask, simd_rgba), _mm_andnot_si128(mask, simd_dst));

                    _mm_storeu_si128(simd_buffer, simd_rgba);

                }
            }
//}}//DEBUG REMOVE ME 

            //TODO redundant with return at the end of function.
            //we should call the same code to avoid issues.
            let mut adv : i32 = 0;
            let mut lft_br : i32 = 0; // NOTE: Maybe remove this
            stbtt_GetCodepointHMetrics(&GLOBAL_FONTINFO as *const stbtt_fontinfo, character as i32, &mut adv as *mut i32, &mut lft_br as *mut i32);
            return (adv as f32 * scale) as i32;
        }}}


        let y_is = y as isize;
        let x_is = x as isize;
        for i in 0..cheight as isize{
            if i + y_is > gheight {continue;}
            if i + y_is <= 0 {continue;}

            for j in 0..cwidth as isize{

                if (j + i*gwidth + offset) > gwidth * gheight {continue;}

                if j + x_is  > gwidth {continue;}
                if j + x_is  <= 0 {continue;}

                let mut text_alpha = char_buffer[j as usize + cwidth * (cheight - 1 - i as usize)] as f32;
                let r = (orig_r * text_alpha) as u32;
                let g = (orig_g * text_alpha) as u32;
                let b = (orig_b * text_alpha) as u32;

                let dst_rgb = buffer.offset( (j + i*gwidth + offset) as isize);

                text_alpha = (255.0 - text_alpha * a) / 255.0;
                
                let _r = (*(dst_rgb as *const u8).offset(2) as f32 * text_alpha ) as u32;
                let _g = (*(dst_rgb as *const u8).offset(1) as f32 * text_alpha ) as u32;
                let _b = (*(dst_rgb as *const u8).offset(0) as f32 * text_alpha ) as u32;


                *buffer.offset( (j + i*gwidth + offset) as isize) = 0x00000000 + (r+_r << 16) + (g+_g << 8) + b+_b;// + (final_alpha << 24);
            }
        }
    }

    let mut adv : i32 = 0;
    let mut lft_br : i32 = 0; // NOTE: Maybe remove this
    stbtt_GetCodepointHMetrics(&GLOBAL_FONTINFO as *const stbtt_fontinfo, character as i32, &mut adv as *mut i32, &mut lft_br as *mut i32);
    return (adv as f32 * scale) as i32;
}}


/// Draws the string to the canvas provided. Returns string width in pixels.
/// Position values x and y are indicate where the string will begin.
/// NOTE there is about a 4 pixel buffer between x and the first pixel the function is able to draw
/// to.
pub fn draw_string( canvas: &mut WindowCanvas, string: &str, x: i32, y: i32,
             color: [f32; 4], size: f32 )->i32{
    let mut offset = 0;
DEBUG_timeit!{"draw_string",{
    for it in string.chars(){
        offset += draw_char(canvas, it, x + offset, y, color, size);
    }
}}
    return offset;
}



/// Draws rectangle to the canvas provided. The dimensions of the rectangle should be given as follows
/// [x, y, width, height]. x, and y are associated with the bottom left corner of the rectangle.
pub fn draw_rect( canvas: &mut WindowCanvas, rect: [i32; 4], color: [f32; 4], filled: bool ){unsafe{
    //TODO
    //- Set alpha on dst canvas use both dst and src to determine alpha

    let buffer = canvas.buffer as *mut u32;

    let c_w = canvas.w as isize;
    let c_h = canvas.h as isize;


    let canvas_dpmm = if canvas.dpmm == 0f32 { DPMM_SCALE } else { canvas.dpmm };

    let mut dpmm_ratio = 1f32; //canvas_dpmm / DPMM_SCALE;
    if (1f32 - dpmm_ratio).abs() < DPMM_TOLERANCE { 
        dpmm_ratio = 1f32;
    }


    let x = ( dpmm_ratio * rect[0] as f32 ).round() as isize + 1; //NOTE 1 is here to remove wrapping TODO
    let y = ( dpmm_ratio * rect[1] as f32 ).round() as isize;

    let _x = if x < 0 { 0 } else { x };
    let _y = if y < 0 { 0 } else { y };


    let w = (dpmm_ratio * rect[2] as f32) as isize;
    let h = (dpmm_ratio * rect[3] as f32) as isize;
    let _w = if x + w > c_w { c_w - x } else if x < 0 { x + w } else {w};
    let _h = if y + h > c_h { c_h - y } else if y < 0 { y + h } else {h};


    let a = color[3];
    let r = (color[0] * a * 255.0) as u32;
    let g = (color[1] * a * 255.0) as u32;
    let b = (color[2] * a * 255.0) as u32;

    let one_minus_a = 1f32 - a;



    if x + w < 0 { return; }
    if y + h < 0 { return; }

    if x < c_w && y < c_h{ //TODO this is not correct
    } else {
        return;
    }

    if a >= 0.99 && filled == true{
        let mut fast_rgba_buffer = vec![0x00000000 + (r << 16) +  (g << 8)  + b; _w as usize];

        for _j in _y.._y+_h{
            let j = _j as isize;
            std::ptr::copy::<u32>(fast_rgba_buffer.as_ptr(), buffer.offset(c_w*j + _x), _w as usize);
        }
    }
    else{
        if filled == false {
        //Loop for non filled rect
            for _j in y..y+h{
                let j = _j as isize;
                for _i in x..x+w{
                    let i = _i as isize;
                    if i > c_w || j > c_h{
                        continue;
                    }
                    let dst_index =  (i + c_w*j) as isize;
                    let dst_rgb = buffer.offset(dst_index);
                    let _r = (*(dst_rgb as *const u8).offset(2) as f32 * one_minus_a) as u32;
                    let _g = (*(dst_rgb as *const u8).offset(1) as f32 * one_minus_a) as u32;
                    let _b = (*(dst_rgb as *const u8).offset(0) as f32 * one_minus_a) as u32;

                    if (_i - x) > 1 && (_i - x ) < w-2 &&
                      (_j - y) > 1 && (_j - y ) < h-2{continue;}

                    *buffer.offset(dst_index) = 0x00000000 + (r+_r << 16) +  (g+_g << 8)  + b+_b;
                }
            }
        } else {

            //TODO
            //simd this
            for _j in _y.._y+_h{
                let j = _j as isize;
                for _i in x..x+_w{
                    let i = _i as isize;
                    let dst_index =  (i + c_w*j) as isize;
                    let dst_rgb = buffer.offset(dst_index);

                    let _r = (*(dst_rgb as *const u8).offset(2) as f32 * one_minus_a) as u32;
                    let _g = (*(dst_rgb as *const u8).offset(1) as f32 * one_minus_a) as u32;
                    let _b = (*(dst_rgb as *const u8).offset(0) as f32 * one_minus_a) as u32;

                    *buffer.offset(dst_index) = 0x00000000 + (r+_r << 16) +  (g+_g << 8)  + b+_b;;
                }
            }
        }
    }
}}

#[derive(Clone, Debug, Copy)]
pub struct TGBitmapHeaderInfo{
    pub header_size:        u32,
    pub width:              i32,
    pub height:             i32,
    pub planes:             u16,
    pub bit_per_pixel:      u16,
    pub compression:        u32,
    pub image_size:         u32,
    pub x_px_per_meter:     i32,
    pub y_px_per_meter:     i32,
    pub colors_used:        u32,
    pub colors_important:   u32,
}


#[repr(packed)]
#[derive(Clone, Debug, Default, Copy)]
pub struct TGBitmapFileHeader{
   pub  type_:              u16,
   pub  size_:              u32,
   pub  reserved_1:         u16,
   pub  reserved_2:         u16,
   pub  off_bits:           u32,
}


#[derive(Clone)]
pub struct TGBitmap{
   pub file_header:        TGBitmapFileHeader,
   pub info_header:        TGBitmapHeaderInfo,
   pub rgba:               Vec<u8>,

  //For ease of use
   pub width  : i32,
   pub height : i32,
}
impl TGBitmap{

    ///Generates a new bitmap of the given width and height.
    pub fn new(w: i32, h: i32)->TGBitmap{
        TGBitmap{
            file_header: TGBitmapFileHeader{
                type_: 0x4d42, //BM
                size_: 0,
                reserved_1: 0,
                reserved_2: 0,
                off_bits: 0,
            },
            info_header:   TGBitmapHeaderInfo{
                    header_size:        54,//TODO
                    width:              w,
                    height:             h,
                    planes:             1,
                    bit_per_pixel:      32,
                    compression:        0,
                    image_size:         0,
                    x_px_per_meter:     0,
                    y_px_per_meter:     0,
                    colors_used:        0,
                    colors_important:   0,
            },
            rgba: vec![0;4 * (w*h) as usize],
            width  : w,
            height : h,
        }
    }

    ///Generates a new bitmap from a bitmap stored directly in memory.
    ///This functions copies data. 
    ///NOTE: this is not the most efficient system. We are double the amount of memory we use
    ///and that may not be necessary.
    pub fn from_buffer(img_buffer: &[u8])->TGBitmap{unsafe{//TODO this is not the way we should switch to STB
        let mut rt = TGBitmap::new(0,0);

        let it =  img_buffer.as_ptr() as *const u8;
        rt.file_header.type_ =  *(it.offset(0) as *const u16);// == 0x42;
        rt.file_header.size_ = *(it.offset(2) as *const u32);
        rt.file_header.reserved_1 = *(it.offset(6) as *const u16);
        rt.file_header.reserved_2 = *(it.offset(8) as *const u16);
        rt.file_header.off_bits =  *(it.offset(10) as *const u32);


        rt.info_header.header_size = *(it.offset(14) as *const u32);
        rt.info_header.width       = *(it.offset(18) as *const i32);
        rt.info_header.height      =  *(it.offset(22) as *const i32);
        rt.info_header.planes      =  *(it.offset(26) as *const u16);
        rt.info_header.bit_per_pixel = *(it.offset(28) as *const u16);
        rt.info_header.compression = *(it.offset(30) as *const u32);
        rt.info_header.image_size  = *(it.offset(34) as *const u32);
        rt.info_header.x_px_per_meter = *(it.offset(38) as *const i32);
        rt.info_header.y_px_per_meter = *(it.offset(42) as *const i32);
        rt.info_header.colors_used  = *(it.offset(46) as *const u32);
        rt.info_header.colors_important = *(it.offset(50) as *const u32);


        let buffer = img_buffer[rt.file_header.off_bits as usize ..].to_vec();
        rt.rgba = buffer;
        rt.width =  rt.info_header.width; 
        rt.height =  rt.info_header.height; 

        return rt;
    }}

    ///Generates a bmp from a file found in the given path.
    ///The function panics if file is not found.
    pub fn load_bmp(filename: &str)->TGBitmap{unsafe{
        let mut rt = TGBitmap::new(0,0);
        let mut f = File::open(filename).expect("BMP file could not be opened.");
        let mut img_buffer = Vec::new();
        f.read_to_end(&mut img_buffer).expect("Buffer could not be read.");

        rt = TGBitmap::from_buffer(&img_buffer);
        return rt;
    }}

    ///Write bmp to disk at given path.
    pub fn save_bmp(&self, filename: &str){unsafe{//TODO

        use std::mem::transmute;
        let filename = format!("{}", filename);
        let mut filebuffer = match File::create(filename){
            Ok(_fb) => _fb,
            Err(_s) => {
                println!("BMP file could not be made. {}", _s);
                return;
            }
        };

        {
            filebuffer.write( &transmute::<_, [u8; 2]>(self.file_header.type_) ).expect("BMP file_header.type could not be written.");
            filebuffer.write( &transmute::<_, [u8; 4]>(self.file_header.size_) ).expect("BMP file_header.size could not be written.");
            filebuffer.write( &transmute::<_, [u8; 2]>(self.file_header.reserved_1) ).expect("BMP file_header.reserverd_1 could not be written.");
            filebuffer.write( &transmute::<_, [u8; 2]>(self.file_header.reserved_2) ).expect("BMP file_header.reserved_2 could not be written.");
            filebuffer.write( &transmute::<_, [u8; 4]>(self.file_header.off_bits) ).expect("BMP file_header.off_bits could not be written.");
        }
        {

            filebuffer.write( &transmute::<_, [u8; 4]>(self.info_header.header_size) ).expect("BMP info_header.header_size could not be written.");
            filebuffer.write( &transmute::<_, [u8; 4]>(self.info_header.width) ).expect("BMP info_header.width could not be written.");
            filebuffer.write( &transmute::<_, [u8; 4]>(self.info_header.height) ).expect("BMP info_header.height could not be written.");
            filebuffer.write( &transmute::<_, [u8; 2]>(self.info_header.planes) ).expect("BMP info_header.planes could not be written.");
            filebuffer.write( &transmute::<_, [u8; 2]>(self.info_header.bit_per_pixel) ).expect("BMP info_header.bit_per_pixel could not be written.");
            filebuffer.write( &transmute::<_, [u8; 4]>(self.info_header.compression) ).expect("BMP info_header.compression could not be written.");
            filebuffer.write( &transmute::<_, [u8; 4]>(self.info_header.image_size) ).expect("BMP info_header.image_size could not be written.");
            filebuffer.write( &transmute::<_, [u8; 4]>(self.info_header.x_px_per_meter) ).expect("BMP info_header.x_px_per_meter could not be written.");
            filebuffer.write( &transmute::<_, [u8; 4]>(self.info_header.y_px_per_meter) ).expect("BMP info_header.y_px_per_meter could not be written.");
            filebuffer.write( &transmute::<_, [u8; 4]>(self.info_header.colors_used) ).expect("BMP info_header.colors_used could not be written.");
            filebuffer.write( &transmute::<_, [u8; 4]>(self.info_header.colors_important) ).expect("BMP info_header.colors_important could not be written.");
        }
        filebuffer.write( &self.rgba ).expect("BMP rgba arr could not be written.");
        
    }}

    ///Generates a TGBitmap from StbiImage format.
    pub fn from_stbi( image: StbiImage )->TGBitmap{
        let mut buffer = vec![0u8; (4*image.width*image.height) as usize];
        for i in (0..image.height as usize).rev(){
            for j in 0..image.width as usize{
                let offset_im  = 4 * ( i*image.width as usize + j );
                let offset_new = 4 * ((image.height as usize - i - 1)*image.width as usize + j);
                buffer[offset_new + 0] = image.buffer[offset_im + 2];
                buffer[offset_new + 1] = image.buffer[offset_im + 1];
                buffer[offset_new + 2] = image.buffer[offset_im + 0];
                buffer[offset_new + 3] = image.buffer[offset_im + 3];
            }
        }
        TGBitmap{
            file_header: TGBitmapFileHeader{
                type_: 0x4d42, //BM
                size_: 0,
                reserved_1: 0,
                reserved_2: 0,
                off_bits: 0,
            },
            info_header:   TGBitmapHeaderInfo{
                    header_size:        54,//TODO
                    width:              image.width,
                    height:             image.height,
                    planes:             1,
                    bit_per_pixel:      32,
                    compression:        0,
                    image_size:         0,
                    x_px_per_meter:     0,
                    y_px_per_meter:     0,
                    colors_used:        0,
                    colors_important:   0,
            },
            rgba: buffer,
            width  : image.width,
            height : image.height,
        }
    }

}

///Returns a new bmp that is the resized version of the old bmp.
///The function resizes to the width and height specified.
///This function currently uses the sampling reduction algorithm.
pub fn resize_bmp(source_bmp: &TGBitmap, w: i32, h: i32)->TGBitmap{unsafe{
    return sampling_reduction_bmp(source_bmp, w, h);
}}

///Returns a new bmp that is the resized version of the old bmp.
///The algorithm replaces each pixel with new pixel(s) of the same color. 
///This technique may result is jaggedness.
pub fn sampling_reduction_bmp(source_bmp: &TGBitmap, w: i32, h: i32)->TGBitmap{unsafe{
    let mut bmp = TGBitmap::new(w, h);

    let scale_w = source_bmp.info_header.width as f32 / w as f32;
    let scale_h = source_bmp.info_header.height as f32 / h as f32;

    let dst_buffer = bmp.rgba.as_mut_ptr() as *mut u32;
    let src_buffer = source_bmp.rgba.as_ptr() as *const u32;

    for j in 0..h as isize{
        for i in 0..w as isize{
            let _i = j * w as isize + i;
            let src_i = (j as f32 * scale_h) as isize * source_bmp.width as isize + (i as f32 * scale_w) as isize;

            let rgb = dst_buffer.offset( _i );
            *rgb = *src_buffer.offset( src_i);
        }
    }

    return bmp;
    
}}


///Draws stbi_image to the canvas provided. x, and y dictate where the image is draws to the canvas.
///This point is associated with the bottom left corner of the image.
pub fn draw_stbi_image( canvas: &mut WindowCanvas, bmp: &StbiImage, mut x: i32, mut y: i32, alpha: f32,
            mut _w: Option<i32>, mut _h: Option<i32>){unsafe{

    if alpha < 0.0 {
        println!("A negative alpha as passed to drawBMP");
        return;
    }
    let w;
    let h;


    let canvas_dpmm = if canvas.dpmm == 0f32 { DPMM_SCALE } else { canvas.dpmm };

    let mut dpmm_ratio = 1f32;//canvas_dpmm / DPMM_SCALE;
    if (1f32 - dpmm_ratio).abs() < DPMM_TOLERANCE { 
        dpmm_ratio = 1f32;
    }
    x = (dpmm_ratio * x as f32).round() as _;
    y = (dpmm_ratio * y as f32).round() as _;

    if dpmm_ratio != 1f32 {
        if _w.is_none() {
            _w  = Some( bmp.width );
        } 
        if _h.is_none() {
            _h  = Some( bmp.height );
        }
    }

    match _w {
        Some(int) => w = (dpmm_ratio * int as f32).round() as _ ,
        None => w = bmp.width,
    }
    match _h {
        Some(int) => h = (dpmm_ratio * int as f32).round() as _,
        None => h = bmp.height,
    }

    //TODO
    //let bmp = if w == source_bmp.width &&
    //             h == source_bmp.height{
    //                 (*source_bmp).clone()
    //             } else {
    //                sampling_reduction_bmp(source_bmp, w, h)
    //             };

    {   //render bmp_buffer to main_buffer

        let buffer = canvas.buffer as *mut u32;
        let gwidth = canvas.w as i32;
        let gheight = canvas.h as i32;
        let offset = (x + y * gwidth) as i32;
        let bit_stride = (bmp.bits_per_pixel / 8) as i32;

        let color = bmp.buffer.as_ptr();
        if alpha >= 0.99 {
            for i in (0..bmp.height).rev(){
                let _w = bmp.width as usize;
                let _off_src = i as isize * _w as isize * bit_stride as isize;
                let _off_dst = i as isize * gwidth as isize;
                std::ptr::copy::<u32>(color.offset(_off_src) as *const u32, buffer.offset( _off_dst + offset as isize), _w);
            }
        } else {
            for i in (0..bmp.height).rev(){
                //TODO
                //when alpha is one copy the bmp bits instead of iterating through the array
                for j in 0..bmp.width{

                    if (j + i*gwidth + offset) < 0 {continue;}
                    if (j + i*gwidth + offset) > gwidth * gheight {continue;}

                    if j + x > gwidth {continue;}
                    if i + y > gheight {continue;}


                    let a = (*color.offset(( bit_stride * (j + i * bmp.width) + 3) as isize) as f32 ) / 255.0;
                    let r = (*color.offset(( bit_stride * (j + i * bmp.width) + 2) as isize) as f32 * alpha * a ) as u32;
                    let g = (*color.offset(( bit_stride * (j + i * bmp.width) + 1) as isize) as f32 * alpha * a ) as u32;
                    let b = (*color.offset(( bit_stride * (j + i * bmp.width) + 0) as isize) as f32 * alpha * a ) as u32;


                    let dst_rgb = buffer.offset( (j + i*gwidth + offset) as isize);
                    let _r = (*(dst_rgb as *const u8).offset(2) as f32 * (1.0 - alpha * a )) as u32;
                    let _g = (*(dst_rgb as *const u8).offset(1) as f32 * (1.0 - alpha * a )) as u32;
                    let _b = (*(dst_rgb as *const u8).offset(0) as f32 * (1.0 - alpha * a )) as u32;

                    let r_cmp = (r+_r).min(255).max(0);
                    let g_cmp = (g+_g).min(255).max(0);
                    let b_cmp = (b+_b).min(255).max(0);

                    *buffer.offset( (j + i*gwidth + offset) as isize) = 0x00000000 + (r_cmp << 16) + (g_cmp << 8) + b_cmp;
                }
            }
        }
    }
}}


///Draws TGBitmap to the canvas provided. x, and y dictate where the image is draws to the canvas.
///This point is associated with the bottom left corner of the image.
pub fn draw_bmp( canvas: &mut WindowCanvas, source_bmp: &TGBitmap, mut x: i32, mut y: i32, alpha: f32,
            mut _w: Option<i32>, mut _h: Option<i32>){unsafe{

DEBUG_timeit!{ "draw_bmp", {
    if alpha < 0.0 {
        println!("A negative alpha as passed to drawBMP");
        return;
    }
    let w;
    let h;


    let canvas_dpmm = if canvas.dpmm == 0f32 { DPMM_SCALE } else { canvas.dpmm };

    let mut dpmm_ratio = 1f32;//canvas_dpmm / DPMM_SCALE;
    if (1f32 - dpmm_ratio).abs() < DPMM_TOLERANCE { 
        dpmm_ratio = 1f32;
    }
    x = (dpmm_ratio * x as f32).round() as _;
    y = (dpmm_ratio * y as f32).round() as _;

    if dpmm_ratio != 1f32 {
        if _w.is_none() {
            _w  = Some( source_bmp.width );
        } 
        if _h.is_none() {
            _h  = Some( source_bmp.height );
        }
    }

    match _w {
        Some(int) => w = (dpmm_ratio * int as f32).round() as _,
        None => w = source_bmp.info_header.width,
    }
    match _h {
        Some(int) => h = (dpmm_ratio * int as f32).round() as _,
        None => h = source_bmp.info_header.height,
    }

    let bmp = if w == source_bmp.info_header.width &&
                 h == source_bmp.info_header.height{
                     (*source_bmp).clone()
                 } else {
                    sampling_reduction_bmp(source_bmp, w, h)
                 };

    {   //render bmp_buffer to main_buffer

        let buffer = canvas.buffer as *mut u32;
        let gwidth = canvas.w as i32;
        let gheight = canvas.h as i32;
        let offset = (x + y * gwidth) as i32;
        let bit_stride = (bmp.info_header.bit_per_pixel / 8) as i32;
        let inv_255 = 1.0 / 255.0;

        let color = bmp.rgba.as_ptr();
        if alpha >= 0.99 {
            for i in (0..bmp.info_header.height).rev(){
                let _w = bmp.info_header.width as usize;
                let _off_src = i as isize * _w as isize * bit_stride as isize;
                let _off_dst = i as isize * gwidth as isize;
                std::ptr::copy::<u32>(color.offset(_off_src) as *const u32, buffer.offset( _off_dst + offset as isize), _w);
            }
        } else {
            for i in (0..bmp.info_header.height).rev(){
                //TODO
                //simd
                for j in 0..bmp.info_header.width{

                    if (j + i*gwidth + offset) < 0 {continue;}
                    if (j + i*gwidth + offset) > gwidth * gheight {continue;}

                    if j + x > gwidth {continue;}
                    if i + y > gheight {continue;}


                    let a = (*color.offset(( bit_stride * (j + i * bmp.info_header.width) + 3) as isize) as f32 ) * inv_255;
                    let r = (*color.offset(( bit_stride * (j + i * bmp.info_header.width) + 2) as isize) as f32 * alpha * a ) as u32;
                    let g = (*color.offset(( bit_stride * (j + i * bmp.info_header.width) + 1) as isize) as f32 * alpha * a ) as u32;
                    let b = (*color.offset(( bit_stride * (j + i * bmp.info_header.width) + 0) as isize) as f32 * alpha * a ) as u32;


                    let dst_rgb = buffer.offset( (j + i*gwidth + offset) as isize);
                    let _r = (*(dst_rgb as *const u8).offset(2) as f32 * (1.0 - alpha * a )) as u32;
                    let _g = (*(dst_rgb as *const u8).offset(1) as f32 * (1.0 - alpha * a )) as u32;
                    let _b = (*(dst_rgb as *const u8).offset(0) as f32 * (1.0 - alpha * a )) as u32;

                    let r_cmp = (r+_r).min(255).max(0);
                    let g_cmp = (g+_g).min(255).max(0);
                    let b_cmp = (b+_b).min(255).max(0);

                    *buffer.offset( (j + i*gwidth + offset) as isize) = 0x00000000 + (r_cmp << 16) + (g_cmp << 8) + b_cmp;
                }
            }
        }
    }
}}
}}




///Draws a circle to the canvas provided. x, and y dictate where the image is draws to the canvas.
///This point(x,y) is the center of the circle.
pub fn draw_circle(canvas: &mut WindowCanvas, mut _x: i32, mut _y: i32, r: f32, color: [f32; 4]){unsafe{
//TODO time test is needed. It is very likely this function is slow.

    let buffer = canvas.buffer as *mut u32;

    let c_w = canvas.w as isize;
    let c_h = canvas.h as isize;

    let canvas_dpmm = if canvas.dpmm == 0f32 { DPMM_SCALE } else { canvas.dpmm };

    let mut dpmm_ratio = 1f32;//canvas_dpmm / DPMM_SCALE;
    if (1f32 - dpmm_ratio).abs() < DPMM_TOLERANCE { 
        dpmm_ratio = 1f32;
    }


    _x = (dpmm_ratio * _x as f32).round() as _;
    _y = (dpmm_ratio * _y as f32).round() as _;

    let x = (_x - r as i32) as isize;
    let y = (_y - r as i32)as isize;
    let w = (2.0*r) as isize;
    let h = (2.0*r) as isize;


    let a = color[3].max(0f32).min(1f32);

    let mut index_i = 0;
    let mut index_j = 0;
    for _j in y..y+h{
        let j = _j as isize;
        index_j += 1;
        index_i = 0;


        for _i in x..x+w{
            let i = _i as isize;
            index_i += 1;

            if i >= c_w || j >= c_h || i < 0 || j < 0 {
                continue;
             }

            let dst_rgb = buffer.offset( (i + c_w*j) as isize);


            let radius = ((index_i as f32 - w as f32/2.0).powf(2.0) + (index_j as f32 - h as f32/2.0).powf(2.0)).sqrt();
            let radius_factor =  ( 1.4*(r-radius) / (1.0 + (1.4*(r-radius)).abs())).max(0.0) ;


            let _r = (*(dst_rgb as *const u8).offset(2) as f32 * (1.0 - a*radius_factor)) as u32;
            let _g = (*(dst_rgb as *const u8).offset(1) as f32 * (1.0 - a*radius_factor)) as u32;
            let _b = (*(dst_rgb as *const u8).offset(0) as f32 * (1.0 - a*radius_factor)) as u32;

            let r = (color[0] * a * radius_factor * 255.0) as u32;
            let g = (color[1] * a * radius_factor * 255.0) as u32;
            let b = (color[2] * a * radius_factor * 255.0) as u32;


            *buffer.offset(i + c_w*j) = 0x00000000 + (r+_r << 16) +  (g+_g << 8)  + b+_b;

        }
    }
}}






///Draws a subcanvas to the canvas provided. x, and y dictate where the image is draws to the canvas.
///This point(x,y) is the center of the circle.
pub fn draw_subcanvas(canvas: &mut WindowCanvas, subcanvas: &SubCanvas, mut x: i32, mut y: i32, alpha: f32 ){unsafe{

    //TODO we need to scale with dpi
    let buffer = canvas.buffer as *mut u32;
    let subbuffer = subcanvas.canvas.buffer as *mut u32;

    let c_w = canvas.w as isize;
    let c_h = canvas.h as isize;

    let canvas_dpmm = if canvas.dpmm == 0f32 { DPMM_SCALE } else { canvas.dpmm };

    let mut dpmm_ratio = 1f32;//canvas_dpmm / DPMM_SCALE;
    if (1f32 - dpmm_ratio).abs() < DPMM_TOLERANCE { 
        dpmm_ratio = 1f32;
    }

    x = (dpmm_ratio * x as f32).round() as _;
    y = (dpmm_ratio * y as f32).round() as _;


    let x = x as isize;
    let y = y as isize;
    let w = subcanvas.canvas.w as isize;
    let h = subcanvas.canvas.h as isize - 1;//-1 is here to remove junk. We should look more closely at this TODO
    let a = alpha.max(0f32);


    if alpha < 0.99f32 {
        let mut j_ = 0;
        for _j in y..y+h{
            let j = _j as isize;
            j_ += 1;

            let mut i_ = 0;
            for _i in x..x+w{
                let i = _i as isize;
                i_ += 1;
                if i < 0 || j < 0 { continue; }
                if i > c_w || j >= c_h{
                    continue;
                }
                let dst_rgb = buffer.offset( (i + c_w*j) as isize);
                let src_rgb = subbuffer.offset( (i_ + w*j_) as isize);


                let _r = (*(dst_rgb as *const u8).offset(2) as f32 * (1.0 - a)) as u32;
                let _g = (*(dst_rgb as *const u8).offset(1) as f32 * (1.0 - a)) as u32;
                let _b = (*(dst_rgb as *const u8).offset(0) as f32 * (1.0 - a)) as u32;

                let r = (*(src_rgb as *const u8).offset(2) as f32 * a) as u32;
                let g = (*(src_rgb as *const u8).offset(1) as f32 * a) as u32;
                let b = (*(src_rgb as *const u8).offset(0) as f32 * a) as u32;


                *buffer.offset(i + c_w*j) = 0x00000000 + (r+_r << 16) +  (g+_g << 8)  + b+_b;

            }
        }
    } else {
        let mut j_ = 0;
        for _j in y..y+h{
            let j = _j as isize;
            j_ += 1;


            let dst_rgb = buffer.offset( (x + c_w*j) as isize);
            let src_rgb = subbuffer.offset( w*j_ as isize);
            let _w = if x+w > canvas.w as isize { c_w - x } else { w };
            if _w < 0 { break; }
            if j > c_h { break; }
            if j_ > c_h { break; }

            std::ptr::copy::<u32>(src_rgb as *const u32, dst_rgb as *mut u32, _w as usize);
        }
    }

}}

pub struct SubCanvas{
    pub canvas: WindowCanvas,
    pub buffer: Vec<u8>,
}
impl SubCanvas{
    pub fn new(w: i32, h: i32)->SubCanvas{unsafe{
        use std::mem;

        let mut wc = WindowCanvas{
            info: TGBitmapHeaderInfo{
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
            },
            buffer: null_mut(),
            w: w,
            h: h,

            display_width    : 0,
            display_width_mm : 0,

            display_height   : 0,
            display_height_mm: 0,

            dpmm: DPMM_SCALE,
        };

        let buffer = vec![0u8; (w*h*4) as _];
        let mut canvas = SubCanvas{
            canvas: wc,
            buffer: buffer
        };
        canvas.canvas.buffer = canvas.buffer.as_mut_ptr() as _;

        return canvas;
    }}
}


pub mod multithreaded_renderer{
//Should we use a subcanvas instead of a windowcanvas?
use crate::WindowCanvas;
use std::{thread, time};
use std::sync::atomic::{AtomicU8, Ordering};
use crate::rendertools::{SubCanvas, draw_subcanvas};
//
//TODO need to get cpuid a
//https://doc.rust-lang.org/stable/core/arch/x86/struct.CpuidResult.html
//Rust/Documentations/(intel and amd)

    static mut THREAD_WINDOW : Option<SubCanvas> = None;

    static mut THREAD_POOL : Option<Vec<thread::JoinHandle<()>>> = None;
    static mut THREAD_STATUS : Option<Vec<AtomicU8>> = None;
    static mut THREAD_BOUNDING_RECT : Option<Vec<[i32; 4]>> = None;

    const CLOSED : u8 = 0;
    const OPEN   : u8 = 1;
    const KILL   : u8 = 3;


    static mut TRANSFORMATION_PIPELINE : Option<Vec<PixelShader>> = None;

    //TODO combine pixel and distance shaders
    //TODO enum for shader data?
    type PixelShader = fn([f32; 2], &[Vec<f32>])->[f32;4];
    static mut PIXEL_SHADER_FUNCTION_PIPELINE : Option<Vec<PixelShader>> = None;
    static mut PIXEL_SHADER_DATA_PIPELINE : Option<Vec<Vec<Vec<f32>>>> = None;


/*TODO
pub type TransformationMaxtrix = [f32; 9];
*/

    #[derive(Debug)]
    pub enum MTError{
        Err,
        PreviouslyInit,
        Default
    }

    pub fn mt_print_status(){unsafe{
        if THREAD_STATUS.is_none(){
            println!("Renderer has been improperly initialized.");
            return;
        }
        println!("Number of threads: {}", THREAD_POOL.as_ref().unwrap().len());
        println!("Thread status: {:?}", THREAD_STATUS);

        let bounding_rects = THREAD_BOUNDING_RECT.as_ref().unwrap();
        for i in 0..bounding_rects.len() {
            println!("Thread {}: rect {:?}", i, bounding_rects[i]);
        }

        if PIXEL_SHADER_FUNCTION_PIPELINE.is_none(){
            println!("Pixel shader pipeline has not been set.");
            return;
        }


        let pixel_shaders = PIXEL_SHADER_FUNCTION_PIPELINE.as_ref().unwrap();
        let pixel_function_inputs = PIXEL_SHADER_DATA_PIPELINE.as_ref().unwrap();


        println!("Number of Pixel Shaders: {}", pixel_shaders.len());
        println!("Number of Pixel Inputs: {}",  pixel_function_inputs.len());

    }}


    fn thread_function(){
        let thread_id = thread::current().name().expect("Thread was not named.")
                                         .parse::<usize>().expect("Thread was not named a number.");
        unsafe{
            if THREAD_STATUS.is_none(){
                panic!("Renderer has been improperly initialized.");
            }
        }
        let mut thread_good = true; 
        while thread_good {
            let status = unsafe{ THREAD_STATUS.as_ref().unwrap()[thread_id].load(Ordering::Relaxed) };
            match status {
                OPEN => { thread::sleep(time::Duration::from_millis(1)); },
                KILL => { thread_good = false; },
                CLOSED => {
                    //TODO
                    //get bounding rect
                    let rect = unsafe{ THREAD_BOUNDING_RECT.as_ref().unwrap()[thread_id] };
                    let canvas = unsafe{ &mut THREAD_WINDOW.as_mut().unwrap().canvas }; 

                    let pixel_functions = unsafe{ PIXEL_SHADER_FUNCTION_PIPELINE.as_ref().unwrap() };
                    let pixel_data = unsafe{ PIXEL_SHADER_DATA_PIPELINE.as_ref().unwrap() };

                    shaders(canvas, rect, //transformation_matrix: &[TransformationMaxtrix], 
                                                           pixel_functions,
                                                           pixel_data);

                    unsafe{ THREAD_STATUS.as_mut().unwrap()[thread_id].store(OPEN, Ordering::Relaxed); }
                },
                _=>{ panic!("Unexpected thread status!"); }

            }
        }

    }


    pub fn init_multithread_renderer(n_threads: usize, window_width: i32, window_height: i32)->Result<(), MTError>{unsafe{
        //TODO we know now many cores we have on this computer (Linux machine with 4 cores)
        //so we create 5 threads, making use of atleast 3 cores
        if n_threads == 0 {
            return Ok(());
        }

        if THREAD_WINDOW.is_some(){
            return Err(MTError::PreviouslyInit);
        }
        THREAD_WINDOW = Some(SubCanvas::new(window_width, window_height));
        let mut arr_threads       = vec![];
        let mut arr_thread_status = vec![];
        let mut arr_thread_rect   = vec![];

        for i in 0..n_threads{
            arr_thread_status.push(AtomicU8::new(OPEN));
        }
        THREAD_STATUS = Some(arr_thread_status);

        let _h = window_height / n_threads as i32;
        for i in 0..n_threads{
            let mut h = _h;
            if i == n_threads - 1 {
                h = window_height - _h*i as i32;
            }


            let mut builder = thread::Builder::new().name(i.to_string());
            arr_threads.push( builder.spawn(thread_function).expect("Thread could not be made.") );
            arr_thread_rect.push([0, _h*i as i32,window_width, h]);

        }
        THREAD_POOL = Some(arr_threads);
        THREAD_BOUNDING_RECT  = Some(arr_thread_rect);

        PIXEL_SHADER_FUNCTION_PIPELINE = Some(vec![]);
        PIXEL_SHADER_DATA_PIPELINE     = Some(vec![]); 




        return Ok(());
    }}

    pub fn mt_render()->Result<(), MTError>{unsafe{
        //tell threads to render the things
        //wait for threads to finish
        if THREAD_STATUS.is_none(){
            return Err(MTError::Err);
        }
        let mut thread_status = THREAD_STATUS.as_mut().unwrap();
        let mut threads_open = true;
        for i in 0..thread_status.len(){
            if thread_status[i].load(Ordering::Relaxed) != OPEN{ 
                threads_open = false;
            }
        }
        if threads_open {
            for i in 0..thread_status.len(){
                thread_status[i].store(CLOSED, Ordering::Relaxed);
            }
            threads_open = false;
        } else {
            return Err(MTError::Err);
        }

        while threads_open == false { 
            threads_open = true;
            for i in 0..thread_status.len(){
                if thread_status[i].load(Ordering::Relaxed) != OPEN{ 
                    threads_open = false;
                }
            }
            
        }


        PIXEL_SHADER_FUNCTION_PIPELINE.as_mut().unwrap().clear();
        PIXEL_SHADER_DATA_PIPELINE.as_mut().unwrap().clear();


        return Ok(());
    }}

    pub fn mt_render_to_canvas(canvas: &mut WindowCanvas, x: i32, y: i32, alpha: f32)->Result<(), MTError>{unsafe{
        if THREAD_STATUS.is_none(){
            return Err(MTError::Err);
        }

        if PIXEL_SHADER_FUNCTION_PIPELINE.as_mut().unwrap().len() > 0 {
            mt_render()?;
        }

        let subcanvas = THREAD_WINDOW.as_ref().unwrap();
        draw_subcanvas(canvas, subcanvas, x, y, alpha);
        return Ok(());
    }}



    pub fn mt_shader(pixel_function: PixelShader, p_data: &[&[f32]])->Result<(), MTError>{unsafe{
        if THREAD_STATUS.is_none(){
            return Err(MTError::Err);
        }
        
        PIXEL_SHADER_FUNCTION_PIPELINE.as_mut().unwrap().push(pixel_function);

        let _pipe = PIXEL_SHADER_DATA_PIPELINE.as_mut().unwrap();
        _pipe.push(vec![]);
        let pipe_index = _pipe.len() - 1;

        for i in 0..p_data.len(){
            _pipe[pipe_index].push(p_data[i].to_vec());
        }
        
        return Ok(());
    }}
    pub fn mt_clear()->Result<(), MTError>{unsafe{
        if THREAD_STATUS.is_none(){
            return Err(MTError::Err);
        }
        let window = THREAD_WINDOW.as_mut().unwrap();
        let c = 0u8;
        std::ptr::write_bytes::<u8>(window.buffer.as_ptr() as *mut u8, c, window.buffer.len());
        return Ok(());
    }}

    pub fn mt_draw_rect(rect: [i32; 4], color: [f32; 4])->Result<(), MTError>{unsafe{
        //TODO
        
        if THREAD_STATUS.is_none(){
            return Err(MTError::Err);
        }

        let thread_window = THREAD_WINDOW.as_ref().unwrap();
        
        let mut f_rect = {//TODO convert to -1 to 1 float representation 
            let w = thread_window.canvas.w as f32;
            let h = thread_window.canvas.h as f32;

            let min_w_h = w.min(h);

            [ 2.0 * rect[0] as f32 / min_w_h - w/min_w_h,
              2.0 * rect[1] as f32 / min_w_h - h/min_w_h,
              2.0 * rect[2] as f32 / min_w_h,
              2.0 * rect[3] as f32 / min_w_h,
            ]
            
        };

        fn d_rect(mut p: [f32; 2], data: &[f32])->f32{
            p[0] -= data[0];
            p[1] -= data[1];

            let b = [data[2], data[3]];
            let mut d = [0f32; 2];
            d[0] = p[0].abs()-b[0];
            d[1] = p[1].abs()-b[1];

            let d_0_m = d[0].max(0f32);
            let d_1_m = d[1].max(0f32);
            return (d_0_m.powi(2) + d_1_m.powi(2)).sqrt();
        }

        fn fill_color(p: [f32; 2], inputs: &[Vec<f32>])->[f32; 4]{
            if inputs.len() < 2 {
                panic!("fill_color did not get proper number of inputs");
            }


            let d = d_rect(p, &inputs[0]);

            let r = inputs[1][0];
            let g = inputs[1][1];
            let b = inputs[1][2];
            let a = inputs[1][3];

            if d.is_finite(){
                return [r,g,b,a*((-d*10f32).exp()).max(0.0).min(1.0)];
            } else {
                return [r,g,b,a];
            }
        }

        
        mt_shader(fill_color, &[&color]);

        return Ok(());
    }}



    fn shaders(canvas : &mut WindowCanvas, rect: [i32; 4], //transformation_matrix: &[TransformationMaxtrix], 
                                                          pixel_func : &[PixelShader],
                                                          pixel_function_inputs: &Vec<Vec<Vec<f32>>> ){unsafe{
        if pixel_func.len() != pixel_function_inputs.len(){
            panic!("shaders lengths are not the same.");
        }

        //if thread::current().name().expect("??") != "1"{
        //    return;
        //}

        let [mut x, mut y, mut w, mut h] = rect;
        let c_w = canvas.w;
        let c_h = canvas.h;

        if x == -1 
        && y == x
        && w == x
        && h == x {
            x = 0;
            y = 0;
            w = c_w;
            h = c_h;
        }
        let min_c_w_h = c_w.min(c_h) as f32;
        let f32_c_w = c_w as f32;
        let f32_c_h = c_h as f32;



        let buffer = canvas.buffer as *mut u32;
        for i in 0..w {
            if i + x > c_w {
                continue;
            }
            for j in 0..h {
                if j + y > c_h {//TODO window canvas bounderies
                    continue;
                }

                let dst_rgb = buffer.offset( (i+x + c_w*(j+y)) as isize);

                let mut p = [(2f32*(i+x) as f32 ) / min_c_w_h - f32_c_w/min_c_w_h, 
                             (2f32*(j+y) as f32 ) / min_c_w_h - f32_c_h/min_c_w_h] ;


                let [mut r, mut g, mut b] = [0u32; 3];

                for ii in 0..pixel_func.len(){
                    let _p = p;//transformation_matrix[ii].apply_tm(p);

                    let [_r, _g, _b, a] = pixel_func[ii](p, &pixel_function_inputs[ii]);
                    if a < 0.01 { continue; }


                    let dst_r = (*(dst_rgb as *const u8).offset(2) as f32 * (1.0 - a)) as u32;
                    let dst_g = (*(dst_rgb as *const u8).offset(1) as f32 * (1.0 - a)) as u32;
                    let dst_b = (*(dst_rgb as *const u8).offset(0) as f32 * (1.0 - a)) as u32;

                    r = (_r*a*255.0) as u32 + dst_r; 
                    g = (_g*a*255.0) as u32 + dst_g; 
                    b = (_b*a*255.0) as u32 + dst_b; 

                    *buffer.offset((i+x + c_w*(j+y)) as isize) = 0x00000000 + (r << 16) +  (g << 8) + b;
                }
            }
        }

    }}


}






