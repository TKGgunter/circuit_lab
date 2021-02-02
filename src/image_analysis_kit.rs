
use crate::rendertools::TGBitmap;


//NOTE thoth gunter 3/14/2020
//this is a const because I 
//want to keep track of when and
//how this value changes
pub const DR_CUT: f32 = 0.1;


pub fn image_chi_squared(source_im: &TGBitmap, im2: &TGBitmap)->f32{
    if source_im.rgba.len() != im2.rgba.len() {
        println!("images are not the same size");//TODO log me
        return std::f32::MAX;
    }

    //TODO we can make this much much faster atleast in debug build
    let mut sigma = [1.0f32; 4];
    let mut _sum = [0.0f32; 4];
    for i in 0..( source_im.width * source_im.height) as usize{
        _sum[0] += im2.rgba[4*i] as f32;
        _sum[1] += im2.rgba[4*i + 1] as f32;
        _sum[2] += im2.rgba[4*i + 2] as f32;
        _sum[3] += im2.rgba[4*i + 3] as f32;

        sigma[0] += (im2.rgba[4*i] as f32 - _sum[0] / (i as f32 + 1.0) ).powf(2.0);
        sigma[1] += (im2.rgba[4*i + 1] as f32 - _sum[1] / (i as f32 + 1.0) ).powf(2.0);
        sigma[2] += (im2.rgba[4*i + 2] as f32 - _sum[2] / (i as f32 + 1.0) ).powf(2.0);
        sigma[3] += (im2.rgba[4*i + 3] as f32 - _sum[3] / (i as f32 + 1.0) ).powf(2.0);
    }
    sigma[0] /= ( source_im.width * source_im.height ) as f32;
    sigma[1] /= ( source_im.width * source_im.height ) as f32;
    sigma[2] /= ( source_im.width * source_im.height ) as f32;
    sigma[3] /= ( source_im.width * source_im.height ) as f32;

    let mut sum = [0.0f32; 4];
    for i in 0..( source_im.width * source_im.height) as usize{
        sum[0] += (source_im.rgba[4*i] - im2.rgba[4*i]).pow(2) as f32 / sigma[0] ;
        sum[1] += (source_im.rgba[4*i + 1] - im2.rgba[4*i + 1]).pow(2) as f32 / sigma[1] ;
        sum[2] += (source_im.rgba[4*i + 2] - im2.rgba[4*i + 2]).pow(2) as f32 / sigma[2] ;
        sum[3] += (source_im.rgba[4*i + 3] - im2.rgba[4*i + 3]).pow(2) as f32 / sigma[3] ;
    }                                                 
    return (sum[0] + sum[1] + sum[2] + sum[3]).sqrt();
}


#[derive(Debug, Clone, Copy)]
pub struct ColorVector{
    pub r : f32,
    pub g : f32,
    pub b : f32,

    pub avg_intensity : f32,
    pub count         : i32,
}

impl ColorVector{
    pub fn init(r: u8, g: u8, b: u8)->ColorVector{
        let _r = r as f32;
        let _g = g as f32;
        let _b = b as f32;
        let intensity = (_r*_r + _g*_g + _b*_b).sqrt() + 0.01;
        ColorVector{
            //TODO
            //doesn't handle low intensity well
            //if colors are right around black it gets handled poorly
            r : _r / intensity, 
            g : _g / intensity, 
            b : _b / intensity, 

            avg_intensity : intensity / (3.0f32.sqrt() * 255.0), 
            count : 1,
        }
    }
    pub fn rgba(&self)->[f32;4]{
        [self.r, self.g, self.b, 1.0]
    }
}


pub struct ImageAnalysisStorage{
    pub init: bool,
    pub bkg_colorvector_set : Vec<ColorVector>,
    pub ch1_colorvector_set : Vec<ColorVector>,
    pub ch2_colorvector_set : Vec<ColorVector>,

    pub ch1_rect : [i32; 4],
    pub ch2_rect : [i32; 4],
}

impl ImageAnalysisStorage{
    pub fn new()->ImageAnalysisStorage{
        ImageAnalysisStorage{
            init: false,
            bkg_colorvector_set :  Vec::with_capacity(100),
            ch1_colorvector_set :  Vec::with_capacity(100),
            ch2_colorvector_set :  Vec::with_capacity(100),

            ch1_rect : [0i32; 4],
            ch2_rect : [0i32; 4],
        }
    }
}



pub struct RGBA {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8 
}


