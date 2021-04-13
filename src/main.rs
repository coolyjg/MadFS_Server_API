mod ffi_wrapper;
// use std::str::from_utf8;
use ffi_wrapper::{
    // MY_POP_DATA, MY_POP_MD,
    // my_create_data, 
    my_write_at_data, my_read_at_data,
    my_free_chunk, 
    my_free_path, my_init_data, my_fin,
    my_get_atime, my_get_ctime, my_get_mtime,
    my_get_mode, my_get_nlink, my_get_size,
    my_get_chunk_size, my_create_metadata,
    my_remove_metadata, my_init_mdata,
};
use std::ffi::CString;

fn main() {
    // let data_path = CString::new("data_pool").unwrap();
    // let dpop = my_init_data(data_path.as_ptr() as *const u8);
    // let chunk_size = 256;
    // let data: Vec<u8> = (1..chunk_size).map(|x| x as u8).collect();
    // let mut buf = vec![0; chunk_size-1];
    // let dpath = CString::new("file1").unwrap().as_ptr();

//***************read and write chunk**************succecced
    // // my_create_data(dpop, dpath as *const u8, 0, "".to_string().as_ptr(), 0);
    // my_write_at_data(dpop, dpath as *const u8, 0, 0, (&data).as_ptr(), data.len() as u32);
    // my_read_at_data(dpop, dpath as *const u8, 0, 0, (&mut buf).as_ptr() as *mut u8);
    // assert_eq!(data, buf);

//*********read and write chunk slice***************failed
    // my_create_data(dpop, dpath as *const u8, 1, "".to_string().as_ptr(), 0);
    // my_write_at_data(dpop, dpath as *const u8, 1, 10, data[..10].as_ptr(), 10);
    // my_read_at_data(dpop, dpath as *const u8, 1, 0, buf[..20].as_ptr() as *mut u8);
    // assert_eq!(buf[10..20], data[..10]);

//**************remove chunks************************succecced
    // let mut buf2 = [0u8; 10];
    // for chunk_id in 0..4{
    //     // my_create_data(dpop, dpath as *const u8, chunk_id, "".to_string().as_ptr(), 0);
    //     my_write_at_data(dpop, dpath as *const u8, chunk_id, 0, (&buf).as_ptr(), buf.len() as u32);
    // }
    // for chunk_id in 1..4{
    //     my_free_chunk(dpop, dpath as *mut u8, chunk_id);
    // }
    // for chunk_id in 1..4{
    //     let result = my_read_at_data(dpop, dpath as *const u8, chunk_id, 0, (&mut buf).as_ptr() as *mut u8);
    //     println!("{}",result);
    // }
    // my_free_path(dpop, dpath as *mut u8);
    // my_fin(dpop);

//****************test medatada API*******************succecced
    let mdata_path = CString::new("mdata_pool").unwrap();    
    let mdpop = my_init_mdata(mdata_path.as_ptr() as *const u8);
    let mdpath = CString::new("mdata").unwrap().as_ptr();
    my_create_metadata(mdpop, mdpath as *mut u8, 10, 20, 30, 0777, 0, 30, 256);
    assert_eq!(10 as isize, my_get_atime(mdpop, mdpath as *mut u8));
    assert_eq!(20 as isize, my_get_mtime(mdpop, mdpath as *mut u8));
    assert_eq!(30 as isize, my_get_ctime(mdpop, mdpath as *mut u8));
    assert_eq!(0777 as u32, my_get_mode(mdpop, mdpath as *mut u8));
    assert_eq!(0 as u32, my_get_nlink(mdpop, mdpath as *mut u8));
    assert_eq!(30 as usize, my_get_size(mdpop, mdpath as *mut u8));
    assert_eq!(256 as usize, my_get_chunk_size(mdpop, mdpath as *mut u8));
    my_remove_metadata(mdpop, mdpath as *mut u8);
    my_fin(mdpop);
}
