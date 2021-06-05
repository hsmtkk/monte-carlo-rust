mod monte_carlo;

use crate::monte_carlo::Calculator;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("Usage: monte-carlo-rust loop");
    }
    let count_str = &args[1];
    let count = count_str.parse::<i64>().expect("failed to parse as i32");
    let calc = monte_carlo::new();
    let val = calc.calculate(count);
    println!("{}", val);
}
