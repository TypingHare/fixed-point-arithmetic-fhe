use std::ops::Add;

pub struct Float {
    pub value: i32,
}

impl Add for Float {
    type Output = Float;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