pub fn update_colorvector_set( rect: [usize;4], slice: &[RGBA], width: i32, height: i32, color_vector_set: &mut Vec<ColorVector>, cut_dr: f32){
    let x1  = rect[0];
    let y1  = rect[1];
    let x2  = rect[0] + rect[2];
    let y2  = rect[1] + rect[3];

    for j in y1..y2 as usize {
        for i in x1..x2 as usize {
            if j*width as usize + i > (width*height) as usize {  panic!("update_colorvector_set attempt calc outside of range."); }

            let c = &slice[ j*width as usize + i];
            let c_v = ColorVector::init(c.r, c.g, c.b);

            let mut in_set = false; 
            let mut in_index = 0;
            for (_i,it) in color_vector_set.iter_mut().enumerate(){
                let dr = delta_colorvector(&c_v, it);
  
                if dr < cut_dr 
                && (it.avg_intensity - c_v.avg_intensity).abs() < 0.15 //TODO thinking about this
                {
                    in_set   = true; 
                    in_index = _i; 
                    break; 
                }
            }
            if !in_set { 
                color_vector_set.push(c_v); 
            } else {  
                let intensity = color_vector_set[in_index].avg_intensity * color_vector_set[in_index].count as f32;
                color_vector_set[in_index].count += 1;  
                color_vector_set[in_index].avg_intensity = ( intensity + c_v.avg_intensity ) / color_vector_set[in_index].count as f32;
            }
        }
    }
}

#[inline]
fn delta_colorvector( c1: &ColorVector, c2: &ColorVector)->f32{
//TODO
//doesn't handle low intensity well
//if colors are right around black it gets handled poorly
    let d_r = c1.r - c2.r;
    let d_g = c1.g - c2.g;
    let d_b = c1.b - c2.b;


    let dr = ( d_r*d_r + 
               d_g*d_g + 
               d_b*d_b ).sqrt();
    
    return dr;
}




pub fn remove_bkg_from_signal(tot_bkg_pxs: i32, bkg: &Vec<ColorVector>, signal: &mut Vec<ColorVector>){
//NOTE
//Remove background vectors from signal sample

        let mut retain_indices = Vec::with_capacity(signal.len());

        for jt in signal.iter(){

            let mut color_good = true;
            for it in bkg.iter(){

                if delta_colorvector(it, jt) < 0.15 && it.count > 400 //TODO this is going to be wrong
                && (it.avg_intensity - jt.avg_intensity).abs() < 10.0 
                && it.count/3 > jt.count
                { color_good = false; 
                  break;
                } 

                if delta_colorvector(it, jt) < 0.15 && it.count  > tot_bkg_pxs / 10  
                { color_good = false; 
                  break;
                } 

            }
            retain_indices.push(color_good);  

        }

        let mut i=0 ;
        signal.retain(|_| (retain_indices[i], i+=1).0);

}


struct DbscanPt {
    index: usize,
    class: i8,
}



fn dbscan(set_vec: &mut Vec<DbscanPt>, eps: usize, min_pts: usize)->i8{
    //TODO
    //write documentation
    fn rangeQuery(set_vec: &mut Vec<DbscanPt>, pt: usize, eps: usize)->Vec<usize>{
        let mut n = Vec::new();
        for (i, it) in set_vec.iter().enumerate(){
            if it.index == pt { continue; }
            if (it.index as isize - pt as isize).abs() <= eps as isize{
                n.push((i).clone());
            }
        }
        return n;
    }
    //0 == Noise
    //-1== None

    let mut class : i8 = 0;
    for i in 0..set_vec.len(){
        if set_vec[i].class != -1{
            continue;
        } 
        let index = set_vec[i].index;
        let mut neighbor_set = rangeQuery(set_vec, index, eps);
        if neighbor_set.len() < min_pts{
            set_vec[i].class = 0;
            continue;
        }
        
        class += 1;
        set_vec[i].class = class;
        let mut j = 0;
        while j < neighbor_set.len(){
            if set_vec[neighbor_set[j]].class == 0 {set_vec[neighbor_set[j]].class = class;}
            if set_vec[neighbor_set[j]].class != -1 { j+=1; continue;}
            set_vec[neighbor_set[j]].class = class;

            let _index = set_vec[neighbor_set[j]].index;
            let mut _n_set = rangeQuery(set_vec, _index, eps);
            if _n_set.len() >= min_pts{

                for _i in 0.._n_set.len(){
                    let mut in_set = false;
                    for _j in 0..neighbor_set.len(){
                        if _n_set[_i] == neighbor_set[_j]{
                            in_set = true;
                            break;
                        }
                    }
                    if !in_set{
                        neighbor_set.push(_n_set[_i].clone());
                    }
                }
            }
            j +=1;
        }
    }
    return class;
}
fn get_min_index( set: &Vec<DbscanPt>, class: i8)->usize{ //NOTE lets assume things are ordered
    //TODO
    //maybe return a option
    for it in set.iter(){
        if it.class == class{
            return it.index; 
        }
    }
    return std::usize::MAX;
}
fn get_max_index( set: &Vec<DbscanPt>, class: i8)->usize{ //NOTE lets assume things are ordered
    //TODO
    //maybe return a option
    let mut index = 0;
    for it in set.iter(){
        if it.class == class{
            if index < it.index{
               index = it.index; 
            }
        }
    }
    return index;
}

