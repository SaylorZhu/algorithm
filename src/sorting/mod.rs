mod bubble_sort;

pub use bubble_sort::bubble_sort;

pub fn is_sorted<T>(arr: &[T]) -> bool
where
    T: std::cmp::PartialOrd,
{
    if arr.is_empty() {
        return true;
    }

    let mut prev = &arr[0];

    for item in arr.iter().skip(1) {
        if prev > item {
            return false;
        }

        prev = item;
    }

    true
}
