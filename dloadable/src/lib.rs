use std::os::raw::c_char;
use std::slice::from_raw_parts;
use std::ffi::CStr;

#[no_mangle]
pub fn main(argc: isize, argv0: *const *const c_char) -> isize {
    // assigned expression is semantically equivalent to `env::args().collect()`
    let args: Vec<String> = unsafe {
        let args_slice = from_raw_parts(argv0, argc as usize);
        args_slice.iter().map(|ptr| CStr::from_ptr(*ptr).to_str().unwrap().to_owned()).collect()
    };
    for (i, arg) in args.iter().enumerate() {
        println!("{}: {}", i, arg);
    }
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