use std::collections::HashMap;

pub struct ManyHist{//TODO rename maybe
    pub vertical_histogram_rm_bkg   : Vec<usize>,
    pub horizontal_histogram_rm_bkg : Vec<usize>,

    pub vertical_histogram_rm_bkg_ch1   : Vec<usize>,
    pub horizontal_histogram_rm_bkg_ch1 : Vec<usize>, 
    pub arr_horizontal_histogram_rm_bkg_ch1 : Vec<Vec<usize>>,

    pub vertical_histogram_rm_bkg_ch2       : Vec<usize>,
    pub arr_horizontal_histogram_rm_bkg_ch2 : Vec<Vec<usize>>,
    pub horizontal_histogram_rm_bkg_ch2     : Vec<usize>,

    //TODO
    //this needs to be an array of hash tables kinda
    pub counts_vertical_color_vec_ch1   : Vec<HashMap<usize,usize>>,
    pub counts_horizontal_color_vec_ch1 : Vec<Vec<HashMap<usize,usize>>>,
    pub counts_vertical_color_vec_ch2   : Vec<HashMap<usize,usize>>,
    pub counts_horizontal_color_vec_ch2 : Vec<Vec<HashMap<usize,usize>>>,
}

impl ManyHist{
    pub fn new()->ManyHist{
        ManyHist{
            vertical_histogram_rm_bkg   : Vec::new(),
            horizontal_histogram_rm_bkg : Vec::new(),

            vertical_histogram_rm_bkg_ch1   : Vec::new(),
            horizontal_histogram_rm_bkg_ch1 : Vec::new(), 
            arr_horizontal_histogram_rm_bkg_ch1 : Vec::new(),

            vertical_histogram_rm_bkg_ch2       : Vec::new(),
            arr_horizontal_histogram_rm_bkg_ch2 : Vec::new(),
            horizontal_histogram_rm_bkg_ch2     : Vec::new(),

            counts_vertical_color_vec_ch1   : Vec::new(),
            counts_horizontal_color_vec_ch1 : Vec::new(),
            counts_vertical_color_vec_ch2   : Vec::new(),
            counts_horizontal_color_vec_ch2 : Vec::new(),
        }
    }
}


