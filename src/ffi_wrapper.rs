extern crate libc;
// use std::io::Result;
use libc::c_int;
use libc::size_t;
use libc::c_void;
use libc::c_char;
use libc::c_long;
// use std::ffi::CString;

pub enum PMEMobjpool {}
// pub static mut MY_POP_DATA: Option<*mut PMEMobjpool>= Some(my_init_data(CString::new("data_pool").unwrap().as_ptr() as *mut u8));
// pub static mut MY_POP_MD: Option<*mut PMEMobjpool>= None;
// pub static mut MY_POP_DATA: Option<*mut PMEMobjpool>= None;

#[link(name = "nvm_st")]
#[link(name = "pmemobj")]
extern "C"{
    fn create_data(pop: *mut PMEMobjpool, p: *const c_char, id: c_int, buf: *const c_char, size: c_int)->c_int;
    fn write_at_data(pop: *mut PMEMobjpool, p: *const c_char, id: c_int, offset: c_int, buf: *mut c_char, size: c_int)->c_int;
    fn read_at_data(pop: *mut PMEMobjpool, p: *const c_char, id: c_int, offset: c_int, buf: *mut c_char)->c_int;
    fn free_chunk(pop: *mut PMEMobjpool, p: *mut c_char, id: c_int)->c_void;
    fn free_path(pop: *mut PMEMobjpool, p: *mut c_char)->c_void;
    fn init_data(path: *const c_char)->*mut PMEMobjpool;
    fn fin(pop: *mut PMEMobjpool)->c_void;
    fn get_atime(pop: *mut PMEMobjpool, p: *mut c_char)->size_t;
    fn get_mtime(pop: *mut PMEMobjpool, p: *mut c_char)->size_t;
    fn get_ctime(pop: *mut PMEMobjpool, p: *mut c_char)->size_t;
    fn get_mode(pop: *mut PMEMobjpool, p: *mut c_char)->c_int;
    fn get_nlink(pop: *mut PMEMobjpool, p: *mut c_char)->c_int;
    fn get_size(pop: *mut PMEMobjpool, p: *mut c_char)->size_t;
    fn get_chunk_size(pop: *mut PMEMobjpool, p: *mut c_char)->size_t;
    fn create_metadata(pop: *mut PMEMobjpool, p: *mut c_char,
        atime: size_t, mtime: size_t, ctime: size_t,
        mode: c_int, nlink: c_int, size: c_long, chunk_size: c_long)->c_void;
    fn remove_metadata(pop: *mut PMEMobjpool, p: *mut c_char)->c_void;
    fn init_mdata(path: *const c_char)->*mut PMEMobjpool;
}

pub fn my_create_data(pop: *mut PMEMobjpool, p: *const u8, id: u32, buf: *const u8, size: u32)->c_int{
    unsafe{
        create_data(pop, p as *const c_char, id as c_int, buf as *const c_char, size as c_int)
    }
}

pub fn my_write_at_data(pop: *mut PMEMobjpool, p: *const u8, id: u32, offset: u32, buf: *const u8, size: u32)->c_int{
    unsafe{
        write_at_data(pop, p as *const c_char, id as c_int, offset as c_int, buf as *mut c_char, size as c_int)
    }
}

pub fn my_read_at_data(pop: *mut PMEMobjpool, p: *const u8, id: u32, offset: u32, buf: *mut u8)->c_int{
    unsafe{
        read_at_data(pop, p as *const c_char, id as c_int, offset as c_int, buf as *mut c_char)
    }
}

pub fn my_free_chunk(pop: *mut PMEMobjpool, p: *mut u8, id: u32)->c_void{
    unsafe{
        free_chunk(pop, p as *mut c_char, id as c_int)
    }
}

pub fn my_free_path(pop: *mut PMEMobjpool, p: *mut u8)->c_void{
    unsafe{
        free_path(pop, p as *mut c_char)
    }
}

pub fn my_init_data(path: *const u8)->*mut PMEMobjpool{
    unsafe{
        init_data(path as *const c_char)
    }
}

pub fn my_fin(pop: *mut PMEMobjpool)->c_void{
    unsafe{
        fin(pop)
    }
}

pub fn my_get_atime(pop: *mut PMEMobjpool, p: *mut u8)->isize{
    unsafe{
        get_atime(pop, p as *mut c_char) as isize
    }
}

pub fn my_get_mtime(pop: *mut PMEMobjpool, p: *mut u8)->isize{
    unsafe{
        get_mtime(pop, p as *mut c_char) as isize
    }
}

pub fn my_get_ctime(pop: *mut PMEMobjpool, p: *mut u8)->isize{
    unsafe{
        get_ctime(pop, p as *mut c_char) as isize
    }
}

pub fn my_get_mode(pop: *mut PMEMobjpool, p: *mut u8)->u32{
    unsafe{
        get_mode(pop, p as *mut c_char) as u32
    }
}

pub fn my_get_nlink(pop: *mut PMEMobjpool, p: *mut u8)->u32{
    unsafe{
        get_nlink(pop, p as *mut c_char) as u32
    }
}

pub fn my_get_size(pop: *mut PMEMobjpool, p: *mut u8)->usize{
    unsafe{
        get_size(pop, p as *mut c_char) as usize
    }
}

pub fn my_get_chunk_size(pop: *mut PMEMobjpool, p: *mut u8)->usize{
    unsafe{
        get_chunk_size(pop, p as *mut c_char) as usize
    }
}

pub fn my_create_metadata(pop: *mut PMEMobjpool, p: *mut u8,
    atime: isize, mtime: isize, ctime: isize, 
    mode: u32, nlink: u32, size: usize, chunk_size: usize
)->c_void{
    unsafe{
        create_metadata(pop, p as *mut c_char, atime as size_t, mtime as size_t,
        ctime as size_t, mode as c_int, nlink as c_int, size as i64, chunk_size as i64)
    }
}

pub fn my_remove_metadata(pop: *mut PMEMobjpool, p: *mut u8)->c_void{
    unsafe{
        remove_metadata(pop, p as *mut c_char)
    }
}

pub fn my_init_mdata(path: *const u8)->*mut PMEMobjpool{
    unsafe{
        init_mdata(path as *const c_char)
    }
}
