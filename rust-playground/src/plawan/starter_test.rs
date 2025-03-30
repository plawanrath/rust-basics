// @plawanrath

#[cfg(test)]
mod tests {
    use super::super::starter::sum;

    #[test]
    fn test_sum() {
        let nums = vec![1, 2, 3];
        assert_eq!(sum(&nums), 6);
    }
}