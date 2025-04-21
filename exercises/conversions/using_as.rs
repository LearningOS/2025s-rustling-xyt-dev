// using_as.rs
//
// Type casting in Rust is done via the usage of the `as` operator. Please note
// that the `as` operator is not only used when type casting. It also helps with
// renaming imports.
//
// The goal is to make sure that the division does not fail to compile and
// returns the proper type.
//
// Execute `rustlings hint using_as` or use the `hint` watch subcommand for a
// hint.

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    // Casting from an integer to float will produce the closest possible float: 
    // * if necessary, rounding is according to `roundTiesToEven` mode ( `roundTiesToEven` 即 round 到最近的偶数，采取这种方式是因为**当** **`ULP > 1`** **时 (即** **`ULP >= 2`** **时) 浮点数当然一定是个偶数，而当** **`ULP<=1`** **时浮点数一定能精确表示整数**)
    // * on overflow, infinity (of the same sign as the input) is produced
    // * note: with the current set of numeric types, overflow can only happen on `u128 as f32` for values greater or equal to `f32::MAX + (0.5 ULP)` (ULP: Unit in the Last Place，最小精度单位)
    total / values.len() as f64 // 2^52 - 1 内转换精确
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}

