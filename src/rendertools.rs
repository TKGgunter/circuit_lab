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




use std::collections::HashMap;
static mut GLOBAL_FONTINFO : stbtt_fontinfo = new_stbtt_fontinfo();
static mut FONT_BUFFER : Option<Vec<u8>> = Some(Vec::new());
static mut FONT_GLYPH_HASHMAP : Option<HashMap<usize, HashMap<(char, u32), Vec<u8>>>> = None;
static mut CURRENT_KEY: usize = 0;

#[inline]
pub fn c3_to_c4(c3: [f32; 3], alpha: f32)->[f32; 4]{
    [c3[0], c3[1], c3[2], alpha]
}

pub fn change_font(buffer: &[u8]){unsafe{

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
    //disk this will will become a problem. If this doesn't workout may use
    //STBTT_DEF const char *stbtt_GetFontNameString(const stbtt_fontinfo *font, int *length, int platformID, int encodingID, int languageID, int nameID);
    //in the future.
    //NOTE TKG. 256 is a guess as to how many unique characters the user might use. For mixed language
    //programs with will most likely not be enough.



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
        panic!("font was not able to load.");
    }
}}



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

pub fn get_advance_string( string: &str, size: f32 )->i32{
    let mut offset = 0;
    for it in string.chars(){
        offset += get_advance(it, size);
    }
    return offset;
}

