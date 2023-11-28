fn main() {
    let mut v = vec![1,2,3,4,5,6,7,8,9,10];

    v.retain(|&x| x % 2 == 0);

    assert_eq!(v.len(), 5);

    let r = sum(2,2);
    assert_eq!(r, 4);
}

fn sum(a: isize, b: isize) -> isize{
    a + b
}

#[cfg(test)]
mod tests {
    use crate::sum;

    #[test]
    fn test_sum() {
        let result = sum(2,2);
        assert_eq!(result, 4)
    }
}