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

pub fn sort5<T: PartialOrd + Copy>(arr: &mut [T; 5]) {
    unsafe {
        sort_2(&mut arr[0], &mut arr[4]);
        sort_2(&mut arr[2], &mut arr[4]);
        sort_2(&mut arr[1], &mut arr[3]);
        sort_2(&mut arr[0], &mut arr[2]);
        sort_2(&mut arr[3], &mut arr[4]);
        sort_2(&mut arr[1], &mut arr[2]);
        sort_2(&mut arr[0], &mut arr[3]);
        sort_2(&mut arr[2], &mut arr[3]);
        sort_2(&mut arr[0], &mut arr[1]);
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

    fn as_arr<const N: usize>(b: u32) -> [i32; N] {
        let mut arr = [0; N];
        for i in 0..N {
            if b & (1 << i) != 0 {
                arr[i] = 1;
            }
        }
        arr
    }

    #[test]
    fn test_as_arr() {
        let arr: [i32; 5] = as_arr(0b10011);
        assert_eq!(arr, [1, 1, 0, 0, 1]);
    }

    #[test]
    fn test_sort2() {
        for b in 1..1 << 2 {
            let mut arr = as_arr(b);
            sort2(&mut arr);
            check_sorted(&arr);
        }
    }

    #[test]
    fn test_sort3() {
        for b in 1..1 << 3 {
            let mut arr = as_arr(b);
            sort3(&mut arr);
            check_sorted(&arr);
        }
    }

    #[test]
    fn test_sort4() {
        for b in 1..1 << 4 {
            let mut arr = as_arr(b);
            sort4(&mut arr);
            check_sorted(&arr);
        }
    }

    #[test]
    fn test_sort5() {
        for b in 1..1 << 5 {
            let mut arr = as_arr(b);
            sort5(&mut arr);
            check_sorted(&arr);
        }
    }
}
