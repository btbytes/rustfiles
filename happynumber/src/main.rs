/*
main.rs

print the first 8 happy numbers.
Pradeep Gowda
2014/11/21
*/
#[allow(unused_imports)]
use std::error::Error;

fn sum_square_digits(num: i32) -> i32 {
    let s: String = num.to_string();

    println!("s: {}", s);


    for c in s.chars() {
        print!("{}", c);
    }
    return 0;
}


fn is_happy_number(num: i32) -> bool {
    let sqsum: i32 = sum_square_digits(num);
    match sqsum {
        0 => false,
        1 => true,
        2 ... 9 => false,
        _ => is_happy_number(sqsum),
    }
}


fn main() {
    //println!("Hello, world!")
    let mut hncount = 0;
    fn foobar(driver: i32) -> i32 {
        return driver * 33;
    }
    foobar (2);
    println!("Number of happy numbers: {}", hncount);
    let mut num: i32 = 1;
    while hncount < 8 {
        //number_to_vec(hncount);
        if is_happy_number(num) {
           hncount += 1;
        }
        num += 1;
    }
}
