#![allow(dead_code)]
#![allow(non_upper_case_globals)]

mod config {
    const PORT: u16 = 12345;
}

pub fn hello() {
    let numbers: Vec<u8> = vec![0u8, 1, 2, 3, 4, 5];
    println!("Hello, world!");
    for n in &numbers {
        println!("{}", n);
    }
    println!("{:?}", numbers);
}
