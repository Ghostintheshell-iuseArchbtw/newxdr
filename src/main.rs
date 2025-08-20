unsafe extern "C" {
    fn kernel_driver_hello();
}

fn main() {
    unsafe {
        kernel_driver_hello();
    }
}
