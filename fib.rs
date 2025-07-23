fn main() {
    println!("{}", fib(10));
}

fn fib(num: u64) -> i64{
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }

    if num == 1 {
        return second;
    }

    for i in 1..num -2{
        let temp = second;
        second = second + first;
        first = temp;
    }

    return second;
}
