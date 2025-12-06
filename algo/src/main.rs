mod fib;
use fib::fib;

fn main() {

    fn bad() -> i32 {
        let x = 5;
        x
    }

    let result = bad();
    println!("bad() = {}", result);

    let result = fib(10);
    println!("fib(10) = {}", result);
}
