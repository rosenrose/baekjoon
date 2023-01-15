use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let nums: Vec<_> = input.skip(1).collect();
    let Some(next) = next_permutation(nums) else {
        println!("-1");
        return;
    };

    for num in next {
        print!("{num} ");
    }
}

fn next_permutation(mut nums: Vec<i32>) -> Option<Vec<i32>> {
    let len = nums.len();

    let Some(i) = (1..len).rfind(|&i| nums[i - 1] < nums[i]) else {
        return None;
    };
    let j = (i..len).rfind(|&j| nums[j] > nums[i - 1]).unwrap();

    nums.swap(i - 1, j);
    quick_sort(&mut nums[i..], len - i);

    Some(nums)
}

fn quick_sort(arr: &mut [i32], len: usize) {
    if len <= 1 {
        return;
    }

    let (mut i, mut j) = (0, len - 1);
    let pivot = arr[(len / 2)];

    while i <= j {
        while arr[i] < pivot {
            i += 1;
        }
        while arr[j] > pivot {
            j -= 1;
        }

        if i > j {
            break;
        }

        arr.swap(i, j);
        i += 1;
        j -= 1;
    }

    quick_sort(&mut arr[..=j], j + 1);
    quick_sort(&mut arr[i..], len - i);
}
