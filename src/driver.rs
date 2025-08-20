unsafe extern "C" {
    fn kernel_driver_hello();
    fn kernel_driver_add(a: i32, b: i32) -> i32;
}

pub fn hello() {
    unsafe {
        kernel_driver_hello();
    }
}

pub fn add(a: i32, b: i32) -> i32 {
    unsafe { kernel_driver_add(a, b) }
}

