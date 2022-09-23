pub fn cocktail_shaker_sort<T: std::cmp::PartialOrd>(arr: &mut [T]) {
    // let len = arr.len();
    // if len == 0 {
    //     return;
    // }
    // let mut top = len - 1;
    // let mut bottom = 0;

    // let mut swapped = true;

    // while swapped {
    //     swapped = false;
    //     for i in bottom..top {
    //         if arr[i] > arr[i + 1] {
    //             arr.swap(i, i + 1);
    //             swapped = true;
    //         }
    //     }

    //     if swapped {
    //         top -= 1;
    //     }

    //     for i in ((bottom + 1)..=top).rev() {
    //         if arr[i] < arr[i - 1] {
    //             arr.swap(i, i - 1);
    //             swapped = true;
    //         }
    //     }
    //     if swapped {
    //         bottom += 1;
    //     }
    // }

    let len = arr.len();

    if len == 0 {
        return;
    }

    loop {
        let mut swapped = false;

        for i in 0..(len - 1) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

        swapped = false;

        for i in (0..(len - 1)).rev() {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::is_sorted;
    use super::*;

    #[test]
    fn basic() {
        let mut arr = vec![5, 2, 1, 3, 4, 6];
        cocktail_shaker_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn empty() {
        let mut arr = Vec::<i32>::new();
        cocktail_shaker_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn one_element() {
        let mut arr = vec![1];
        cocktail_shaker_sort(&mut arr);
        assert!(is_sorted(&arr));
    }

    #[test]
    fn pre_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5, 6];
        cocktail_shaker_sort(&mut arr);
        assert!(is_sorted(&arr));
    }
}