pub fn image_hists(input_bmp: &TGBitmap, bkg_colorvector_set: &Vec<ColorVector>, ch1_colorvector_set: &Vec<ColorVector>, ch2_colorvector_set: &Vec<ColorVector>, out_bmp: &mut Option<TGBitmap>)->ManyHist{

    fn helper_update_myhist(x: &Option<usize>, y: &mut Vec<HashMap<usize, usize>>, i: usize){
        if x.is_some(){
            let k = x.unwrap();
            let v = y[i].get_mut(&k);

            if v.is_some(){
                *v.unwrap() += 1;
            } else {
                y[i].insert(k, 1);
            }
        }
    }


    let mut vertical_histogram_rm_bkg   = vec![0; input_bmp.width as usize];
    let mut horizontal_histogram_rm_bkg = vec![0; input_bmp.height as usize];

    let mut vertical_histogram_rm_bkg_ch1   = vec![0; input_bmp.width as usize];
    let mut horizontal_histogram_rm_bkg_ch1 = vec![0; input_bmp.height as usize];
    let mut arr_horizontal_histogram_rm_bkg_ch1 = vec![vec![0; input_bmp.height as usize]; 4];
    //TODO
    //this needs to be an array of hash tables kinda
    let mut counts_vertical_color_vec_ch1   = vec![ HashMap::new(); input_bmp.width as usize];
    let mut counts_horizontal_color_vec_ch1 = vec![ vec![HashMap::new(); input_bmp.height as usize]; 4];


    let mut vertical_histogram_rm_bkg_ch2       = vec![0; input_bmp.width as usize];
    let mut arr_horizontal_histogram_rm_bkg_ch2 = vec![vec![0; input_bmp.height as usize]; 4];
    let mut horizontal_histogram_rm_bkg_ch2     = vec![0; input_bmp.height as usize];
    //TODO
    //this needs to be an array of hash tables kinda
    let mut counts_vertical_color_vec_ch2   = vec![ HashMap::new(); input_bmp.width as usize];
    let mut counts_horizontal_color_vec_ch2 = vec![ vec![HashMap::new(); input_bmp.height as usize]; 4];


    let mut ch_max_count = ch1_colorvector_set[0].count; 
    if ch2_colorvector_set[0].count > ch_max_count{ ch_max_count = ch2_colorvector_set[0].count; }

    //Both background and foreground
    for i in 0..input_bmp.width as usize {
        for j in 0..input_bmp.height as usize {
            let index = 4*( i + j*input_bmp.width as usize);
            let r = input_bmp.rgba[ index + 2];
            let g = input_bmp.rgba[ index + 1];
            let b = input_bmp.rgba[ index + 0];
            let cv = ColorVector::init(r, g, b);

            ////////////////////////
            let mut was_background = false;
            for it in bkg_colorvector_set.iter(){
                if it.count > ch_max_count{
                    if delta_colorvector(it, &cv) < 0.1
                    && (it.avg_intensity - cv.avg_intensity).abs() < 0.25{
                        was_background = true;
                        break;
                    }
                } else {
                    break;
                }
            }
            ////////////////////////
            let mut is_ch1 = false;
            let mut is_ch2 = false;

            let mut ch1_color_vec = None;
            let mut ch2_color_vec = None;

            if !was_background{

                let mut is_good = false;

                let mut max_count = 0;
                for (_jt, jt) in ch1_colorvector_set.iter().enumerate(){
                    if delta_colorvector(jt, &cv) < 0.1 
                    && (jt.avg_intensity - cv.avg_intensity).abs() < 0.15{

                        is_good = true;
                        is_ch1 = true;
                        max_count = jt.count;
                        ch1_color_vec = Some(_jt);
                        break;
                    }
                }
                for (_jt, jt) in ch2_colorvector_set.iter().enumerate(){
                    if delta_colorvector(jt, &cv) < 0.1 
                    && (jt.avg_intensity - cv.avg_intensity).abs() < 0.15{
                        is_good = true;
                        is_ch2 = true;
                        if max_count < jt.count {
                            max_count = jt.count;
                        }
                        ch2_color_vec = Some(_jt);
                        break;
                    }
                }

                was_background = false;
                for it in bkg_colorvector_set.iter(){
                    //TODO this is confusing 
                    //plz clean up
                    //TODO slight speed up can made by starting at index where 
                    //counts background counts are low
                    let mut temp_is_good = is_good;
                    if it.count > max_count{ temp_is_good = false; }
                    if delta_colorvector(it, &cv) < 0.1 && !temp_is_good
                    && (it.avg_intensity - cv.avg_intensity).abs() < 0.25{

                        was_background = true;
                        break;
                    }
                }
            }
            if !was_background {
                if !is_ch1 && !is_ch2{
                    vertical_histogram_rm_bkg[i] += 1;
                    horizontal_histogram_rm_bkg[j] += 1; 
                }

                if is_ch1 {
                    vertical_histogram_rm_bkg_ch1[i] += 1;
                    horizontal_histogram_rm_bkg_ch1[j] += 1;

                    helper_update_myhist(&ch1_color_vec, &mut counts_vertical_color_vec_ch1, i);


                    if i < input_bmp.width as usize / 4{
                        arr_horizontal_histogram_rm_bkg_ch1[0][j] += 1;
                        helper_update_myhist(&ch1_color_vec, &mut counts_horizontal_color_vec_ch1[0], j);

                    } else if i < input_bmp.width as usize / 2{
                        arr_horizontal_histogram_rm_bkg_ch1[1][j] += 1;
                        helper_update_myhist(&ch1_color_vec, &mut counts_horizontal_color_vec_ch1[1], j);

                    } else if i < input_bmp.width as usize * 3 / 4 {
                        arr_horizontal_histogram_rm_bkg_ch1[2][j] += 1;
                        helper_update_myhist(&ch1_color_vec, &mut counts_horizontal_color_vec_ch1[2], j);

                    } else {
                        arr_horizontal_histogram_rm_bkg_ch1[3][j] += 1;
                        helper_update_myhist(&ch1_color_vec, &mut counts_horizontal_color_vec_ch1[3], j);

                    }
                }

                if is_ch2{
                    vertical_histogram_rm_bkg_ch2[i] += 1;
                    horizontal_histogram_rm_bkg_ch2[j] += 1;

                    helper_update_myhist(&ch2_color_vec, &mut counts_vertical_color_vec_ch2, i);


                    if i < input_bmp.width as usize / 4{
                        arr_horizontal_histogram_rm_bkg_ch2[0][j] += 1;
                        helper_update_myhist(&ch2_color_vec, &mut counts_horizontal_color_vec_ch2[0], j);

                    } else if i < input_bmp.width as usize / 2{
                        arr_horizontal_histogram_rm_bkg_ch2[1][j] += 1;
                        helper_update_myhist(&ch2_color_vec, &mut counts_horizontal_color_vec_ch2[1], j);

                    } else if i < input_bmp.width as usize * 3 / 4 {
                        arr_horizontal_histogram_rm_bkg_ch2[2][j] += 1;
                        helper_update_myhist(&ch2_color_vec, &mut counts_horizontal_color_vec_ch2[2], j);

                    } else {
                        arr_horizontal_histogram_rm_bkg_ch2[3][j] += 1;
                        helper_update_myhist(&ch2_color_vec, &mut counts_horizontal_color_vec_ch2[3], j);

                    }


                }
            }
        }
    }

    return ManyHist{
        vertical_histogram_rm_bkg   : vertical_histogram_rm_bkg,
        horizontal_histogram_rm_bkg : horizontal_histogram_rm_bkg,

        vertical_histogram_rm_bkg_ch1   : vertical_histogram_rm_bkg_ch1,
        horizontal_histogram_rm_bkg_ch1 : horizontal_histogram_rm_bkg_ch1, 
        arr_horizontal_histogram_rm_bkg_ch1 : arr_horizontal_histogram_rm_bkg_ch1,

        vertical_histogram_rm_bkg_ch2       : vertical_histogram_rm_bkg_ch2,
        arr_horizontal_histogram_rm_bkg_ch2 : arr_horizontal_histogram_rm_bkg_ch2,
        horizontal_histogram_rm_bkg_ch2     : horizontal_histogram_rm_bkg_ch2,

        counts_vertical_color_vec_ch1   : counts_vertical_color_vec_ch1,
        counts_horizontal_color_vec_ch1 : counts_horizontal_color_vec_ch1,
        counts_vertical_color_vec_ch2   : counts_vertical_color_vec_ch2,
        counts_horizontal_color_vec_ch2 : counts_horizontal_color_vec_ch2,
    };
}






