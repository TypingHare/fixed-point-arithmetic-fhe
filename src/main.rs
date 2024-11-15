use crate::fixed::Fixed32;

mod fixed;
mod fixed_tfhe;

fn main() {
    // let x = 1160f32;
    // let y = 13f32;
    // let real_result = x / y;
    // println!("real result: {}", real_result);
    //
    // let a = Fixed32::from(13f32, 16);
    // let b = Fixed32::from(2f32, 16);
    // let approx_result = a / b;
    // println!("approx result: {}", approx_result.to_f32())

    println!("? {}", 16777216 / 83886080);
    let x = Fixed32::from(5., 24);
    println!("real reciprocal: {}", 1. / 5.);
    println!("approx reciprocal: {}", x.reciprocal().to_f32());
}
