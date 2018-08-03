/// Computes Fibonacci number with given index.
#[allow(unused)]
fn fib(n: u32) -> u32 {
    let mut r = 1;
    let mut p = 1;

    for _ in 1..n {
        let t = r;
        r += p;
        p = t;
    }

    return r;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_values() {
        let expected = vec![1, 1, 2, 3, 5, 8, 13];

        let actual = (0..7)
            .map(fib)
            .collect::<Vec<_>>();

        assert_eq!(actual, expected);
    }
}
