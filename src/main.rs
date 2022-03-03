use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::slice; 

// use rand::Rng;


// use std::Core;

// use std::mem::size_of;
// use std::time::{SystemTime, UNIX_EPOCH};

/* fn partition(left: usize, right: usize, values: &mut [i32]) -> usize {
    let pivot = values[right];

    // i = index of smaller element (memory)
    let mut i = left;

    // j = the current index
    for j in left..right {
        if values[j] < pivot {
            // swap the smaller and larger value
            values.swap(i, j);

            i += 1;
        }
    }

    // swap pivot element (rightmost element) and its found position (sandwiched in the middle)
    values.swap(i, right);

    // return the index of the pivot
    return i; // + 1;
} */

fn insertion_sort(left: usize, right: usize, values: &mut [i32]) {
    // rotate through values
    for i in left + 1..right + 1 {
        let key = values[i as usize];
        let mut j: isize = i as isize - 1;

        // move all greater one step to the right
        while j >= left as isize && values[j as usize] > key {
            // values.swap(j as usize + 1, j as usize);
            values[(j + 1) as usize] = values[j as usize];
            j -= 1;
        }

        // and move the smaller element back
        values[(j + 1) as usize] = key;
    }
}

// Should go quicker because it more often splits from the middle element
fn hoares_partition(left: usize, right: usize, values: &mut [i32]) -> usize {
    // let pivot = values[(left + right) / 2];
    let pivot = pick_pivot(left, right, values); // values[(left + right) / 2];

    let mut i: isize = left as isize - 1;
    let mut j = right + 1;

    // you're doing this multiple times, so...
    loop {
        // find smallest element
        // Rust's do while
        while {
            i += 1;
            values[i as usize] < pivot
        } {}

        // find largest element
        while {
            j -= 1;
            values[j] > pivot
        } {}

        // if equal, return index
        if i >= j as isize {
            return j;
        }

        values.swap(i as usize, j);
    }
}

// http://ceur-ws.org/Vol-2568/paper4.pdf
// https://stackoverflow.com/questions/12454866/how-to-optimize-quicksort
// https://yourbasic.org/golang/quicksort-optimizations/

fn median(a: i32, b: i32, c: i32) -> i32 {
    if a < b {
        if b < c {
            return b;
        } else if a < c {
            return c;
        } else {
            return a;
        }
    }
    if a < c {
        return a;
    } else if b < c {
        return c;
    } else {
        return b;
    }
}

// Choose the pivot element to be the median of three random elements
fn pick_pivot(left: usize, right: usize, values: &mut [i32]) -> i32 {
    // RNG from: https://github.com/rust-lang/rust/blob/37b6a5e5e82497caf5353d9d856e4eb5d14cbe06/src/libcore/slice/sort.rs#L488

    /* fn random(left: usize, right: usize) -> i32 {
        return rand::thread_rng().gen_range(left..right) as i32;
    } */

    return median(values[left], values[right], values[(left + right) / 2])


    /* let random = || -> i32 {
        return rand::thread_rng().gen_range(left..right) as i32;
    }; */

    // let random = ((random * 7621) + 1) % 32768;

    // let mut file = File::open("/dev/urandom");

    // let mut random = [0];  

    // let mut random = fs::read("/dev/urandom"); 

    // file.fs::read(&mut random); 

    // dbg!(random); 
    // println!("{:?}", random);

    // return 0 // random[0] as i32; 

    // let median = median(values[left], values[right], values[(left+right)/2]); 

    // let median = median(random(left, right), random(left, right), random(left, right)); 

    // let median = median(random(), random(), random()); 

    // dbg!(median); 

    // return median; 

    /* println!("{:?}", right);
    let mut random = (right - left) as u32; // = (right-left) as u32;

    let mut gen_u32 = || {
        random ^= random << 13;
        random ^= random >> 17;
        random ^= random << 5;
        random
    };

    let mut gen_usize = || {
        if size_of::<usize>() <= 4 {
            gen_u32() as usize
        } else {
            (((gen_u32() as u64) << 32) | (gen_u32() as u64)) as usize
        }
    };

    let modulus = (right - left).next_power_of_two();

    let mut other = gen_usize() & (modulus - 1);

    dbg!(other);
    println!("hello"); */

    // Can't use the rand crate
    /* let num1 = vec![2, 3];
    let num2 = vec![2, 5,123, 214, 123, 123,123];
    let address1 = &num1 as *const Vec<i32>;
    let address2 = &num2 as *const Vec<i32>;

    let number1 = address1 as usize;
    let number2 = address2 as usize;
    println!("{:?}", number1);
    println!("{:?}", number2);
    println!("{}", number1 % (right-left));
    println!("{}", number2 % (right-left)); */

    /* let nanos: usize = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .subsec_nanos().try_into().unwrap();

    println!("{:?}", right -left);

    println!("{:?}", nanos);
    println!("{:?}", nanos % (right -left)); */

    /* let nanos = unsafe {srand()};
    println!("{:?}", nanos); */

    // Also avoid having to call values.len() by reading the first
    // values[nanos % values.len()];
}

