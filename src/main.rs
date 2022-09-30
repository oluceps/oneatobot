use core::panic;
use itertools::Itertools;
use rand::{thread_rng, Rng};
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

    // fill in with 0~9 integer
    for i in 0..10 {
        int_pool.push(i);
    }

    // HashMap  x,y -> assumed right answers pool
    let mut map_xy_anspool: HashMap<[u8; 2], Vec<Vec<u8>>> = HashMap::new();

    let mut rand_1: Vec<u8> = vec![];
    let mut rng = rand::thread_rng();

    // generate initial random num
    for _ in 0..length {
        let index = rng.gen_range(0..int_pool.len());
        rand_1.push(int_pool[index]);
        int_pool.remove(index);

        //println!("{:?}", &int_pool.len());
    }
    // rand num generate complete

    let mut primitive_anspool_with_sequence: Vec<Vec<u8>> = vec![];
    for perm in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        .into_iter()
        .permutations(length.into())
    {
        // perm nums with spec length
        primitive_anspool_with_sequence.push(perm);
    }

    // CLASSIFICATION
    for i in primitive_anspool_with_sequence.iter() {
        // println!("comparing {:?}{:?} \n{:?}", &i, &rand_1, check_ans(i, &rand_1));

        // find entry in map_xy_anspool, and push into empty Vec
        let ent = map_xy_anspool.entry(check_ans(i, &rand_1)).or_default();
        ent.push(i.to_vec());
    }

    println!("{:?}", map_xy_anspool.len());

    // println!("{:?}\n {:?}", rand_1, map_xy_anspool);

    // filter empty map in map_xy_anspool
    for (key, item) in map_xy_anspool.clone() {
        // println!("{:?}  {:?}", &key, &item);
        if item.is_empty() {
            map_xy_anspool.remove(&key).unwrap();
        }
    }

    println!("{:?}", map_xy_anspool);

    // LOOP
    loop {
        // TODO: get the list in this step
        let fdback = feedback();

        let loop_anspool = match map_xy_anspool.get(&fdback) {
            Some(v) => v.clone(),
            None => panic!("AB input error"),
        };

        // clear map_xy_anspool
        // choose next num, count derivations

        let next_num = choose(&loop_anspool);

        println!("\n\nthe next num : {:?}", &next_num);
        // vec of num for next gen
        let vec_map_xy_anspool_val = map_xy_anspool.get(&fdback).unwrap();

        primitive_anspool_with_sequence.retain(|x| vec_map_xy_anspool_val.contains(x));

        // reinit map
        map_xy_anspool.clear();

        println!("map_xy_anspool now is : \n{:?}", &map_xy_anspool);

        // CLASSIFICATION in loop anspool
        // complete: shrink primitive pool (IMPORTANT)
        for i in primitive_anspool_with_sequence.iter() {
            // find entry in map_xy_anspool, and push into empty Vec
            let ent = map_xy_anspool.entry(check_ans(i, &next_num)).or_default();
            ent.push(i.to_vec());
        }
        println!("\n\nmap after cmp & add: \n {:?}", &map_xy_anspool);
    }
}

// should return back an Vec with len as length
fn choose(raw: &Vec<Vec<u8>>) -> &Vec<u8> {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..raw.len());
    raw.get(index).unwrap()
}

// TODO: ref
fn feedback() -> [u8; 2] {
    println!("A:");
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let a: u8 = input_line.trim().parse().expect("Input not an u8 integer");

    println!("B:");
    input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let b: u8 = input_line.trim().parse().expect("Input not an u8 integer");
    [a, b]
}

fn init_anspool(
    length: &u8,
    mut map_xy_anspool: HashMap<[u8; 2], Vec<Vec<u8>>>,
) -> HashMap<[u8; 2], Vec<Vec<u8>>> {
    // all probablity of xAyB under specific length
    for b in 0..length + 1 {
        for a in 0..length - b + 1 {
            map_xy_anspool.insert([a, b], Vec::new());
        }
    }
    map_xy_anspool
}

fn check_ans<'a>(num: &'a Vec<u8>, num_1: &'a Vec<u8>) -> [u8; 2] {
    let mut a: u8 = 0;
    let mut b: u8 = 0;

    for (i, element) in num.iter().enumerate() {
        if num_1.get(i).unwrap() == element {
            a += 1;
        }
        if num_1.contains(element) && num_1.get(i).unwrap() != element {
            b += 1;
        }
    }
    let res: [u8; 2] = [a, b];
    res
}
