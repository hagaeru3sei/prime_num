use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

#[pymodule]
fn mypackage(_py: Python, m: &PyModule) -> PyResult<()> {

    fn is_prime(x: i64) -> bool {
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

    #[pyfn(m, "is_prime")]
    fn is_prime_py(_py: Python, x: i64) -> bool {
        let r = is_prime(x);
        r
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
