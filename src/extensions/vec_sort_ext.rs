pub(crate) trait VecSort<N> {

    fn merge_sort(&self) -> Self;
    fn quick_sort(&self) -> Self;
    fn heap_sort(&self) -> Self;
    fn tim_sort(&self) -> Self;
}

impl <N: Clone + PartialOrd> VecSort<N> for Vec<N> {

    fn merge_sort(&self) -> Self {
        if self.len() <= 1 { return self.clone(); }
        let mid = self.len() / 2;
        let left = &self[..mid].to_vec().merge_sort();
        let right = &self[mid..].to_vec().merge_sort();

        let mut result = Self::new();
        let (mut i, mut j) = (0, 0);
        while i < left.len() && j < right.len() {
            if left[i] < right[j] {
                result.push(left[i].clone());
                i += 1;
            } else {
                result.push(right[j].clone());
                j += 1;
            }
        }
        result.extend(left[i..].to_vec());
        result.extend(right[j..].to_vec());
        result
    }

    fn quick_sort(&self) -> Self {
        if self.len() <= 1 { return self.clone(); }
        let mut array = self.clone().into_iter();
        array.next().map_or_else(Self::new, |pivot| {
            let (lower, higher): (Self, Self) = array.partition(|it| it < &pivot);
            let lower = lower.quick_sort();
            let higher = higher.quick_sort();
            lower.into_iter()
                .chain(core::iter::once(pivot))
                .chain(higher)
                .collect()
        })
    }

    fn heap_sort(&self) -> Self {

        fn shift_down<T: Clone + PartialOrd>(array: &mut [T], start: usize, end: usize) {
            let mut root = start;
            loop {
                let mut child = root * 2 + 1;
                if child > end {
                    break;
                }
                if child < end && array[child] < array[child + 1] {
                    child += 1;
                }
                if array[root] < array[child] {
                    array.swap(root, child);
                    root = child;
                } else {
                    break;
                }
            }
        }

        if self.len() <= 1 { return self.clone(); }
        let array = &mut self.clone();
        for start in (0..array.len() / 2).rev() {
            shift_down(array, start, self.len() - 1);
        }
        for end in (1..self.len()).rev() {
            array.swap(0, end);
            shift_down(array, 0, end - 1);
        }
        array.clone()
    }

    fn tim_sort(&self) -> Self {

        const fn calc_min_run(n: usize) -> usize {
            let (mut n, mut r) = (n, 0);
            while n >= 32 {
                r |= n & 1;
                n >>= 1;
            }
            n + r
        }

        fn insertion_sort<T: Clone + PartialOrd>(arr: &mut [T], left: usize, right: usize) {
            for i in (left + 1)..=right {
                let mut j = i;
                while j > left && arr[j] < arr[j - 1] {
                    arr.swap(j, j - 1);
                    j -= 1;
                }
            }
        }

        fn merge<T: Clone + PartialOrd>(arr: &mut [T], left: usize, mid: usize, right: usize) {
            let len1 = mid - left + 1;
            let len2 = right - mid;
            let left_arr = arr[left..=mid].to_vec();
            let right_arr = arr[mid + 1..=right].to_vec();

            let mut i = 0;
            let mut j = 0;
            let mut k = left;

            while i < len1 && j < len2 {
                if left_arr[i] <= right_arr[j] {
                    arr[k] = left_arr[i].clone();
                    i += 1;
                } else {
                    arr[k] = right_arr[j].clone();
                    j += 1;
                }
                k += 1;
            }

            arr[k..k + len1].clone_from_slice(&left_arr[i..]);
            arr[k + len1..k + len1 + len2].clone_from_slice(&right_arr[j..]);
        }

        let (array, n) = (&mut self.clone(), self.len());
        let min_run = calc_min_run(n);

        for start in (0..n).step_by(min_run) {
            let end = std::cmp::min(start + min_run - 1, n - 1);
            insertion_sort(array, start, end);
        }

        let mut size = min_run;
        while size < n {
            for left in (0..n).step_by(2 * size) {
                let mid = std::cmp::min(n - 1, left + size - 1);
                let right = std::cmp::min(left + 2 * size - 1, n - 1);
                if mid < right { merge(array, left, mid, right); }
            }
            size *= 2;
        }

        array.clone()
    }
}
