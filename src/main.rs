extern crate dcp_utils;

use std::collections::HashMap;


// 1.1
// Get Product of all other elements without using division
// expects elements len to be greater than 1
fn get_product_of_other(elements: &Vec<i32>, products_out: &mut Vec<i32>) {
    // [1,2,3,4,5]
    // before [1, 2, 6, 24, 120]
    // after[120, 120, 60, 20, 5]
    // out [120, 60, 40, 30, 24]
    // to get product take the index + 1 from after and multiply index - 1 from before
    let mut products_before = Vec::new();
    let mut products_after = Vec::new();

    products_before.push(*elements.first().unwrap());
    for i in 1..elements.len() {
        products_before.push(products_before.last().unwrap() * elements[i]);
    }

    products_after.push(*elements.last().unwrap());
    for i in (0..elements.len() - 1).rev() {
        products_after.push(products_after.last().unwrap() * elements[i])
    }

    // now reverse the after array to get correct indices
    products_after.reverse();

    // now build the out array
    products_out.push(products_after[1]);
    for i in 1..elements.len() - 1 {
        products_out.push(products_before[i - 1] * products_after[i + 1])
    }
    products_out.push(products_before[products_before.len() - 2]);
}

// 1.2
// Locate smallest window to be sorted
fn locate_smallest_window_to_sort(elements: &Vec<i32>) -> (usize, usize) {
    let mut win_start: usize = 0;
    let mut win_end: usize = 0;
    // find start by going from back to front and tracking smallest seen
    let mut smallest: i32 = i32::max_value();
    for i in (0..elements.len()).rev() {
        if elements[i] < smallest {
            smallest = elements[i];
        }
        else {
            win_start = i;
        }
    }

    let mut largest: i32 = i32::min_value();
    for i in 0..elements.len() {
        if elements[i] > largest {
            largest = elements[i];
        }
        else {
            win_end = i;
        }
    }

    return (win_start, win_end);
}

// 1.3
// Find Max Contiguous Sub Array Sum
fn get_max_contiguous_sum(elements: &Vec<i32>) -> i32 {
    let mut max = 0;
    let mut curr = 0;
    for num in elements {
        curr = std::cmp::max(num + curr, *num);
        max = std::cmp::max(max, curr);
    }
    return max;
}

// Find Max Contiguous Sub Array Sum Wrapping
fn get_max_contiguous_sum_wrapping(elements: &Vec<i32>) -> i32 {
    let sum = elements.iter().fold(0, |acc, x| acc + x);

    let mut min = 0;
    let mut curr = 0;
    for num in elements {
        curr = std::cmp::min(num + curr, *num);
        min = std::cmp::min(min, curr);
    }

    return std::cmp::max(sum - min, get_max_contiguous_sum(&elements));
}

// 1.4
// Find Number of Smaller Elements to the Right
fn get_smaller_elements_to_right_naive(elements: &Vec<i32>, smaller_to_right_out: &mut Vec<i32>) {
    for i in 0..elements.len() {
        let mut num_smaller = 0;
        for j in (i + 1)..elements.len() {
            if elements[j] < elements[i] {
                num_smaller += 1;
            }
        }
        smaller_to_right_out.push(num_smaller);
    }
}

// if we iterate from back to front, when we bisect, we know that any elements after the insertion
// are to the right of the current number/index
fn get_smaller_elements_to_right(elements: &Vec<i32>, smaller_to_right_out: &mut Vec<usize>) {
    let mut seen = Vec::new();
    for num in elements.iter().rev() {
        let i = dcp_utils::bisect::bisect_left(&seen, *num);
        smaller_to_right_out.push(i);
        dcp_utils::bisect::insort(&mut seen, *num);
    }
    smaller_to_right_out.reverse();
}

