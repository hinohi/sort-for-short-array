unsafe fn sort_2<T: PartialOrd + Copy>(a: *mut T, b: *mut T) {
    let min_v = if *a < *b { *a } else { *b };
    let max_v = if *a >= *b { *a } else { *b };
    *a = min_v;
    *b = max_v;
}

pub fn sort2<T: PartialOrd + Copy>(arr: &mut [T; 2]) {
    unsafe {
        sort_2(&mut arr[0], &mut arr[1]);
    }
}

pub fn sort3<T: PartialOrd + Copy>(arr: &mut [T; 3]) {
    unsafe {
        sort_2(&mut arr[0], &mut arr[1]);
        sort_2(&mut arr[0], &mut arr[2]);
        sort_2(&mut arr[1], &mut arr[2]);
    }
}

pub fn sort4<T: PartialOrd + Copy>(arr: &mut [T; 4]) {
    unsafe {
        sort_2(&mut arr[0], &mut arr[2]);
        sort_2(&mut arr[1], &mut arr[3]);

        sort_2(&mut arr[0], &mut arr[1]);
        sort_2(&mut arr[2], &mut arr[3]);

        sort_2(&mut arr[1], &mut arr[2]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_sorted(arr: &[i32]) {
        let mut v = arr.to_vec();
        v.sort();
        assert_eq!(arr, v.as_slice());
    }

    #[test]
    fn test_sort2() {
        for a0 in 0..=1 {
            for a1 in 0..=1 {
                let mut arr = [a0, a1];
                sort2(&mut arr);
                check_sorted(&arr);
            }
        }
    }

    #[test]
    fn test_sort3() {
        for a0 in 0..=1 {
            for a1 in 0..=1 {
                for a2 in 0..=1 {
                    let mut arr = [a0, a1, a2];
                    sort3(&mut arr);
                    check_sorted(&arr);
                }
            }
        }
    }

    #[test]
    fn test_sort4() {
        for a0 in 0..=1 {
            for a1 in 0..=1 {
                for a2 in 0..=1 {
                    for a3 in 0..=1 {
                        let mut arr = [a0, a1, a2, a3];
                        sort4(&mut arr);
                        check_sorted(&arr);
                    }
                }
            }
        }
    }
}