//TODO
//+ add verbose
//+ return each dimensions mean and median locations (this should be relative to the bounding box)
pub fn bound_character(bias: [i32; 4], vert_hist: &Vec<usize>, horz_hist: &Vec<Vec<usize>>, 
                                       vert_hist_counts: &Vec<HashMap<usize, usize>>, horz_hist_counts: &Vec<Vec<HashMap<usize, usize>>>, 
                                       colorvector_set: &Vec<ColorVector>, width: i32, height: i32, verbose: bool)->Option<[i32;4]>{
    let mut rt = [0; 4];

    let mut tot_count_set = 0;
    for it in colorvector_set.iter(){
        tot_count_set += it.count;
    }


    {//Determine the x position and width
        let mut mean = 0.0;
        for it in vert_hist.iter(){
            mean += *it as f32 / height as f32;
        }
        mean /= vert_hist.len() as f32;


        let mut dbscan_vec = Vec::new();
        for (i, it) in vert_hist.iter().enumerate(){
            let _y_offset = *it as f32 / height as f32;
            if _y_offset >  mean {
                dbscan_vec.push(DbscanPt{index: i, class: -1});
            }
        }


        let number_of_classes = dbscan(&mut dbscan_vec, 8, 7);
        
        let mut good_class = -1  ;
        let mut min_score = std::f32::MAX;
        let mut good_coor = [0, 0i32];
        let mut old_sum_hist = 0.0;


        for i in 1..number_of_classes+1{
            let _max = get_max_index(&dbscan_vec, i) as i32;
            let _min = get_min_index(&dbscan_vec, i) as i32;

            let coor_and_width  = [_min, _max - _min]; 
            if coor_and_width[1] < width/10{
                if verbose{
                    println!("continued: {}", i);
                }
                continue;
            }

            let sub_window_width = coor_and_width[1] as f32;
            let mut counts = vec![0; colorvector_set.len()];
            for (i, it) in vert_hist[_min as usize.._max as usize].iter().enumerate(){
                for j in 0..counts.len(){
                    counts[j] += *vert_hist_counts[_min as usize +i].get(&j).unwrap_or(&0);
                }
            }

            
            //TODO explain quality score
            let posx_score = ((bias[0]-coor_and_width[0])  ) as f32;
            let width_score = ((bias[2]-coor_and_width[1]) ) as f32;

            let mut score = posx_score.powf(2.0) +  width_score.powf(2.0);
            let set_len = colorvector_set.len();
            let mut set_score = 0.0f32;
            for (i_set, iter_set) in colorvector_set.iter().enumerate(){
                //TODO
                let _a = iter_set.count as f32 / (counts[i_set] as f32 + 1.0);
                let frac_weight = iter_set.count as f32 / tot_count_set as f32;

                let _set_score = ( frac_weight * (1.0 - _a).powf(2.0) / set_len as f32).powf(2.0);
                set_score += _set_score;
            }

            score += set_score;
            score = score.sqrt();
            
            if verbose{
                println!("i {} {} {} || score:{} set_score: {}", i, _min, _max,  score, set_score);
                for (i_set, iter_set) in colorvector_set.iter().enumerate(){
                    println!("\t{} {} {} {:?}", i_set, counts[i_set], iter_set.count, &iter_set.rgba());
                }
            }

            if score < min_score{ 
                min_score = score; 
                good_class = i;
                good_coor = coor_and_width;
            }  else {
            }
        }
        //TODO
        if good_class == -1 { println!("\nWe could not find the character! \nHandle Later \n Number of classes {}.", number_of_classes); return None;} 

        rt[0] = good_coor[0];
        rt[2] = good_coor[1];
    }

    {//Determine the y position and height
        let mut means = [0.0; 4];
        let mut stds  = [0.0f32; 4]; //TODO think about this
        let mut min_mean = std::f32::MAX;

        for (i, it) in horz_hist.iter().enumerate(){ 
            for (j, jt) in it.iter().enumerate(){ 
                means[i] +=  *jt as f32 / width as f32;
                stds[i]  += j as f32 / horz_hist[0].len() as f32 * ( *jt as f32 / width as f32  - means[i]/horz_hist[0].len() as f32).powf(2.0);
            }
            means[i] /= horz_hist[0].len() as f32;
            stds[i] = stds[i].sqrt() / horz_hist[0].len() as f32;

            if means[i] < min_mean {
                min_mean = means[i]; 
            }
        }


        
        //NOTE 2/2/2020 Thoth Gunter
        //We sub divide the horizontal histograms into smaller slices.
        //We do this to be sure our signal does not be buried by noise.
        //However this process causes a problem. For very wide characters
        //We must look across multiple windows. The algorithm as it is 
        //Only looks at 2 windows.  If the character spans more than two windows
        //we are not smart enough to pick the most interesting window. 
        //This should be a rare problem but a problem non the less. 
        //I'd imagine this is more of a problem for characters facing toward the left; 
        //we should track if possible.

        //TODO
        //we need a way to debug this stuff we are 
        let window_width = width as usize / horz_hist.len();
        let mut index1 = std::usize::MAX;
        let mut index2 = std::usize::MAX;
        for i in 0..horz_hist.len(){
            if (rt[0] as usize) < (i+1)*window_width && rt[0] as usize >= i*window_width{
                index1 = i; 
            }
            let p2 = (rt[0] + rt[2]) as usize; 
            if p2 < (i+1)*window_width &&  p2 >= i*window_width{
                index2 = i; 
            }
        }
        if index1 == std::usize::MAX { println!("No good horizontal hist window found 1. {:?}", rt); return None;} 
        if index2 == std::usize::MAX { println!("No good horizontal hist window found 2. {:?}", rt); return None;} 



        let mut dbscan_vec_index1 = Vec::new();
        let mut dbscan_vec_index2 = Vec::new();
        for (i, it) in horz_hist[index1].iter().enumerate(){
            let _x_offset = *it as f32 / width as f32;

            if _x_offset >  min_mean  {
                dbscan_vec_index1.push(DbscanPt{index: i, class: -1});
            }
        }
        if index1 != index2 && index2 != std::usize::MAX {
            for (i, it) in horz_hist[index2].iter().enumerate(){
                let _x_offset = (*it as f32 / width as f32 );

                if _x_offset >  min_mean  {
                    dbscan_vec_index2.push(DbscanPt{index: i, class: -1});
                }
            }
        }
        let number_of_classes1 = dbscan(&mut dbscan_vec_index1, 10, 9);
        let number_of_classes2 = dbscan(&mut dbscan_vec_index2, 10, 9);


        let mut good_class1 = -1  ;
        let mut min_score1 = std::f32::MAX;
        let mut good_coor1 = [0, 0i32];

        for i in 1..number_of_classes1+1{
            let _max = get_max_index(&dbscan_vec_index1, i) as i32;
            let _min = get_min_index(&dbscan_vec_index1, i) as i32;

            let coor_and_height  = [_min, _max - _min]; 
            if coor_and_height[1] < 50{
                continue;
            }

            let sub_window_height = coor_and_height[1] as f32;
            let mut counts = vec![0; colorvector_set.len()];
            for (i, it) in horz_hist[index2][_min as usize.._max as usize].iter().enumerate(){
                for j in 0..counts.len(){
                    counts[j] += *vert_hist_counts[_min as usize + i].get(&j).unwrap_or(&0);
                }
            }

            let _width = width as f32 / horz_hist.len() as f32;
            let pos_score    = (bias[1]-coor_and_height[0]) as f32;
            let height_score = (bias[3]-coor_and_height[1]) as f32;
            let mut score = pos_score.powf(2.0) + height_score.powf(2.0);

            let set_len = colorvector_set.len();
            let mut set_score = 0.0;
            for (i_set, iter_set) in colorvector_set.iter().enumerate(){
                //TODO
                let _a = iter_set.count as f32 / (counts[i_set] as f32 + 1.0);
                let frac_weight = iter_set.count as f32 / tot_count_set as f32;

                set_score += ( frac_weight * (1.0 - _a).powf(2.0) / set_len as f32).powf(2.0);
            }
            score += set_score;
            score = score.sqrt();



            if min_score1 > score{ 
                min_score1 = score; 
                good_class1 = i;
                good_coor1 = [_min, _max-_min];
            } 
        }
        if good_class1 == -1 { println!("\nWe could not find the character part1! \nHandle Later \n Number of classes {} {}.", number_of_classes1, dbscan_vec_index1.len()); return None;} 



        let mut good_class2 = -1  ;
        let mut min_score2 = std::f32::MAX;
        let mut good_coor2 = [0, 0i32];
        for i in 1..number_of_classes2+1{
            let _max = get_max_index(&dbscan_vec_index2, i) as i32;
            let _min = get_min_index(&dbscan_vec_index2, i) as i32;

            let coor_and_height  = [_min, _max - _min]; 
            if coor_and_height[1] < 50{
                continue;
            }

            let sub_window_height = coor_and_height[1] as f32;
            let mut counts = vec![0; colorvector_set.len()];
            for (i, it) in horz_hist[index2][_min as usize.._max as usize].iter().enumerate(){
                for j in 0..counts.len(){
                    counts[j] += *vert_hist_counts[_min as usize + i].get(&j).unwrap_or(&0);
                }
            }


            let _width = width as f32 / horz_hist.len() as f32;
            let pos_score    = (bias[1]-coor_and_height[0]) as f32;
            let height_score = (bias[3]-coor_and_height[1]) as f32;
            let mut score = pos_score.powf(2.0) + height_score.powf(2.0);

            let set_len = colorvector_set.len();
            let mut set_score = 0.0;
            for (i_set, iter_set) in colorvector_set.iter().enumerate(){
                //TODO
                let _a = iter_set.count as f32 / (counts[i_set] as f32 + 1.0);
                let frac_weight = iter_set.count as f32 / tot_count_set as f32;

                set_score += ( frac_weight * (1.0 - _a).powf(2.0) / set_len as f32).powf(2.0);
            }

            score += set_score;
            score = score.sqrt();

            if min_score2 > score{ 
                min_score2 = score; 
                good_class2 = i;
                good_coor2 = [_min, _max-_min];
            } 
        }


        if good_class2 != -1 {
            if min_score1 <= min_score2{
                rt[1] = good_coor1[0];
                rt[3] = good_coor1[1];
            } else {
                rt[1] = good_coor2[0];
                rt[3] = good_coor2[1];
            }
            /*
            if good_coor1[0] <= good_coor2[0]{ rt[1] = good_coor1[0]; }
            else { rt[1] = good_coor2[0]; }

            if good_coor1[1] >= good_coor2[1]{ rt[3] = good_coor1[1] - rt[1]; }
            else { rt[3] = good_coor2[1] - rt[1]; }
            */

        } else {
            rt[1] = good_coor1[0]; 
            rt[3] = good_coor1[1];// - rt[1];
        }
    }


    assert!(rt[2] > 0, "\ncharacter rect width was negative or zero {}\n", rt[2]);
    assert!(rt[3] > 0, "\ncharacter rect height was negative or zero {}\n", rt[3]);
    if verbose{
        println!("XXXXXXX");
    }
    return Some(rt);
}




