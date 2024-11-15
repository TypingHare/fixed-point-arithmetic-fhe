use std::ops::{
    Add,
    Div,
    Mul,
    Sub,
};

#[derive(Debug, Clone, Copy)]
pub struct Fixed32 {
    // Stores the integer representing of the fixed-point value. The
    // fixed-point representation is scaled based on the `exp` field.
    value: i32,

    // The exponent used to determine the scaling factor of the fixed-point
    // number. It represents the negative power of 2 used to scale the value.
    exp: i32,
}

impl Fixed32 {
    pub fn new(value: i32, exp: i32) -> Self {
        Self { value, exp }
    }

    pub fn from<T: Into<f32>>(value: T, exp: i32) -> Self {
        let val: f32 = value.into() * (1 << exp) as f32;
        Self {
            value: val.round() as i32,
            exp,
        }
    }

    pub fn to_f32(self) -> f32 {
        self.value as f32 / (1 << self.exp) as f32
    }

    pub fn reciprocal(self) -> Self {
        let quotient: i32 = (1 << self.exp) / self.value;
        let result = Fixed32::new(quotient, self.exp);

        if quotient > 0 {
            // Apply Newton-Raphson method
            let guess = Fixed32 {
                value: quotient << self.exp,
                exp: self.exp,
            };
            let two = Fixed32::from(2f32, self.exp);
            let mut result = guess;
            for _ in 0..5 {
                result = result * (two - result * self)
            }

            result
        } else {
            result
        }
    }
}

impl Add for Fixed32 {
    type Output = Fixed32;

    fn add(self, other: Self) -> Self::Output {
        if self.exp == other.exp {
            Fixed32::new(self.value + other.value, self.exp)
        } else if self.exp > other.exp {
            let shift = self.exp - other.exp;
            Fixed32::new(self.value + (other.value << shift), self.exp)
        } else {
            let shift = other.exp - self.exp;
            Fixed32::new((self.value << shift) + other.value, other.exp)
        }
    }
}

impl Sub for Fixed32 {
    type Output = Fixed32;

    fn sub(self, other: Self) -> Self::Output {
        if self.exp == other.exp {
            Fixed32::new(self.value - other.value, self.exp)
        } else if self.exp > other.exp {
            let shift = self.exp - other.exp;
            Fixed32::new(self.value - (other.value << shift), self.exp)
        } else {
            let shift = other.exp - self.exp;
            Fixed32::new((self.value << shift) - other.value, other.exp)
        }
    }
}

impl Mul for Fixed32 {
    type Output = Fixed32;

    fn mul(self, other: Self) -> Self::Output {
        if self.exp != other.exp {
            panic!(
                "Only support multiplication between two fixed-point \
            numbers with the same exponential!"
            )
        }

        let val1: i64 = self.value as i64;
        let val2: i64 = other.value as i64;
        let product: i64 = (val1 * val2) >> self.exp;

        Fixed32 {
            value: product as i32,
            exp: self.exp,
        }
    }
}

impl Div for Fixed32 {
    type Output = Fixed32;

    fn div(self, other: Self) -> Self::Output {
        if self.exp != other.exp {
            panic!(
                "Only support multiplication between two fixed-point \
            numbers with the same exponential!"
            )
        }

        if other.value == 0 {
            panic!("Division by zero error!");
        }

        self * other.reciprocal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_same_exp() {
        let a = Fixed32::new(10, 4);
        let b = Fixed32::new(15, 4);
        let result = a + b;
        assert_eq!(result.value, 25);
        assert_eq!(result.exp, 4);
    }

    #[test]
    fn test_add_different_exp() {
        let a = Fixed32::new(10, 3);
        let b = Fixed32::new(15, 2);
        let result = a + b;
        assert_eq!(result.value, 40);
        assert_eq!(result.exp, 3);
    }

    #[test]
    fn test_sub_same_exp() {
        let a = Fixed32::new(20, 4);
        let b = Fixed32::new(10, 4);
        let result = a - b;
        assert_eq!(result.value, 10);
        assert_eq!(result.exp, 4);
    }

    #[test]
    fn test_sub_different_exp() {
        let a = Fixed32::new(40, 5);
        let b = Fixed32::new(15, 3);
        let result = a - b;
        assert_eq!(result.value, -20);
        assert_eq!(result.exp, 5);
    }

    #[test]
    fn test_mul_same_exp() {
        // (10 * 20) >> 4 = 200 >> 4 = 12
        let a = Fixed32::new(10, 4);
        let b = Fixed32::new(20, 4);
        let result = a * b;
        assert_eq!(result.value, 12);
        assert_eq!(result.exp, 4);
    }

    #[test]
    #[should_panic]
    fn test_mul_different_exp() {
        let a = Fixed32::new(10, 4);
        let b = Fixed32::new(20, 3);
        let _result = a * b;
    }

    #[test]
    fn test_div_same_exp() {
        let a = Fixed32 {
            value: 1158,
            exp: 5,
        };
        let b = Fixed32 {
            value: 2058,
            exp: 5,
        };
        let result = a / b;
        println!("{}", result.value);

        let result_float = result.to_f32();
        let expected_result = 1158.0 / 2058.0;

        assert!(
            (result_float - expected_result).abs() < 0.1,
            "Test case 1 failed: got {}, expected {}",
            result_float,
            expected_result
        );
    }
}