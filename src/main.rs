use std::io;
use std::io::prelude::*;

// use std::thread::sleep;
// use std::time::{Duration, Instant};

/* Returns index of the pivot in the modified vector, with unsorted greater elements to the right, lesser to the left
Worst case is greatest/least O(n^2), n-1 comparisons, best case median */
/* fn partition(left: i32, right: i32, values: &mut Vec<i32>) -> i32 {
    let pivot = values[right as usize];

    // i = index of smaller element (memory)
    let mut i = left - 1;

    // j = the current index
    for j in left..right {
        if values[j as usize] < pivot {
            i += 1;

            // swap the smaller and larger value
            values.swap(i as usize, j as usize);
        }
    }

    // swap pivot element (rightmost element) and its found position (sandwiched in the middle)
    values.swap((i + 1) as usize, right as usize);

    // return the index of the pivot
    return i + 1;
}

/* Recursive subroutine that chops the vector at the pivot
Choosing a better pivot is pivotal, but this implementation picks the last element */
fn quick_sort(left: i32, right: i32, values: &mut Vec<i32>) {
    // if it's greater than, do nothing because it's already sorted
    if left < right {
        let pivot_index = partition(left, right, values);

        // Sort the left vector (all values lesser than the pivot)
        // right is length -1 if indexing starts at zero
        quick_sort(left, pivot_index - 1, values);

        // Sort the right vector (all values greater than the pivot.
        quick_sort(pivot_index + 1, right, values);
    }
} */

fn partition(left: i32, right: i32, values: &mut [i32]) -> i32 {
    let pivot = values[right as usize];

    // i = index of smaller element (memory)
    let mut i = left - 1;

    // j = the current index
    for j in left..right {
        if values[j as usize] < pivot {
            i += 1;

            // swap the smaller and larger value
            values.swap(i as usize, j as usize);
        }
    }

    // swap pivot element (rightmost element) and its found position (sandwiched in the middle)
    values.swap((i + 1) as usize, right as usize);

    // return the index of the pivot
    return i + 1;
}

fn insertion_sort(left: i32, right: i32, values: &mut [i32]) {
    // rotate through values 
    for i in left+1..right+1 {
        let key = values[i as usize]; 
        let mut j = i - 1; 

        // move all greater one step to the right 
        while j >= left && values[j as usize] > key {
            // values.swap(j as usize + 1, j as usize); 
            values[(j + 1) as usize] = values[j as usize]; 
            j -= 1; 
        }

        // and move the smaller element back 
        values[(j + 1) as usize] = key; 
    }
 }

/* Recursive subroutine that chops the vector at the pivot
Choosing a better pivot is pivotal, but this implementation picks the last element */
fn quick_sort(left: i32, right: i32, values: &mut [i32]) {
    // if it's greater than, do nothing because it's already sorted
    if left < right {
        // if the subvector is short enough, do insertion sort instead 
        if right - left < 100 {
            insertion_sort(left, right, values);
        } else {
            let pivot_index = partition(left, right, values);

            // Sort the left vector (all values lesser than the pivot)
            // right is length -1 if indexing starts at zero
            quick_sort(left, pivot_index - 1, values);
    
            // Sort the right vector (all values greater than the pivot.
            quick_sort(pivot_index + 1, right, values);    
        }
    }
}

fn main() {
    /* let now = Instant::now();

    let mut values = vec![1, 5, 2, 6, 3, 10];

    for _ in 0..10 {
        quick_sort(0, (values.len() - 1) as i32, &mut values);
        let formatted = values
            .iter()
            .map(|x| x.to_string() + " ")
            .collect::<String>();

        println!("{}", formatted.trim());
    }

    println!("{:?}", now.elapsed().as_nanos()); */

    // println!("{:?}", values.to_string());

    // let mut values = vec![-2, 3, -1, 5, 4, -3, 0];

    // let mut values = vec![1, 3, 5, 2, 7, 10, 4];

    // 86 / 95

    // Input
    /* let mut line = String::with_capacity(200_000); // FIX ME!

    io::stdin().lock().read_line(&mut line); //.read_to_string(&mut line);

    let mut values: Vec<i32> = line // PRE ALLOCATE!
        .split_whitespace()
        .skip(1) // <-- SKIP LENGTH PARAM
        .map(|_value| _value.parse::<i32>().unwrap())
        .collect();

    quick_sort(0, (values.len() - 1) as i32, &mut values);

    let formatted = values
        .iter()
        .map(|x| x.to_string() + " ")
        .collect::<String>();

    println!("{}", formatted.trim()); */

    // Input
    let mut line = String::with_capacity(200_000); // FIX ME!

    io::stdin().lock().read_line(&mut line); //.read_to_string(&mut line);

    let mut values: Vec<i32> = line // PRE ALLOCATE!
        .split_whitespace()
        .skip(1) // <-- SKIP LENGTH PARAM
        .map(|_value| _value.parse::<i32>().unwrap())
        .collect();

    quick_sort(0, (values.len() - 1) as i32, &mut values[..]);

    for elem in values {
        print!("{} ", elem);
    }

    // let mut values = [1, 5, 2, 6, 3, 10];

    /* let mut values = [6, 4];

    insertion_sort(0, (values.len() - 1) as i32, &mut values);

    for elem in values {
        print!("{} ", elem);
    } */
}
