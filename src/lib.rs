#![allow(dead_code)]
#![allow(non_upper_case_globals)]

mod config {
    const PORT: u16 = 12345;
}

const numbers: [u8; 6] = [0u8, 1, 2, 3, 4, 5];

pub fn hello() {
    println!("Hello, world!");
    for n in numbers {
        println!("{}", n);
    }
    println!("{:?}", numbers);
}
