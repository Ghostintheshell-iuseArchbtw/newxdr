mod driver;

fn main() {
    driver::hello();
    let result = driver::add(2, 3);
    println!("Result from kernel driver: {}", result);
}