//TODO
//We wish to find horizontal edges in order to determine health bars and meter gauges.
//We use the following kernels to this goal.
//Let a kernel be indecated by 'A' and some 3x3 sub set of pixels by 'B',
//such that a give pixel is indecated by B_ij.
//Applying the kernel to the pixel results in a score for pixel.
//The score is calculated as follows:
//    s_ij = \Sum_x^3 \Sum_y^3 A_xy B_(i-x-1)(j-y-1)
//
//We know that the closer the score is to zero the less likely 
//that pixel/region of pixel aligns with out goal.


const HORIZONTAL_MASK     : [[f32; 3 ]; 3] =  [[-1.0, -1.0, -1.0],
                                               [ 2.0,  2.0,  2.0],
                                               [-1.0, -1.0, -1.0]];
const HORIZONTAL_MASK_ALT : [[f32; 3 ]; 3] =  [[ 1.0,  1.0,  1.0],
                                               [-2.0, -2.0, -2.0],
                                               [ 1.0,  1.0,  1.0]];

const HORIZONTAL_MASK_BAR : [[f32; 3 ]; 3] =  [[-2.0, -2.0, -2.0],
                                               [ 1.0,  1.0,  1.0],
                                               [ 1.0,  1.0,  1.0]];
const HORIZONTAL_MASK_BAR_ALT : [[f32; 3 ]; 3] =  [[2.0,  2.0,  2.0],
                                                  [-1.0, -1.0, -1.0],
                                                  [-1.0, -1.0, -1.0]];





