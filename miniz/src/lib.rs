#![allow(unused)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

extern{
    pub fn mz_version()->*const i8;


//int mz_compress(unsigned char *pDest, mz_ulong *pDest_len,
//                const unsigned char *pSource, mz_ulong source_len);
    pub fn mz_compress(pDest: *mut u8, pDest_len: *mut usize,
                       pSource: *const u8, source_len: usize)->i32;

//int mz_compress2(unsigned char *pDest, mz_ulong *pDest_len,
//                 const unsigned char *pSource, mz_ulong source_len, int level);
    pub fn mz_compress2(pDest: *mut u8, pDest_len: *mut usize,
                       pSource: *const u8, source_len: usize, level: i32)->i32;

//int mz_uncompress(unsigned char *pDest, mz_ulong *pDest_len,
//                  const unsigned char *pSource, mz_ulong source_len);
    pub fn mz_uncompress(pDest: *mut u8, pDest_len: *mut usize,
                       pSource: *const u8, source_len: usize)->i32;


//mz_ulong mz_compressBound(mz_ulong source_len);
		pub fn mz_compressBound(source_len: usize)->usize;

}

const MZ_OK         : i32 = 0;
const MZ_STREAM_END : i32 = 1;
const MZ_NEED_DICT  : i32 = 2;
const MZ_ERRNO          : i32 = -1;
const MZ_STREAM_ERROR   : i32 = -2;
const MZ_DATA_ERROR     : i32 = -3;
const MZ_MEM_ERROR      : i32 = -4;
const MZ_BUF_ERROR      : i32 = -5;
const MZ_VERSION_ERROR  : i32 = -6;
const MZ_PARAM_ERROR    : i32 = -10000;


pub fn uncompress(source: &[u8], mut dest_len: usize)->Result<Vec<u8>, &'static str>{unsafe{
    let len = source.len();
    let ptr = source.as_ptr();


    let mut buffer = vec![0u8;dest_len]; 

    let error = mz_uncompress(buffer.as_mut_ptr(), &mut dest_len as *mut _,
                ptr, len);
    //TODO check error code
    if error == MZ_STREAM_END{
        return Err("Stream error");
    } else if error == MZ_NEED_DICT {
        return Err("Dict error");
    } else if error == MZ_ERRNO {
        return Err("Check errno");
    } else if error == MZ_MEM_ERROR {
        return Err("Memory Error");
    } else if error == MZ_BUF_ERROR {
        return Err("Buffer Error");
    } else if error == MZ_VERSION_ERROR {
        return Err("Version Error");
    } else {
    }
    
    return Ok(buffer);
}}
pub fn compress(source: &[u8])->Result<Vec<u8>, &'static str>{unsafe{
    let len = source.len();
    let ptr = source.as_ptr();


    let mut dest_len = compress_bound(len);
    let mut buffer = vec![0u8;dest_len]; 

    let error = mz_compress(buffer.as_mut_ptr(), &mut dest_len as *mut _,
                ptr, len);

    //TODO check error code
    if error == MZ_STREAM_END{
        return Err("Stream error");
    } else if error == MZ_NEED_DICT {
        return Err("Dict error");
    } else if error == MZ_ERRNO {
        return Err("Check errno");
    } else if error == MZ_MEM_ERROR {
        return Err("Memory Error");
    } else if error == MZ_BUF_ERROR {
        return Err("Buffer Error");
    } else if error == MZ_VERSION_ERROR {
        return Err("Version Error");
    } else {
    }
    
    return Ok(buffer);
    
}}

pub fn compress_bound(source_len: usize)->usize{unsafe{
		mz_compressBound(source_len)
}}

