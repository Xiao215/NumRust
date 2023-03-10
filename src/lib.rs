use pyo3::prelude::*;

#[pyfunction]
fn primecounter(range_from: u64, range_til: u64) -> (u32, u32) {
    /* Returns the number of found prime numbers between [range_from] and [range_til] """ */
    let mut prime_count: u32 = 0;
    let mut check_count: u32 = 0;
    let _from: u64 = if range_from < 2 { 2 } else { range_from };
    let mut prime_found: bool;

    for num in _from..=range_til {
        prime_found = false;
        for divnum in 2..num {
            check_count += 1;
            if num % divnum == 0 {
                prime_found = true;
                break;
            }
        }
        if !prime_found {
            prime_count += 1;
        }
    }
    return (prime_count, check_count);
}

fn pow(base:u64,exp:u64)->u64{
    // Power function
    let mut result:u64=1;
    let mut iter:u64=exp;
    while iter>0{
        result=result*base;
        iter--;
    }
    return result;
}


/// Put the function in a Python module
#[pymodule]
fn numrust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(primecounter, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn simple_test_false() {
    //     assert_eq!(is_prime(0), false);
    //     assert_eq!(is_prime(1), false);
    //     assert_eq!(is_prime(12), false)
    // }

    // #[test]
    // fn simple_test_true() {
    //     assert_eq!(is_prime(2), true);
    //     assert_eq!(is_prime(3), true);
    //     assert_eq!(is_prime(41), true)
    // }
}
