use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BigNumber {
    pub mantissa: f64,
    pub exponent: i64,
}

impl BigNumber {
    pub fn new(mantissa: f64, exponent: i64) -> Self {
        let mut bn = BigNumber { mantissa, exponent };
        bn.normalize();
        bn
    }

    pub fn to_f64(&self) -> f64 {
        self.mantissa * 10f64.powi(self.exponent as i32)
    }

    pub fn zero() -> Self {
        BigNumber {
            mantissa: 0.0,
            exponent: 0,
        }
    }

    pub fn one() -> Self {
        BigNumber {
            mantissa: 1.0,
            exponent: 0,
        }
    }

    pub fn gte(&self, other: &Self) -> bool {
        if self.exponent != other.exponent {
            return self.exponent > other.exponent;
        }
        self.mantissa >= other.mantissa
    }

    fn normalize(&mut self) {
        if self.mantissa == 0.0 {
            self.exponent = 0;
            return;
        }
        while self.mantissa.abs() >= 10.0 {
            self.mantissa /= 10.0;
            self.exponent += 1;
        }
        while self.mantissa.abs() < 1.0 && self.mantissa != 0.0 {
            self.mantissa *= 10.0;
            self.exponent -= 1;
        }
    }
}

impl Add for BigNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.mantissa == 0.0 {
            return other;
        }
        if other.mantissa == 0.0 {
            return self;
        }

        let diff = self.exponent - other.exponent;
        if diff.abs() > 15 {
            return if diff > 0 { self } else { other };
        }

        if diff > 0 {
            let num = self.mantissa + other.mantissa / 10f64.powi(diff as i32);
            BigNumber::new(num, self.exponent)
        } else {
            let num = self.mantissa / 10f64.powi((-diff) as i32) + other.mantissa;
            BigNumber::new(num, other.exponent)
        }
    }
}

impl Sub for BigNumber {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if other.mantissa == 0.0 {
            return self;
        }
        // Simple subtraction, assuming we don't go negative for now for simplicity in this snippet
        // In a real game we need to handle negatives or clamp to zero
        let diff = self.exponent - other.exponent;
        if diff.abs() > 15 {
            if diff > 0 {
                return self;
            } else {
                return BigNumber::zero();
            } // heuristic
        }

        if diff > 0 {
            let num = self.mantissa - other.mantissa / 10f64.powi(diff as i32);
            BigNumber::new(num, self.exponent)
        } else {
            // If we are here, we might be subtracting a larger number from a smaller one
            // For resource costs, usually we check can_afford first.
            let num = self.mantissa / 10f64.powi((-diff) as i32) - other.mantissa;
            BigNumber::new(num, other.exponent)
        }
    }
}

impl Mul<f64> for BigNumber {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        BigNumber::new(self.mantissa * rhs, self.exponent)
    }
}

impl Mul<BigNumber> for BigNumber {
    type Output = Self;

    fn mul(self, rhs: BigNumber) -> Self {
        BigNumber::new(self.mantissa * rhs.mantissa, self.exponent + rhs.exponent)
    }
}

impl fmt::Display for BigNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.exponent < 3 {
            write!(f, "{:.2}", self.mantissa * 10f64.powi(self.exponent as i32))
        } else {
            write!(f, "{:.2}e{}", self.mantissa, self.exponent)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BigNumber;

    #[test]
    fn normalizes_large_mantissa() {
        let value = BigNumber::new(1234.0, 0);
        assert_eq!(value, BigNumber::new(1.234, 3));
    }

    #[test]
    fn add_handles_same_exponent() {
        let lhs = BigNumber::new(2.0, 3);
        let rhs = BigNumber::new(3.5, 3);
        assert_eq!(lhs + rhs, BigNumber::new(5.5, 3));
    }

    #[test]
    fn add_handles_far_apart_exponents() {
        let lhs = BigNumber::new(9.9, 20);
        let rhs = BigNumber::new(1.0, 0);
        assert_eq!(lhs + rhs, lhs);
    }

    #[test]
    fn subtract_handles_basic_case() {
        let lhs = BigNumber::new(9.0, 2);
        let rhs = BigNumber::new(4.0, 2);
        assert_eq!(lhs - rhs, BigNumber::new(5.0, 2));
    }

    #[test]
    fn multiply_by_scalar_and_bignumber() {
        let value = BigNumber::new(2.5, 3);
        assert_eq!(value * 4.0, BigNumber::new(1.0, 4));

        let other = BigNumber::new(2.0, 2);
        assert_eq!(value * other, BigNumber::new(5.0, 5));
    }

    #[test]
    fn gte_compares_exponent_then_mantissa() {
        let a = BigNumber::new(1.1, 6);
        let b = BigNumber::new(9.9, 5);
        let c = BigNumber::new(1.2, 6);

        assert!(a.gte(&b));
        assert!(!a.gte(&c));
    }
}
