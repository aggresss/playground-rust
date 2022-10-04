mod quick_sort;
pub use self::quick_sort::{partition, quick_sort};

mod insertion_sort;
pub use self::insertion_sort::insertion_sort;

mod heap_sort;
pub use self::heap_sort::heap_sort;

pub fn is_sorted<T>(arr: &[T]) -> bool
where
    T: PartialOrd,
{
    if arr.is_empty() {
        return true;
    }

    let mut prev = &arr[0];

    for item in arr.iter().skip(1) {
        if prev > item {
            return false;
        }

        prev = item
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sorted() {
        assert!(is_sorted(&[] as &[isize]));
        assert!(is_sorted(&["a"]));
        assert!(is_sorted(&[1, 2, 3]));
        assert!(is_sorted(&[0, 1,1]));

        assert_eq!(is_sorted(&[1, 0]), false);
        assert_eq!(is_sorted(&[2, 3, 1, -1, 5]), false);
    }
}