// https://stackoverflow.com/questions/11872952/simd-the-following-code
/* Recursive subroutine that chops the vector at the pivot
Choosing a better pivot is pivotal, but this implementation picks the last element */
fn quick_sort(left: usize, right: usize, values: &mut [i32]) {
    // if it's greater than, do nothing because it's already sorted
    if left < right {
        // if the subvector is short enough, do insertion sort instead
        // know the optimal is between 35-150
        // or lower bound 5 upper bound 500, really
        // pdqsqort has the insertion sort bound 24
        if right - left < 24 {
            insertion_sort(left, right, values);
        } else {
            let pivot_index = hoares_partition(left, right, values);

            // Sort the left vector (all values lesser than the pivot)
            // right is length -1 if indexing starts at zero
            quick_sort(left, pivot_index, values);

            // Sort the right vector (all values greater than the pivot.
            quick_sort(pivot_index + 1, right, values);
        }
    }
}

fn main() {
    // println!("{:?}", values.to_string());

    // 86 / 95

    // Input
    let mut line = String::with_capacity(200_000); // FIX ME!

    io::stdin().lock().read_line(&mut line); //.read_to_string(&mut line);

    let mut values: Vec<i32> = line // PRE ALLOCATE!
        .split_whitespace()
        .map(|_value| _value.parse::<i32>().unwrap())
        .collect();

    // Also avoid having to call values.len() by reading the first

    let length = values.remove(0) as usize - 1;

    quick_sort(0, length, &mut values[..]);

    let formatted = values.iter().map(|x| x.to_string() + " ").collect::<String>();

    println!("{}", formatted);

    /* let mut line = String::with_capacity(200_000); // FIX ME!

    io::stdin().lock().read_line(&mut line); //.read_to_string(&mut line);

    let mut values: Vec<i32> = line // PRE ALLOCATE!
        .split_whitespace()
        .skip(1)
        .map(|_value| _value.parse::<i32>().unwrap())
        .collect();

    values.sort_unstable();

    let formatted = values.iter().map(|x| x.to_string() + " ").collect::<String>();

    println!("{}", formatted.trim());  */

    /* for elem in values {
        print!("{} ", elem);
    } */

    // https://www.minimalrust.com/an-adventure-in-simd-2/
    // https://elib.dlr.de/145402/1/thesis_col.pdf

    /* let mut values = [
        1, 5, 2, 5, 7, 6, 3, 10, 12, 12, 124, 23, 23, 123, 123, 12, 241, 213, 23, 2, 124, 24, 2, 3,
        2, 2341, 23, 22,
    ]; */

    /* let mut values = [1, 5, 2, 6, 3, 10];

    pick_pivot(0, values.len()-1, &mut values[..]); */

    /* let mut values = [1, 5, 2, 6, 3, 10];

    // let mut values = vec![-2, 3, -1, 5, 4, -3, 0];

    // let mut values = vec![1, 3, 5, 2, 7, 10, 4];

    // let mut values = [6, 4];

    quick_sort(0, values.len() - 1, &mut values[..]);

    for elem in values {
        print!("{} ", elem);
    } */
}
