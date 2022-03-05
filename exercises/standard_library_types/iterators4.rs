// iterators4.rs

// I AM DONE

pub fn factorial(num: u64) -> u64 {
    if num < 3 {
        return num;
    }
    let mut result = 1;
    for i in 1..num + 1 {
        result = i * result;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
