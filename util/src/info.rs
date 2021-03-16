
use std::collections::{HashSet, HashMap};


pub fn print_vector_i32(v: &Vec<i32>) {
    for val in v.iter() {
        print!("{} ", val);
    }
    println!();
}

pub fn print_hash_set_i32(s: &HashSet<i32>) {
    for val in s.iter() {
        print!("{} ", val);
    }
    println!();
}

pub fn print_hash_set_u32(s: &HashSet<u32>) {
    for val in s.iter() {
        print!("{} ", val);
    }
    println!();
}
pub fn print_map_i32_to_i32(m : &HashMap<i32, i32>) {
    for (val, key) in m.iter() {
        println!("{} -> {}", val, key);
    }
}

pub fn print_map_i32_to_vector_i32(m: &HashMap<i32, Vec<i32>>) {
    for (val, key) in m.iter() {
        print!("{} -> {{", val);
        for inner_val in key.iter() {
            print!("{} ", inner_val);
        }
        println!("}}");
    }
}

/// elements in v1, v2 are in order
/// check whether v2 is contained in v1
pub fn imply(v1 : &Vec<i32>, v2: &Vec<i32>) -> bool {
    if v1.len() < v2.len() {
        return false;
    }

    let mut first1 = 0usize;
    let last1 = v1.len() - 1;

    let mut first2 = 0usize;
    let last2 = v2.len() - 1;

    while first2 != last2 {
        if (first1 == last1) || comp(v1[first1], v2[first2]) {
            println!("v2[{}] = {}, v1[{}] = {}", first2, v2[first2], first1, v1[first1]);

            println!("comp: {}", comp(v2[first2], v1[first1]));
            return false;
        }

        if v1[first1] == v2[first2] {
            first2 += 1;
        }
        first1 += 1;
        println!("first1 = {}, first2 = {}", first1, first2);
    }
    println!("end");
    return true;
}

#[test]
fn test_imply() {
    let v1 = vec![1, 2, 3, 4];
    let v2 = vec![2, 3, 4, 5, 6];
    assert_eq!(imply(&v1, &v2), true);
}


pub fn cub_intersect(v1: &Vec<i32>, v2: &Vec<i32>) -> Vec<i32> {
    vec_intersect(v1, v2)
}

fn vec_intersect(v1: &Vec<i32>, v2: &Vec<i32>) -> Vec<i32> {
    unimplemented!()
}

fn comp(i: i32, j: i32) -> bool {
    i.abs() < j.abs()
}