// 2.1
// given a string s and a word w, return all starting indicies of anagram w in s
// eg s: abxaba, w: ab, out: [0,3,4]
// assert s.len > w.len
fn get_anagram_start_indicies(s: &Vec<u8>, w: &String, anagram_indicies_out: &mut Vec<usize>) {
    let mut anagram_counts = HashMap::new();
    for c in w.chars() {
        let count = anagram_counts.entry(c).or_insert(0);
        *count += 1;
    }

    let mut anagram_checker = HashMap::new();
    let anagram_len = w.chars().count();

    // init our anagram checker
    for i in 0..anagram_len {
        let count = anagram_checker.entry(s[i] as char).or_insert(0);
        *count += 1;
    }

    if anagram_checker == anagram_counts {
        anagram_indicies_out.push(0);
    }

    for i in anagram_len..s.len() {
        // decrement char count at head or remove
        let mut head_count = anagram_checker[&(s[i - anagram_len] as char)];
        head_count -= 1;
        if head_count == 0 {
            anagram_checker.remove(&(s[i - anagram_len] as char)); 
        }

        // increment char count at tail
        let count = anagram_checker.entry(s[i] as char).or_insert(0);
        *count += 1;

        if anagram_checker == anagram_counts {
            anagram_indicies_out.push((i - anagram_len) + 1)
        }
    }
}

// 2.2
// Generate Palindrome Pairs
fn get_palindrome_pair_indicies(words: &Vec<String>, palindrome_pairs_out: &mut Vec<usize>) {
    
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_1_1() {
        let a = vec![1,2,3,4,5];
        let mut b = Vec::new();
        super::get_product_of_other(&a, &mut b);
        assert_eq!(b[0], 120);
        assert_eq!(b[1], 60);
        assert_eq!(b[2], 40);
        assert_eq!(b[3], 30);
        assert_eq!(b[4], 24);
    }

    #[test]
    fn test_1_1_2() {
        let a = vec![3,2];
        let mut b = Vec::new();
        super::get_product_of_other(&a, &mut b);
        assert_eq!(b[0], 2);
        assert_eq!(b[1], 3);
    }

    #[test]
    fn test_1_2() {
        let a = vec![1,0,3,7,9,8,4,6];
        let (start, end) = super::locate_smallest_window_to_sort(&a);
        assert_eq!(start, 0);
        assert_eq!(end, a.len() - 1);
    }

    #[test]
    fn test_1_2_2() {
        let a = vec![3,7,5,6,9];
        let (start, end) = super::locate_smallest_window_to_sort(&a);
        assert_eq!(start, 1);
        assert_eq!(end, 3);
    }

    #[test]
    fn test_1_3() {
        let a = vec![34, -50, 42, 14, -5, 86];
        let max_contig = super::get_max_contiguous_sum(&a);
        assert_eq!(max_contig, 137);
    }

    #[test]
    fn test_1_3_wrapping() {
        let a = vec![8, -1, 3, 4];
        let max_contig = super::get_max_contiguous_sum_wrapping(&a);
        assert_eq!(max_contig, 15);
    }

    #[test]
    fn test_1_4_naive() {
        let a = vec![3,4,9,6,1];
        let mut b = Vec::new();
        super::get_smaller_elements_to_right_naive(&a, &mut b);
        assert_eq!(b[0], 1);
        assert_eq!(b[1], 1);
        assert_eq!(b[2], 2);
        assert_eq!(b[3], 1);
        assert_eq!(b[4], 0);
    }

    #[test]
    fn test_1_4() {
        let a = vec![3,4,9,6,1];
        let mut b = Vec::new();
        super::get_smaller_elements_to_right(&a, &mut b);
        assert_eq!(b[0], 1);
        assert_eq!(b[1], 1);
        assert_eq!(b[2], 2);
        assert_eq!(b[3], 1);
        assert_eq!(b[4], 0);
    }

    #[test]
    fn test_2_1() {
        // eg s: abxaba, w: ab, out: [0,1,3,4]
        let s = String::from("abxaba").into_bytes();
        let w = String::from("ab");
        let mut out = Vec::new();
        super::get_anagram_start_indicies(&s, &w, &mut out);
        print!("{:?}", out);
        assert_eq!(out[0], 0);
        assert_eq!(out[1], 3);
        assert_eq!(out[2], 4);
    }
}
