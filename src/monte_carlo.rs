use rand::Rng;

pub trait Calculator {
    fn calculate(&self, count: i64) -> f64;
}

struct CalculatorImpl {}

pub fn new() -> impl Calculator {
    return CalculatorImpl::new();
}

impl CalculatorImpl {
    fn new() -> CalculatorImpl {
        return CalculatorImpl {};
    }
}

impl Calculator for CalculatorImpl {
    fn calculate(&self, count: i64) -> f64 {
        let mut lt = 0;
        let mut rnd = rand::thread_rng();
        for _i in 1..count {
            let x: f64 = rnd.gen();
            let y: f64 = rnd.gen();
            let z = x * x + y * y;
            if z < 1.0 {
                lt += 1;
            }
        }
        return 4.0 * (lt as f64) / (count as f64);
    }
}

#[cfg(test)]
mod tests {
    use crate::monte_carlo::Calculator;
    #[test]
    fn test_calculate() {
        let calc = crate::monte_carlo::new();
        let val = calc.calculate(1000);
        assert!(3.0 < val);
        assert!(val < 4.0);
    }
}
