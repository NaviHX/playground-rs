pub mod tag;
pub mod trc;
pub mod ghost_cell;

#[macro_export]
macro_rules! log_call {
    ($f:expr, $($args:expr),*) => {{
        println!("Call {}", stringify!($f($($args),*)));
        let res = $f($($args),*);

        println!("Finished {} with result {:?}", stringify!($f($($args),*)), res);
        res
    }}
}

/// # Contract
/// - `slice` is sorted from small to large.
pub fn binary_search<T: Ord>(slice: &[T], target: &T) -> Result<usize, usize> {
    // # Invariants
    // - ** Any element in [..l] < target **;
    // - ** Any element in [r..] >= target **;
    // - l < r.

    let (mut l, mut r) = (0, slice.len());
    while l < r {
        use std::cmp::Ordering::*;

        let m = l + (r - l) / 2;
        match target.cmp(&slice[m]) {
            Greater => l = m + 1,
            Less | Equal => r = m,
        }
    }

    (r != slice.len() && slice[r] == *target)
        .then_some(r)
        .ok_or(r)
}

#[cfg(test)]
mod test {
    use super::binary_search;

    #[test]
    fn search() {
        assert_eq!(binary_search(&[1, 2, 3, 4, 5], &3), Ok(2));
    }

    #[test]
    fn search_start() {
        assert_eq!(binary_search(&[2, 3, 3, 4], &3), Ok(1));
    }

    #[test]
    fn search_in_empty_slice() {
        assert_eq!(binary_search(&[], &0), Err(0));
    }

    #[test]
    fn search_less_than_min() {
        assert_eq!(binary_search(&[2, 3, 4], &1), Err(0));
    }

    #[test]
    fn search_greater_than_max() {
        assert_eq!(binary_search(&[1, 2, 3], &4), Err(3));
    }
}
