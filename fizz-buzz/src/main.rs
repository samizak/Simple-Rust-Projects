fn main() {
    fizz_buzz();
    fizz_buzz_with_match();
}

fn fizz_buzz() {
    for i in 1..100000 {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{i}");
        }
    }
}

fn fizz_buzz_with_match() {
    for i in 1..100000 {
        match (i % 3, i & 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{i}"),
        }
    }
}
