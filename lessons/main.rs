fn main() {
    let arr = [-1, 3, 5, 7, 8, 10, 24, 37, 42, 135];

    let result = bin_search(&arr, 23);

    match result {
        Some((found_value, found_index)) => {
            println!("Found value {} at index {}", found_value, found_index)
        }
        None => println!("Not found"),
    }
}

fn bin_search(arr: &[i32], desired_value: i32) -> Option<(i32, usize)> {
    if arr.is_empty() {
        return None;
    }

    let mut low_bound: usize = 0;
    let mut up_bound = arr.len() - 1;

    let mut i: usize = 0;

    while low_bound <= up_bound {
        i += 1;

        let mid = (low_bound + up_bound) / 2;
        let mid_value = arr[mid];

        if mid_value == desired_value {
            return Some((mid_value, mid));
        } else if desired_value > mid_value {
            low_bound = mid + 1;
        } else {
            // avoid underflow when mid == 0
            if mid == 0 {
                break;
            }
            up_bound = mid - 1;
        }

        println!("Step {}", i);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    const ARR: [i32; 10] = [-1, 3, 5, 7, 8, 10, 24, 37, 42, 135];
    #[test]
    fn element_found() {
        assert_eq!((-1, 0), bin_search(&ARR, -1).unwrap());
    }
    #[test]
    fn element_not_found() {
        let result = bin_search(&ARR, 1234);
        assert!(result.is_none())
    }
    #[test]
    fn middle_element() {
        assert_eq!((10, 5), bin_search(&ARR, 10).unwrap());
    }
    #[test]
    fn last_element() {
        assert_eq!((135, 9), bin_search(&ARR, 135).unwrap());
    }
    #[test]
    fn empty_array() {
        let empty: [i32; 0] = [];
        assert!(bin_search(&empty, 1).is_none());
    }
}
