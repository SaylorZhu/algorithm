pub fn bubble_sort<T: std::cmp::PartialOrd>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    let mut n = arr.len();
    let mut sorted = false;

    while !sorted {
        sorted = true;
        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
        n -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;
    #[test]
    fn descending() {
        let mut v = vec![6, 5, 4, 3, 2, 1];
        bubble_sort(&mut v);
        assert!(is_sorted(&v));
    }

    #[test]
    fn ascending() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        bubble_sort(&mut v);
        assert!(is_sorted(&v));
    }

    #[test]
    fn random() {
        let mut v = vec![2, 1, 4, 3, 6, 5];
        bubble_sort(&mut v);
        assert!(is_sorted(&v));
    }

    #[test]
    fn empty() {
        let mut v: Vec<i32> = vec![];
        bubble_sort(&mut v);
        assert!(is_sorted(&v));
    }
}
