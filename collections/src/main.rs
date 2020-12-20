use std::collections::VecDeque;
use std::collections::LinkedList;
use std::collections::HashMap;

fn vecdque() {
    // A double-ended queue implemented with a growable ring buffer.
    let mut v : VecDeque<u32> = VecDeque::new();
    v.push_front(1);
    v.push_front(2);
    v.push_front(3);
    println!("{:?}", v);
    println!("{:?}", v.get(0));
    v.swap(0, 1);
    println!("{:?}", v);
    for i in v.iter_mut() {
        *i = *i * 2;
    }
    println!("{:?}", v);
    let drained = v.drain(2..).collect::<VecDeque<_>>();
    println!("{:?}", drained);
    println!("{:?}", v);
    v.insert(1, 9);
    println!("{:?}", v);
    v.rotate_left(1);
    println!("{:?}", v);
}

fn likedlist() {
    let mut list: LinkedList<u32> = LinkedList::new();
    let mut list2: LinkedList<u32> = LinkedList::new();
    list.push_front(1);
    list2.push_front(2);
    list.append(&mut list2);
    println!("{:?}", list);
    println!("{:?}", list.contains(&1));
    println!("{:?}", list.contains(&3));
    println!("{:?}", list.front());
    println!("{:?}", list.pop_back());
    println!("{:?}", list);
    println!("{:?}", list2);
    list.push_back(3);
    println!("{:?}", list);
    println!("{:?}", list2);
    list2.push_back(4);
    println!("{:?}", list);
    println!("{:?}", list2);
    let split = list.split_off(1);
    println!("{:?}", list);
    println!("{:?}", split);
}

fn hashmap() {
    let mut hm = HashMap::new();
    hm.insert("hoge", "zzz".to_string());
    println!("{:?}", hm);
    println!("{:?}", hm.contains_key("hoge"));
    println!("{:?}", hm.contains_key("fuga"));
    match hm.remove("hoge") {
        Some(value) => println!("{:?}", value),
        None => (),
    };
    match hm.remove("fuga") {
        Some(value) => println!("{:?}", value),
        None => println!("not found"),
    };
    let mut letters = HashMap::new();
    for ch in "stand by you".chars() {
        let counter = letters.entry(ch).or_insert(0);
        *counter += 1;
    }
    for (key, val) in letters.iter() {
        // not stable
        println!("{}: {}", key, val);
    }
}

fn main() {
    vecdque();
    println!("linkedlist");
    likedlist();
    println!("hashmap");
    hashmap();
}
