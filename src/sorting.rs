pub fn quicksort_inplace(xs: &mut [i32; 10], a: usize, b: usize) {
    if a >= b {
        return;
    }

    let mut l = a;
    let mut r = b-1;
    let p = xs[b];

    while l <= r {
        while l <= r && xs[l] <= p {
            l += 1;
        }
        while l <= r && xs[r] >= p {
            r -= 1;
        }
        if l < r {
            let temp = xs[l];
            xs[l] = xs[r];
            xs[r] = temp;
        }
    }
    let temp = xs[l];
    xs[l] = xs[b];
    xs[b] = temp;
    
    quicksort_inplace(xs, a, l-1);
    quicksort_inplace(xs, l+1, b);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_inplace_quicksort() {
        let mut xs: [i32; 10] = [5,9,6,8,4,7,2,8,4,3];
        let l = xs.len();
        let sorted: [i32; 10] = [2,3,4,4,5,6,7,8,8,9];
        quicksort_inplace(&mut xs, 0, l-1);

        assert_eq!(sorted, xs);
    }
}