//NOTE
//Below is GG specific
pub fn gg_determine_health( bmp: &TGBitmap, x: usize, y: usize, w:usize, health_present_cv: ColorVector,
                                                                     health_delta_cv: ColorVector,
                                                                     health_absent_cv: ColorVector,
)->(f32, f32, f32){

    let mut percent_health_present = 0.0f32;
    let mut percent_health_absent = 0.0f32;
    let mut percent_health_delta = 0.0f32;

    let width  = bmp.info_header.width as usize;
    let height = bmp.info_header.height as usize;

    let _v =  y * width + x;

    for i in _v .. _v+w{
        let r = bmp.rgba[4*i+2];
        let g = bmp.rgba[4*i+1];
        let b = bmp.rgba[4*i+0];
        let pixel_cv = ColorVector::init(r, g, b);

        //health present
        if delta_colorvector(&health_present_cv, &pixel_cv) < 0.3
        && (health_present_cv.avg_intensity - pixel_cv.avg_intensity).abs() < 0.25
        {
            percent_health_present += 1.0;
        }

        //health absent
        if (health_absent_cv.avg_intensity - pixel_cv.avg_intensity).abs() < 0.25
        {
            percent_health_absent += 1.0;
        }

        //health delta
        //TODO
        //for some reason this doesn't work ...figure it out
        if delta_colorvector(&health_delta_cv, &pixel_cv) < 0.3
        && (health_delta_cv.avg_intensity - pixel_cv.avg_intensity).abs() < 0.25
        {
            percent_health_delta += 1.0;
        }

    }

    percent_health_present /= w as f32;
    percent_health_absent  /= w as f32;
    percent_health_delta   /= w as f32;

    return (percent_health_present, percent_health_absent, percent_health_delta);
}
  
