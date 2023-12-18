#![cfg_attr(coverage_nightly, feature(coverage_attribute))]


#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    let r = sum(668.5, 668.5);

    println!("sum result is: {}", r)
}

fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

#[cfg(test)]
mod tests {
    use crate::sum;

    #[test]
    fn test_sum_integer() {
        let result = sum(2, 2);
        assert_eq!(result, 4)
    }

    #[test]
    fn test_sum_float() {
        let result = sum(3.5, 5.3);

        assert_eq!(result, 8.8)
    }
}
