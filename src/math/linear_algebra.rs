use arrow::array::{Array, PrimitiveArray, Float64Array, Float64Builder, BufferBuilder};
use arrow::error::ArrowError;
use arrow::datatypes::ArrowPrimitiveType;

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

pub fn dot(a: &Float64Array, b: &Float64Array) -> Result<Option<f64>, ArrowError> {
    let v_multiply_result = arrow::compute::multiply_checked(a, b)?;
    let v_dot_result = arrow::compute::sum_checked(&v_multiply_result)?;
    Ok(v_dot_result)
}

pub fn sum_of_squares(a: &Float64Array) -> Result<Option<f64>, ArrowError> {
    let v_sum_of_squares_result = dot(a, a)?;
    Ok(v_sum_of_squares_result)
}

pub fn magnitude(a: &Float64Array) -> Result<Option<f64>, ArrowError> {
    let v_magnitude_result = sum_of_squares(a)?;
    Ok(v_magnitude_result)
}

pub fn distance(a: &Float64Array, b: &Float64Array) -> Result<Option<f64>, ArrowError> {
    let v_subtract_result = arrow::compute::subtract(a, b)?;
    let v_distance_result = magnitude(&v_subtract_result)?;
    Ok(v_distance_result)
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

    #[test]
    fn test_dot_product() {
        let a = Float64Array::from(vec![1.0, 2.0]);
        let b = Float64Array::from(vec![3.0, 4.0]);
        let result = dot(&a, &b);
        match result {
            Ok(result) => {
                match result {
                    Some(result) => result,
                    None => None
                }
            Err(error) => error
        }
        assert_eq!(result, 11.0)
    }

    #[test]
    fn test_sum_of_squares() {
        let a = Float64Array::from(vec![1.0, 2.0, 3.0]);
        let result = sum_of_squares(&a);
        match result {
            Ok(result) => result,
            Err(error) => error
        }
        assert_eq!(result, 14.0)
    }

    #[test]
    fn test_magnitude() {
        let a = Float64Array::from(vec![1.0, 2.0, 3.0]);
        let result = magnitude(&a);
        match result {
            Ok(result) => result,
            Err(error) => error
        }
        assert_eq!(result, (14 as f64).sqrt())
    }

    #[test]
    fn test_distance() {
        let a = Float64Array::from(vec![1.0, 2.0, 3.0]);
        let b = Float64Array::from(vec![3.0, 2.0, 1.0]);
        let result = distance(&a, &b);
        match result {
            Ok(result) => result,
            Err(error) => error
        }
        assert_eq!(result, (8 as f64).sqrt())
    }
}
