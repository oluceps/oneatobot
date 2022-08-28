use itertools::Itertools;
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
    //println!("length of map generated: {}", ab_map.len());

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

        //println!("{:?}", &int_pool.len());
    }

    let mut anspool_seq: Vec<Vec<u8>> = vec![];
    for perm in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        .into_iter()
        .permutations(length.into())
    {
        anspool_seq.push(perm);
    }

//    println!("{:?} \n {:?}", rand_1, anspool_seq);

    for i in anspool_seq.iter() {
        //        println!("comparing {:?}{:?} \n{:?}", &i, &rand_1, check_ans(i, &rand_1));
        let ent = ab_map.entry(check_ans(i, &rand_1)).or_default();
        ent.push(i.to_vec());
    }

    println!("{:?}\n {:?}",rand_1, ab_map);
    // let anspool_seq = permute(rand_1);

    // println!("{:?}", anspool_seq);

    // println!("{:?}", rand_1);

    //    println!("{:?}", ab_map.get(&[1, 1]).unwrap());

    //   println!("{:?}", &ab_map);
}

// nums from anspool_seq;
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
