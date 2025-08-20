fn main() {
    cc::Build::new()
        .file("kernel/driver.c")
        .compile("kernel_driver");
}