pub fn draw_char( canvas: &mut WindowCanvas, character: char, mut x: i32, mut y: i32,
             color: [f32; 4], size: f32 )->i32{unsafe{

    //Check that globalfontinfo has been set
    if GLOBAL_FONTINFO.data == null_mut() {
        //TODO we need to log this out
        println!("Global font has not been set.");
        return -1;
    }

    //construct a char buffer
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
        scale = stbtt_ScaleForPixelHeight(&GLOBAL_FONTINFO as *const stbtt_fontinfo, size);
        let baseline = (ascent as f32 * scale ) as i32;

        cwidth = (scale * (ascent - descent) as f32 ) as usize + 4;
        cheight = (scale * (ascent - descent) as f32 ) as usize + 4;

        let glyph_index = stbtt_FindGlyphIndex(&GLOBAL_FONTINFO as *const stbtt_fontinfo, character as i32);
        
        char_buffer = match &mut FONT_GLYPH_HASHMAP{
            Some(fbh)=>{ 
                match fbh.get_mut(&CURRENT_KEY){
                    Some(font)=>{
                        match font.get(&(character, size as u32)){
                            Some(gi)=>
                            {
                                gi
                            },
                            None=>{
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
    //If the character is invisible then don't render
    
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
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        if false {if is_x86_feature_detected!("sse2") {unsafe{
//panic!("TODO {}  {}", buffer as usize , buffer as usize & 15);

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

            let mut text_alpha = [0f32;4];
            for i in 0..cheight as isize{
                if i + y as isize  > gheight {continue;}
                if i + y as isize  <= 0 {continue;}

                for j in (0..cwidth as isize).step_by(4){
                    //if j + 4 + x as isize  > gwidth {continue;}
                    //if j + x as isize  <= 0 {continue;}
                    //if (j + 4 + i*gwidth + offset) > gwidth * gheight {continue;}

                    for _j in 0..4{
                        if (j + _j + i*gwidth + offset) > gwidth * gheight {continue;}

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
                        if (j as usize +_j) as usize + cwidth * (cheight - 1 - i as usize) >= char_buffer.len() { continue; }

                        text_alpha[_j] = char_buffer[(j as usize +_j) as usize + cwidth * (cheight - 1 - i as usize)] as f32;
                        let r : &mut [f32; 4] = std::mem::transmute(&mut simd_r);
                        let g : &mut [f32; 4] = std::mem::transmute(&mut simd_g);
                        let b : &mut [f32; 4] = std::mem::transmute(&mut simd_b);
                        r[_j] = text_alpha[_j];
                        g[_j] = text_alpha[_j];
                        b[_j] = text_alpha[_j];
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

                    //TODO double check this
                    let mut simd_dst_r_u32 = _mm_cvtps_epi32(simd_dst_r);
                    simd_dst_r_u32 = _mm_slli_epi32(simd_dst_r_u32, 16);

                    let mut simd_dst_g_u32 = _mm_cvtps_epi32(simd_dst_g);
                    simd_dst_g_u32 = _mm_slli_epi32(simd_dst_g_u32, 8);

                    let mut simd_dst_b_u32 = _mm_cvtps_epi32(simd_dst_b);


                    let simd_rgba = _mm_or_si128(_mm_or_si128(simd_dst_r_u32, simd_dst_g_u32), simd_dst_b_u32);
                    
                    ////TODO for some reason this crashes idk why :(
                    //let _buffer = buffer.offset( (j as isize + i*gwidth + offset) as isize ) as *mut u64;
                    //let simd_buffer = buffer.offset( (j as isize + i*gwidth + offset) as isize ) as *mut _;

                    //let _rgba = std::mem::transmute::<&__m128i, &[u64; 2]>(&simd_rgba);
                    //*_buffer = _rgba[0];
                    //*_buffer.offset(1) = _rgba[1];
                    ////_mm_store_si128(simd_buffer, simd_rgba);
                    ////_mm_store_si128(std::mem::transmute(buffer.offset( (j as isize + i*gwidth + offset) as isize )), simd_rgba);

                    
                    for _j in 0..4{
                        if j + _j + x as isize  > gwidth {continue;}
                        if j + _j + x as isize  <= 0 {continue;}

                        let _j = _j as usize;

                        let rgba : &[u32; 4] = std::mem::transmute(&simd_rgba);

                        *buffer.offset( (j+_j as isize + i*gwidth + offset) as isize) = 0x00000000 + rgba[_j];
                    }
                }
            }

            //TODO reduntant with return at the end of function.
            //we should call the same code to avoid issues.
            let mut adv : i32 = 0;
            let mut lft_br : i32 = 0; // NOTE: Maybe remove this
            stbtt_GetCodepointHMetrics(&GLOBAL_FONTINFO as *const stbtt_fontinfo, character as i32, &mut adv as *mut i32, &mut lft_br as *mut i32);
            return (adv as f32 * scale) as i32;
        }}}

        for i in 0..cheight as isize{
            if i + y as isize  > gheight {continue;}
            if i + y as isize  <= 0 {continue;}

            for j in 0..cwidth as isize{

                if (j + i*gwidth + offset) > gwidth * gheight {continue;}

                if j + x as isize  > gwidth {continue;}
                if j + x as isize  <= 0 {continue;}

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


pub fn draw_string( canvas: &mut WindowCanvas, string: &str, x: i32, y: i32,
             color: [f32; 4], size: f32 )->i32{
    let mut offset = 0;
    for it in string.chars(){
        offset += draw_char(canvas, it, x + offset, y, color, size);
    }
    return offset;
}



pub fn draw_rect( canvas: &mut WindowCanvas, rect: [i32; 4], color: [f32; 4], filled: bool ){unsafe{
    //TODO
    //- Set alpha on dst canvas use both dst and src to determine alpha

    let buffer = canvas.buffer as *mut u32;

    let c_w = canvas.w as isize;
    let c_h = canvas.h as isize;

    let x = rect[0] as isize + 1; //NOTE 1 is here to remove wrapping TODO
    let y = rect[1] as isize;
    let _x = if x < 0 { 0 } else { x };
    let _y = if y < 0 { 0 } else { y };


    let w = rect[2] as isize;
    let h = rect[3] as isize;
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

//TODO remove redundancy with from_buffer
    pub fn load_bmp(filename: &str)->TGBitmap{unsafe{
        let mut rt = TGBitmap::new(0,0);
        let mut f = File::open(filename).expect("BMP file could not be opened.");
        let mut img_buffer = Vec::new();
        f.read_to_end(&mut img_buffer).expect("Buffer could not be read.");

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

//TODO
//time me
pub fn resize_bmp(source_bmp: &TGBitmap, w: i32, h: i32)->TGBitmap{unsafe{
    let mut bmp = TGBitmap::new(w, h);
    if source_bmp.info_header.width < w{
        println!("Trash", );
    }
    if source_bmp.info_header.height < h{
        println!("Trash");
    }
    let scale_w = w as f32 / source_bmp.info_header.width as f32;
    let scale_h = h as f32 / source_bmp.info_header.height as f32;



    let source_buffer = source_bmp.rgba.as_ptr();
    let dst_buffer = bmp.rgba.as_mut_ptr() as *mut u32;

    let bytes_per_pix = (source_bmp.info_header.bit_per_pixel / 8) as isize;

    for j in 0..source_bmp.info_header.height{
        for i in 0..source_bmp.info_header.width{
            let mut _i;
            let mut _j;
            _i = (i as f32 * scale_w) as i32;
            _j = (j as f32 * scale_h) as i32;


            if _i >= w { _i = w-1; }
            if _j >= h { _j = h-1; }


            let src_rgb = source_buffer.offset(  bytes_per_pix * (i + source_bmp.info_header.width * j) as isize);
            let src_r =  *(src_rgb as *const u8).offset(2);
            let src_g =  *(src_rgb as *const u8).offset(1);
            let src_b =  *(src_rgb as *const u8).offset(0);

            let mut _scale_w = scale_w;
            let mut _scale_h = scale_h;
            fn get_correct_scale_for_pixel(original_index: i32, scale: f32)->f32{
                let mut post_index  = scale * (original_index as f32);
                let mut _it = post_index;
                if ((post_index - post_index.trunc()) / scale).trunc() >= 1.0{
                    _it -= 1.0 * ((post_index - post_index.trunc()) / scale).trunc() * scale;
                }
                return  1.0/ (  (((1.0+_it).trunc() - _it ) / scale).trunc() + 1.0) ;
            }
            _scale_h = get_correct_scale_for_pixel(j, scale_h);
            _scale_w = get_correct_scale_for_pixel(i, scale_w);
            ///////////////////////////////

            let r = (src_r as f32 * _scale_w * _scale_h) as u32;
            let g = (src_g as f32 * _scale_w * _scale_h) as u32;
            let b = (src_b as f32 * _scale_w * _scale_h) as u32;

            *dst_buffer.offset( (_i + w * _j) as isize ) += 0x00000000 + (r << 16) + (g << 8) + b;
        }
    }
    return bmp;
}}

pub fn sampling_reduction_bmp(source_bmp: &TGBitmap, w: i32, h: i32)->TGBitmap{unsafe{
    let mut bmp = TGBitmap::new(w, h);
    if source_bmp.info_header.width < w{
        println!("Trash", );
    }
    if source_bmp.info_header.height < h{
        println!("Trash");
    }
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


pub fn draw_stbi_image( canvas: &mut WindowCanvas, bmp: &StbiImage, x: i32, y: i32, alpha: f32,
            _w: Option<i32>, _h: Option<i32>){unsafe{

    if alpha < 0.0 {
        println!("A negative alpha as passed to drawBMP");
        return;
    }
    let w;
    let h;

    match _w {
        Some(int) => w = int,
        None => w = bmp.width,
    }
    match _h {
        Some(int) => h = int,
        None => h = bmp.height,
    }

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


pub fn draw_bmp( canvas: &mut WindowCanvas, source_bmp: &TGBitmap, x: i32, y: i32, alpha: f32,
            _w: Option<i32>, _h: Option<i32>){unsafe{

    if alpha < 0.0 {
        println!("A negative alpha as passed to drawBMP");
        return;
    }
    let w;
    let h;

    match _w {
        Some(int) => w = int,
        None => w = source_bmp.info_header.width,
    }
    match _h {
        Some(int) => h = int,
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
                //when alpha is one copy the bmp bits instead of iterating through the array
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




//TODO time test is needed. It is very likely this function is slow.
//move to rendertools
pub fn draw_circle(canvas: &mut WindowCanvas, _x: i32, _y: i32, r: f32, color: [f32; 4]){unsafe{
    let buffer = canvas.buffer as *mut u32;

    let c_w = canvas.w as isize;
    let c_h = canvas.h as isize;

    let x = (_x - r as i32) as isize;
    let y = (_y - r as i32)as isize;
    let w = (2.0*r) as isize;
    let h = (2.0*r) as isize;


    let a = color[3];

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






//TODO if alpha is 1
// program crashes and memcopy bleeds.
pub fn draw_subcanvas(canvas: &mut WindowCanvas, subcanvas: &SubCanvas, x: i32, y: i32, alpha: f32 ){unsafe{

    let buffer = canvas.buffer as *mut u32;
    let subbuffer = subcanvas.canvas.buffer as *mut u32;

    let c_w = canvas.w as isize;
    let c_h = canvas.h as isize;

    let x = x as isize;
    let y = y as isize;
    let w = subcanvas.canvas.w as isize;
    let h = subcanvas.canvas.h as isize - 1;//-1 is here to remove junk. We should look more closely at this TODO
    let a = alpha;


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
            //println!("{} {} {}", j, j_, c_h);

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

    //TODO combine pixel shader and distance shader, I don't like operating like this 
    //TODO enum for render data?
    type PixelShader = fn(f32, [f32; 2], &[f32])->[f32;4];
    static mut PIXEL_SHADER_FUNCTION_PIPELINE : Option<Vec<PixelShader>> = None;
    static mut PIXEL_SHADER_DATA_PIPELINE : Option<Vec<Vec<f32>>> = None;

    type DistanceShader = fn([f32; 2], &[f32])->f32;
    static mut DISTANCE_SHADER_FUNCTION_PIPELINE : Option<Vec<DistanceShader>> = None;
    static mut DISTANCE_SHADER_DATA_PIPELINE : Option<Vec<Vec<f32>>> = None;

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
        println!("Number of Pixel Shaders: {}", pixel_shaders.len());

        if DISTANCE_SHADER_FUNCTION_PIPELINE.is_none(){
            println!("Distance shader pipeline has not been set.");
            return;
        }
        let distance_shaders = DISTANCE_SHADER_FUNCTION_PIPELINE.as_ref().unwrap();
        println!("Number of Distance Shaders: {}", distance_shaders.len());
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

                    let distance_functions = unsafe{ DISTANCE_SHADER_FUNCTION_PIPELINE.as_ref().unwrap() };
                    let distance_data = unsafe{ DISTANCE_SHADER_DATA_PIPELINE.as_ref().unwrap() };
                    let pixel_functions = unsafe{ PIXEL_SHADER_FUNCTION_PIPELINE.as_ref().unwrap() };
                    let pixel_data = unsafe{ PIXEL_SHADER_DATA_PIPELINE.as_ref().unwrap() };

                    shaders(canvas, rect, //transformation_matrix: &[TransformationMaxtrix], 
                                                           distance_functions,
                                                           distance_data,
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
                h = window_height - _h*(i-1) as i32;
            }


            let mut builder = thread::Builder::new().name(i.to_string());
            arr_threads.push( builder.spawn(thread_function).expect("Thread could not be made.") );
            arr_thread_rect.push([0, _h*i as i32,window_width, h]);

        }
        THREAD_POOL = Some(arr_threads);
        THREAD_BOUNDING_RECT  = Some(arr_thread_rect);

        PIXEL_SHADER_FUNCTION_PIPELINE = Some(vec![]);
        PIXEL_SHADER_DATA_PIPELINE     = Some(vec![]); 


        DISTANCE_SHADER_FUNCTION_PIPELINE = Some(vec![]);
        DISTANCE_SHADER_DATA_PIPELINE     = Some(vec![]);


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

        DISTANCE_SHADER_FUNCTION_PIPELINE.as_mut().unwrap().clear();
        DISTANCE_SHADER_DATA_PIPELINE.as_mut().unwrap().clear();

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



    pub fn mt_shader(distance_function: DistanceShader, d_data: &[f32], pixel_function: PixelShader, p_data: &[f32])->Result<(), MTError>{unsafe{
        if THREAD_STATUS.is_none(){
            return Err(MTError::Err);
        }
        
        PIXEL_SHADER_FUNCTION_PIPELINE.as_mut().unwrap().push(pixel_function);
        PIXEL_SHADER_DATA_PIPELINE.as_mut().unwrap().push(p_data.to_vec());

        DISTANCE_SHADER_FUNCTION_PIPELINE.as_mut().unwrap().push(distance_function);
        DISTANCE_SHADER_DATA_PIPELINE.as_mut().unwrap().push(d_data.to_vec());
        
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
        fn fill_color(d:f32,  p: [f32; 2], inputs: &[f32])->[f32; 4]{
            if inputs.len() < 4 {
                panic!("fill_color did not get proper number of inputs");
            }
            let r = inputs[0];
            let g = inputs[1];
            let b = inputs[2];
            let a = inputs[3];

            if d.is_finite(){
                return [r,g,b,a*((-d*10f32).exp()).max(0.0).min(1.0)];
            } else {
                return [r,g,b,a];
            }
        }

        
        mt_shader(d_rect, &f_rect, fill_color, &color);

        return Ok(());
    }}



    fn shaders(canvas : &mut WindowCanvas, rect: [i32; 4], //transformation_matrix: &[TransformationMaxtrix], 
                                                           distance_function : &[DistanceShader],
                                                           distance_function_inputs: &[Vec<f32>], 
                                                          pixel_func : &[PixelShader],
                                                          pixel_function_inputs: &[Vec<f32>] ){unsafe{
        if distance_function.len() != distance_function_inputs.len()
        || distance_function.len() != pixel_func.len()
        || distance_function.len() != pixel_function_inputs.len(){
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

                for ii in 0..distance_function.len(){
                    let _p = p;//transformation_matrix[ii].apply_tm(p);
                    let d = distance_function[ii](_p, &distance_function_inputs[ii]);

                    let [_r, _g, _b, a] = pixel_func[ii](d, p, &pixel_function_inputs[ii]);
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






