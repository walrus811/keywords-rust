extern "C" {
    fn abs(input: i32) -> i32;
    fn sqrt(input: f64) -> f64;
}

fn main() {
    unsafe {
        println!(
            "Absolute value of -3 according to C: {} {}",
            abs(-3),
            sqrt(25.0)
        );
    }
}
