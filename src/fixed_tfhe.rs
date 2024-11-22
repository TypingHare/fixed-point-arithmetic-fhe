use std::ops::{
    Add,
    Div,
    Mul,
    Sub,
};
use tfhe::{
    prelude::{
        CastInto,
        FheDecrypt,
        FheTryEncrypt,
    },
    ClientKey,
    FheInt32,
    FheInt64,
};

pub struct TfheFixed32 {
    // Stores the integer representing of the fixed-point value. The
    // fixed-point representation is scaled based on the `exp` field.
    value: FheInt32,

    // The exponent used to determine the scaling factor of the fixed-point
    // number. It represents the negative power of 2 used to scale the value.
    exp: u32,
}

impl TfheFixed32 {
    pub fn new(value: FheInt32, exp: u32) -> Self {
        Self { value, exp }
    }

    pub fn new_with_key(client_key: &ClientKey, value: i32, exp: u32) -> Self {
        Self {
            value: FheInt32::try_encrypt(value, client_key).unwrap(),
            exp,
        }
    }

    pub fn from<T: Into<f32>>(
        client_key: &ClientKey,
        value: T,
        exp: u32,
    ) -> TfheFixed32 {
        let val_f32: f32 = value.into() * (1 << exp) as f32;
        let val_i32: i32 = val_f32.round() as i32;
        TfheFixed32::new(
            FheInt32::try_encrypt(val_i32, client_key).unwrap(),
            exp,
        )
    }

    pub fn to_f32(&self, client_key: &ClientKey) -> f32 {
        let val_i32: i32 = self.value.decrypt(client_key);
        val_i32 as f32 / (1 << self.exp) as f32
    }

    pub fn reciprocal(self) -> f32 {
        // FIXME
        let quotient: i32 = (1 << self.exp) / self.value;
        let result = TfheFixed32::new(quotient, self.exp);

        if quotient > 0 {
            // Apply Newton-Raphson method
            let guess = TfheFixed32{
                value: quotient << self.exp,
                exp: self.exp,
            };
            let two = TfheFixed32::from(2f32, self.exp);
            let mut result = guess;
            for _ in 0..5 {
                result = result * (two - result * self)
            }

            result
        } else {
            // quotient less than 1, how to find the initial guess?
            // sin(x)?
            result
        }
    }
}

impl Add for TfheFixed32 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.exp == other.exp {
            TfheFixed32::new(self.value + other.value, self.exp)
        } else if self.exp > other.exp {
            let shift = self.exp - other.exp;
            TfheFixed32::new(self.value + (other.value << shift), self.exp)
        } else {
            let shift = other.exp - self.exp;
            TfheFixed32::new((self.value << shift) + other.value, other.exp)
        }
    }
}

impl Sub for TfheFixed32 {
    type Output = TfheFixed32;

    fn sub(self, other: Self) -> Self::Output {
        if self.exp == other.exp {
            TfheFixed32::new(self.value - other.value, self.exp)
        } else if self.exp > other.exp {
            let shift = self.exp - other.exp;
            TfheFixed32::new(self.value - (other.value << shift), self.exp)
        } else {
            let shift = other.exp - self.exp;
            TfheFixed32::new((self.value << shift) - other.value, other.exp)
        }
    }
}

impl Mul for TfheFixed32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let lhs_val_i64: FheInt64 = self.value.cast_into();
        let rhs_val_i64: FheInt64 = rhs.value.cast_into();
        let product_i64: FheInt64 = (lhs_val_i64 * rhs_val_i64) >> self.exp;
        let product_i32: FheInt32 = product_i64.cast_into();

        Self::new(product_i32, self.exp)
    }
}

impl Div for TfheFixed32 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        let quotient = self.value / other.value * (1 << self.exp);
        Self::new(quotient, self.exp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tfhe::{
        generate_keys,
        set_server_key,
        ConfigBuilder,
    };

    #[test]
    fn test_add() {
        let config = ConfigBuilder::default().build();
        let (client_key, server_key) = generate_keys(config);

        set_server_key(server_key);
        let a = TfheFixed32::new_with_key(&client_key, 10, 24);
        let b = TfheFixed32::new_with_key(&client_key, 15, 24);
        let result = a + b;
        let result_val: i32 = result.value.decrypt(&client_key);

        assert_eq!(result_val, 25);
        assert_eq!(result.exp, 24);
    }

    #[test]
    fn test_sub() {
        let config = ConfigBuilder::default().build();
        let (client_key, server_key) = generate_keys(config);

        set_server_key(server_key);
        let a = TfheFixed32::new_with_key(&client_key, 15, 24);
        let b = TfheFixed32::new_with_key(&client_key, 10, 24);
        let result = a - b;
        let result_val: i32 = result.value.decrypt(&client_key);

        assert_eq!(result_val, 5);
        assert_eq!(result.exp, 24);
    }

    #[test]
    fn test_mul() {
        let config = ConfigBuilder::default().build();
        let (client_key, server_key) = generate_keys(config);

        set_server_key(server_key);
        let a = TfheFixed32::from(&client_key, 2.47, 24);
        let b = TfheFixed32::from(&client_key, 3.19, 24);
        let result = a * b;
        let result_val = result.to_f32(&client_key);

        assert_eq!(result_val, 7.8793);
        assert_eq!(result.exp, 24);
    }
}
