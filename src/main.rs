// Returns index of the pivot
// Worst case is greatest/least O(n^2), n-1 comparisons
// best case median
// Only the vector between left and right will be partitioned
// The goal is to devide the array into two groups, or rather, return the index that makes it possible
// The pivot will come to the center of the vector, with unsorted on the left and right

use std::io;
use std::io::prelude::*;
use std::mem::swap; 

fn partition(left: isize, right: isize, values: &mut Vec<isize>) -> isize {
    let pivot = values[right as usize];

    // i = index of smaller element
    let mut i = left - 1;

    // j = the current index
    for j in left..right {
        if values[j as usize] < pivot {
            i += 1;

            // Swaps the left and right values
            values.swap(i as usize, j as usize); 
            /* let temp_left = values[i as usize];
            values[i as usize] = values[j as usize];
            values[j as usize] = temp_left; */
        }
    }

    // Swap the previous pivot (which was the rightmost element) and its found position (in the middle)
    values.swap((i + 1) as usize, right as usize); 
    
    /* let temp_old_at_new_pivot_position = values[(i + 1) as usize];
    values[(i + 1) as usize] = values[right as usize];
    values[right as usize] = temp_old_at_new_pivot_position; */

    // return the index of the pivot
    return i + 1;
}

// Recursive subroutine that chops the array at the pivot
// The pivot gets sandwiched between the sections
// Choosing a better pivot is pivotal
// Sorts all elements greater than the current to the right, less than to the left
// And does that recursively for the right and left subvectors
// And it's recursive, so it returns upwards
// This implementation picks the first element
// If it would be greater than, do nothing because it's already sorted
// right is length -1 if indexing starts at zero
fn quick_sort(left: isize, right: isize, values: &mut Vec<isize>) {
    if left < right {
        let pivot_index = partition(left, right, values);

        // Sort the left vector (all values lesser than the pivot)
        quick_sort(left, pivot_index - 1, values);

        // Sort the right vector (all values greater than the pivot)
        quick_sort(pivot_index + 1, right, values);
    }
}

fn main() {
    // let mut values = vec![1, 5, 2, 6, 3, 10];

    // let mut values = vec![-2, 3, -1, 5, 4, -3, 0];

    // let mut values = vec![1, 3, 5, 2, 7, 10, 4];

    /* for line in io::stdin().lock().lines().map(|line| line.unwrap()) {

        let mut values: Vec<isize> = line // PRE ALLOCATE!
            .split_whitespace()
            .skip(1) // <-- SKIP LENGTH PARAM
            .map(|_value| _value.parse::<isize>().unwrap())
            .collect();

        quick_sort(0, (values.len() - 1) as isize, &mut values);

        let formatted = values.iter().map(|x| x.to_string() + " ").collect::<String>();

        println!("{}", formatted.trim());

        // println!("{:?}", values.join(" "));
    } */

    // Input
    let mut line = String::with_capacity(500_000); // FIX ME!

    io::stdin().lock().read_line(&mut line); //.read_to_string(&mut line);

    let mut values: Vec<isize> = line // PRE ALLOCATE!
    .split_whitespace()
    .skip(1) // <-- SKIP LENGTH PARAM
    .map(|_value| _value.parse::<isize>().unwrap())
    .collect();

    quick_sort(0, (values.len() - 1) as isize, &mut values);

    let formatted = values.iter().map(|x| x.to_string() + " ").collect::<String>();

    println!("{}", formatted.trim());
}
