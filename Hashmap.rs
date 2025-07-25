use std::collections::HashMap;

fn main(){
    let input_vec = vec![(String::from("Ravi"), 22), (String::from("Ravi sorathiya"), 23), (String::from("Ravi vaniya"), 24)];
    let hm = make_hashmap(input_vec);
    println!("{:?}", hm);
}

fn make_hashmap(vec: Vec<(String, i32)>) -> HashMap<String, i32>{
    let mut hm = HashMap::new();
    for (key, value) in vec{
        hm.insert(key, value);
    }

    return hm;
}
