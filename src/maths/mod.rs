use std::ops::{Add, Div, Mul, Sub};

mod simplex_2d;
mod octave_simplex_2d;
mod selective_noise;
pub use octave_simplex_2d::OctaveNoiseGen;
pub use selective_noise::SelectiveNoiseGen;


/// just grabbed from my rust_maths crate and changed to descending
pub fn quicksort_ascending<T, P: PartialOrd + Copy>(list: Vec<(P, T)>) -> Vec<(P, T)>{
    if list.len() <= 1 {return list;}

    let mut less = Vec::new();
    let mut equal = Vec::new();
    let mut more = Vec::new();

    let target_val = list[list.len() / 2].0;

    for item in list {
        if item.0 < target_val {less.push(item)}
        else if item.0 > target_val {more.push(item)}
        else {equal.push(item)}
    }

    let mut sorted = quicksort_ascending(less);
    sorted.append(&mut equal);
    sorted.append(&mut quicksort_ascending(more));
    sorted
}


/// just grabbed from my rust_maths crate and changed to descending
pub fn quicksort_descending<T, P: PartialOrd + Copy>(list: Vec<(P, T)>) -> Vec<(P, T)>{
    if list.len() <= 1 {return list;}

    let mut less = Vec::new();
    let mut equal = Vec::new();
    let mut more = Vec::new();

    let target_val = list[list.len() / 2].0;

    for item in list {
        if item.0 < target_val {less.push(item)}
        else if item.0 > target_val {more.push(item)}
        else {equal.push(item)}
    }

    let mut sorted = quicksort_descending(more);
    sorted.append(&mut equal);
    sorted.append(&mut quicksort_descending(less));
    sorted
}


/// Returns the position value that could be used to lerp between the two values, result is clamped to 0.0..=1.0 so values outisde the the values will return either 1 or 0
pub fn inverse_lerp<T>(min: T, max: T, value: T) -> f32
where
    T: Div<Output = f32>,
    T: Sub<Output = T>,
    T: Copy
{
    ((value - min) / (max - min)).clamp(0.0, 1.0)
}


/// Lerp, linearly interpolates between the two values and gives an output
/// - The position values are all clamped between 0 and 1
pub fn lerp<T>(min: T, max: T, position: f32) -> T
where
    T: Mul<f32, Output = T>,
    T: Sub<Output = T>,
    T: Add<Output = T>,
{
    let pos = position.clamp(0.0, 1.0);
    min * (1.0 - pos) + max * pos
}