pub fn gg_determine_tension(bmp: &TGBitmap, x: usize, y: usize, w: usize)->(f32, f32){
    let mut percent_absent = 0.0f32;
    let mut percent_present = 0.0f32;

    let width  = bmp.info_header.width as usize;
    let height = bmp.info_header.height as usize;

    let _v =  y * width + x;
    let black_cv = ColorVector::init(0, 0, 0);

    for i in _v .. _v+w{
        let r = bmp.rgba[4*i+2];
        let g = bmp.rgba[4*i+1];
        let b = bmp.rgba[4*i+0];
        let pixel_cv = ColorVector::init(r, g, b);

        //health present
        if (black_cv.avg_intensity - pixel_cv.avg_intensity).abs() < 0.25
        {
            percent_absent += 1.0;
        } else {
            percent_present += 1.0;
        }
    }

    percent_absent   /= w as f32;
    percent_present  /= w as f32;

    return (percent_absent, percent_present);
}


  
pub fn gg_determine_burst(bmp: &TGBitmap, rect: [usize; 4])->(f32, f32){
    let mut percent_absent = 0.0f32;
    let mut percent_present = 0.0f32;

    let w  = bmp.info_header.width as usize;
    let h = bmp.info_header.height as usize;

    let white_cv = ColorVector::init(255, 255, 255);

    for i in rect[1]..rect[1]+rect[3]{
        for j in rect[0] .. rect[0]+rect[2]{

            let _v =  i * w + j;

            let r = bmp.rgba[4*_v+2];
            let g = bmp.rgba[4*_v+1];
            let b = bmp.rgba[4*_v+0];
            let pixel_cv = ColorVector::init(r, g, b);

            //health present
            if (white_cv.avg_intensity - pixel_cv.avg_intensity).abs() < 0.15
            {
                percent_present += 1.0;
            } else {
                percent_absent += 1.0;
            }
        }
    }

    percent_absent   /= (rect[3]*rect[2]) as f32;
    percent_present  /= (rect[3]*rect[2]) as f32;

    return (percent_absent, percent_present);
}



