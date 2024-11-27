// you can use includes, for example:
// use std::cmp;

// you can write to stdout for debugging purposes, e.g.
// println!("this is a debug message");

use std::collections::HashSet;

// pub fn solution(A: &mut Vec<i32>) -> bool {
//     let mut is_odd = 0;
//     let mut is_even = 0;

//     for num in A.iter() {
//         if num % 2 == 0 {
//             is_even += 1
//         } else {
//             is_odd += 1
//         }
//     }

//     is_odd == is_even
// }

// A = [2]
// T = [0, 0, 1, 1]
pub fn solution3(T: &mut Vec<i32>, A: &mut Vec<i32>) -> i32 {
    // Using Hash Sets for uniqure elements
    let mut required_skills = HashSet::new();

    for skill in A {
        let mut current_skill = *skill;

        while !required_skills.contains(&current_skill) {
            required_skills.insert(current_skill);


            // If current skill is greater than T.len
            if (current_skill as usize) >= T.len() {
                break;
            }

            current_skill = T[current_skill as usize];

            // If we reach the root skill (0), break
            if current_skill == 0 {
                required_skills.insert(current_skill);
                break;
            }
        }
    }

    required_skills.len() as i32
}


pub fn solution(S: &mut String) -> i32 {
    // Using HashSet would be perfect choice here since 
    // We need to track uniqueness of chars
    let mut seen = HashSet::new();
    let mut split = 1; // Start with one substring

    for c in S.chars() {
        if seen.contains(&c) {
            // If the character is already seen, start a new split count
            split += 1;
            seen.clear(); 
        }
        seen.insert(c); // Add the current character to the set
    }

    split
}

// fn main() {
//     let mut s1 = String::from("world");
//     println!("Result: {}", solution(&mut s1)); // Output: 1

//     let mut s2 = String::from("dddd");
//     println!("Result: {}", solution(&mut s2)); // Output: 4

//     let mut s3 = String::from("cycle");
//     println!("Result: {}", solution(&mut s3)); // Output: 2

//     let mut s4 = String::from("abba");
//     println!("Result: {}", solution(&mut s4)); // Output: 2
// }
// fn main() {
//     let mut test1 = vec![2, 7, 4, 6, 3, 1];
//     println!("Test 1: {}", solution(&mut test1)); // Expected: true

//     let mut test2 = vec![-1, 1];
//     println!("Test 2: {}", solution(&mut test2)); // Expected: false

//     let mut test3 = vec![2, -1];
//     println!("Test 3: {}", solution(&mut test3)); // Expected: true

//     let mut test4 = vec![1, 2, 4, 3];
//     println!("Test 4: {}", solution(&mut test4)); // Expected: true

//     let mut test5 = vec![-1, -3, 4, 7, 7, 7];
//     println!("Test 5: {}", solution(&mut test5)); // Expected: false
// }

fn main() {
    let mut t1 = vec![0, 0, 1, 1];
    let mut a1 = vec![2];
    println!("Test 1: {}", solution3(&mut t1, &mut a1)); // Output: 3

    let mut t2 = vec![0, 0, 1, 2];
    let mut a2 = vec![1, 2];
    println!("Test 2: {}", solution3(&mut t2, &mut a2)); // Output: 2

    let mut t2 = vec![0, 0, 0, 2, 3, 3];
    let mut a2 = vec![2, 5, 6];
    println!("Test 2: {}", solution3(&mut t2, &mut a2)); // Output: 5

    let mut t3 = vec![0, 3, 0, 5, 0, 5];
    let mut a3 = vec![4, 2, 6, 1, 0];
    println!("Test 3: {}", solution3(&mut t3, &mut a3)); // Output: 7
}
