use itertools::Itertools;
use rand::prelude::*;
use rand::Rng;
use std::vec;
use std::{collections::HashMap, io};
fn main() {
    println!("input length: ");
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let length: u8 = input_line.trim().parse().expect("Input not an u8 integer");

    let mut int_pool: Vec<u8> = vec![];

    for i in 0..10 {
        int_pool.push(i);
    }
    //    println!("{:?} {}", int_pool, int_pool.len());

    let mut ab_map: HashMap<[u8; 2], Vec<Vec<u8>>> = HashMap::new();

    for b in 0..length + 1 {
        for a in 0..length - b + 1 {
            ab_map.insert([a, b], Vec::new());
        }
    }
    println!("length of map generated: {}", ab_map.len());

    // ab_map.insert([1, 1], vec![vec![1u8, 3u8, 2u8]]);

    // let tes = ab_map.entry([1, 1]).or_insert(vec![]);

    // tes.push(vec![2u8, 1u8, 3u8]);

    let mut rand_1: Vec<u8> = vec![];
    let mut rng = rand::thread_rng();

    // generate initial random num
    for _ in 0..length {
        let index = rng.gen_range(0..int_pool.len());
        rand_1.push(int_pool[index]);
        int_pool.remove(index);

        println!("{:?}", &int_pool.len());
    }

    let mut anspool_seq: Vec<Vec<u8>> = vec![];
    for perm in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().permutations(3) {
        anspool_seq.push(perm);
    }

    println!("{:?}", anspool_seq);

    // let anspool_seq = permute(rand_1);

    // println!("{:?}", anspool_seq);

    // println!("{:?}", rand_1);

    //    println!("{:?}", ab_map.get(&[1, 1]).unwrap());

    //   println!("{:?}", &ab_map);
}

fn permute(nums: Vec<u8>) -> Vec<Vec<u8>> {
    let mut vec = Vec::new();

    if nums.len() == 1 {
        vec.push(nums);
    } else {
        for (i, _item) in nums.iter().enumerate() {
            let left: u8 = nums[i];

            let mut v1: Vec<&[u8]> = vec![];

            v1.push(&nums[0..i]);
            v1.push(&nums[i + 1..]);

            let right = v1.concat();

            let arr = permute(right);

            for (j, _item2) in arr.iter().enumerate() {
                let mut vec2 = Vec::new();

                vec2.push(left);

                for item3 in &arr[j] {
                    vec2.push(*item3);
                }

                vec.push(vec2);
            }
        }
    }

    vec
}
