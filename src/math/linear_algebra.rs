use arrow::*;

fn array_builder(m: usize, n: usize) -> Float64Array {

    // Initialize array builder
    let mut primitive_array_builder = Float64Builder::new(m);

    for _ in 0..m {
        primitive_array_builder.append_value(0.0).unwrap();
    }

    let values = (0..n)
        .collect::<Vec<f64>>();

    // Append slice of values to array
    primitive_array_builder.append_slice(&values).unwrap();
    // Consume builder and convert to arry
    primitive_array_builder.finish()
}

fn vec_add(v: &Float64Array, w: &Float64Array) -> &Float64Array {
    // TODO assert v.len() == w.len()
    let mut r = array_builder(v.len(), 1);  // Build vector
    
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
        let m: usize = 2;
        let n: usize = 3;
        let array = array_builder(m, n);
        println!("{:?}", array);
        // assert_eq!(test_data, expected);
    }
}