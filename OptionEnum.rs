fn main(){
    let index = find_first_a(String::from("Ravi"));
    match index{
        Some(index)=>println!("{}", index),
        None => println!("a is not there")
    }
}

fn find_first_a(s: String)->Option<i32>{
    for (index, char) in s.chars().enumerate(){
        if char == 'a'{
            return Some(index as i32);
        }
    }
    return None;
}
