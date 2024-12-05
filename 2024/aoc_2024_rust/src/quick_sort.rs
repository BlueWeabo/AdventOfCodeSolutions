fn partition(arr: &mut Vec<i64>, start: usize, end: usize) -> usize {
    if start == end {
        return end;
    }
    if end - start == 1 {
        return start;
    }
    let pivot = arr[start];
    let mut i = start;
    let mut j = end - 1;
    println!("{pivot} {i} {j}");

    loop {
        while arr[i] < pivot && i < end{
            i += 1;
        }

        while arr[j] > pivot && j > start{
            j -= 1;
        }

        if i >= j {
            break;
        }

        let temp: i64 = arr[i];
        arr[i] = arr[j];
        arr[j] = temp;
    }
    let temp: i64 = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
    j
}

pub fn quick_sort(arr: &mut Vec<i64>, start: usize, end: usize) {
    if start < end {
        let pi = partition(arr, start, end);
        if pi > start {
            quick_sort(arr, start, pi);
        }
        quick_sort(arr, pi+1, end);
    }
}
