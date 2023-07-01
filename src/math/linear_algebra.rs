use arrow::*;

fn array_builder(n: usize) -> Float64Array {

    // Initialize array builder
    let mut primitive_array_builder = Float64Builder::new(n);

    for _ in 0..n {
        primitive_array_builder.append_value(0.0).unwrap();
    }

    // Consume builder and convert to arry
    primitive_array_builder.finish()
}

fn vec_add(v: &Float64Array, w: &Float64Array) -> &Float64Array {
    // TODO assert v.len() == w.len()
    let mut r = array_builder(v.len());  // Build vector


    
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
        assert_eq!(array.len(), n);
        assert_eq!(array.value(0), 0.0);
        assert_eq!(array.is_null(n - 1), false);
    }
}