fn main(){
println!("{}", is_even(4));
}

fn is_even(a: i32) -> bool{
    if a%2==0 {
        return true
    } else {
        return false
    }
}
