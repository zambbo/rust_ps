use std::io;

static mut cur_k: i32 = 0;
static mut k: i32 = 0;
static mut k_num: i32 = -1;

fn merge_sort(arr: &mut Vec<i32>, start: usize, end: usize) {
    if start >= end {
        return;
    }

    let mid: usize = (start + end) / 2;
    merge_sort(arr, start, mid);
    merge_sort(arr, mid + 1, end);
    merge(arr, start, mid, end);
}

fn merge(arr: &mut Vec<i32>, start:usize, mid:usize, end:usize) {
    let mut temp: Vec<i32> = Vec::new();

    let mut i: usize = start;
    let mut j: usize = mid+1;
    while i <= mid && j <= end {
        if arr[i] < arr[j] {
            temp.push(arr[i]);
            i+=1;
        } else {
            temp.push(arr[j]);
            j+=1;
        }
    }

    if i <= mid {
        while i <= mid {
            temp.push(arr[i]);
            i += 1;
        }
    }

    if j <= end {
        while j <= end {
            temp.push(arr[j]);
            j += 1;
        }
    }

    i = start;
    for &elem in temp.iter() {
        arr[i] = elem;
        unsafe {
            cur_k += 1;
            if cur_k == k {
                k_num = elem;
            }
        }
        i += 1;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut a_k_iter = input.trim().split_whitespace();
    let n: usize = a_k_iter.next().unwrap().parse().unwrap();
    unsafe {
        k = a_k_iter.next().unwrap().parse().unwrap();
    }

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let mut arr: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

    merge_sort(&mut arr, 0, n-1);
    unsafe {
        println!("{}", k_num);
    }
}
