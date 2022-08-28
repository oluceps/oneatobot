use rand::prelude::*;
use rand::Rng;
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
    


    println!("{:?}", rand_1);

    //    println!("{:?}", ab_map.get(&[1, 1]).unwrap());

    //   println!("{:?}", &ab_map);
}


