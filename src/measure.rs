use std::time::Instant;

pub fn diff<T>(exact: T, approximation: T) -> f32
where
    T: Into<f32> + Copy,
{
    let exact_f32 = exact.into();
    let approximation_f32 = approximation.into();
    (exact_f32 - approximation_f32).abs() / exact_f32
}

pub fn measure_time<F: FnOnce() -> T, T>(closure: F) -> (T, f64) {
    let start_time = Instant::now();
    let result = closure();
    let elapsed_time = start_time.elapsed().as_secs_f64();
    (result, elapsed_time * 1000.)
}
