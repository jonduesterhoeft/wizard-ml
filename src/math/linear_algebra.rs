use arrow::array::{Array, PrimitiveArray, Float64Array, Float64Builder, BufferBuilder}

/// Initializes an m x n matrix to zero.
fn array_builder(n: usize) -> Float64Array {

    // Initialize array builder with capacity for n items
    let mut primitive_array_builder = Float64Builder::with_capacity(n);

    for _ in 0..n {
        primitive_array_builder.append_value(0.0);
    }

    // Consume builder and convert to arry
    primitive_array_builder.finish()
}

fn vec_add(v: &Float64Array, w: &Float64Array) -> Float64Array {
    // TODO assert v.len() == w.len()
    let mut r = array_builder(v.len());  // Build vector
    r
}

fn vec_subtract() {

}

fn vec_sum() {

}

fn scalar_multiply() {

}

fn vector_mean() {

}

fn dot() {

}

fn sum_of_squares() {

}

fn magnitude() {

}

fn distance() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_builder() {
        let n: usize = 3;
        let array = array_builder(n);
        println!("{:?}", array);
        assert_eq!(array.len(), n);
        assert_eq!(array.value(0), 0.0);
        assert_eq!(array.is_null(n - 1), false);
    }
}