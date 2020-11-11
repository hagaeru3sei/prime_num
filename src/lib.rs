use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

pub fn is_prime(x: i64) -> bool {
    if x < 2 {
        return false
    } else if x == 2 || x == 3 || x == 5 {
        return true
    } else if x % 2 == 0 || x % 3 == 0 || x % 5 == 0 {
        return false
    }
    let mut k = 3;
    while k*k <= x {
        if x % k == 0 {
            return false
        }
        k += 2
    }
    true
}

#[pymodule]
fn mypackage(_py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "is_prime")]
    fn is_prime_py(_py: Python, x: i64) -> bool {
        is_prime(x)
    }

    #[pyfn(m, "example3")]
    fn example3_py(_py: Python) {
        for i in 1..10000000 {
            is_prime(i);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_is_prime() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(9), false);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(131071), true);
        assert_eq!(is_prime(131073), false);
        assert_eq!(is_prime(2147483647), true);
    }

}
