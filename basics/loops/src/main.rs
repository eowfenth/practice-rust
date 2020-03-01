fn main() {
    println!("{}", fibonnaci(4));
}

fn fibonnaci(n: u32) -> u32 {
    let mut sum = 1;
    let mut last = 1;
    let mut curr = 1;
    for _number in 1..(n) {
        sum = last + curr;
        last = curr;
        curr = sum;
    }

    sum
}