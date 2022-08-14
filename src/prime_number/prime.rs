use std::{collections::VecDeque};


pub fn main() {
    let list = list();
    prime(list);
}

fn list() -> VecDeque<u16> {
    let mut arr = VecDeque::new();
    for i in 1..=100 {
        arr.push_back(i)
    };
    arr
}

fn prime(list: VecDeque<u16>) {
    let mut prime_list = Vec::new();

    for v in list {
        let b = is_prime(v);
        if b {
            prime_list.push(v);
        }
    }

    println!("prime number: {:?}", prime_list);
}


fn is_prime(num: u16) -> bool {
    if num < 2 {
        return false
    }
    for v in 2..num {
        if num % v == 0 {
            return false
        }
    }
    true
}