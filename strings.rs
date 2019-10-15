fn main(){
    let s1 = String::from("Cookies!");
    let s2 = s1.clone();
    let s1_len = calculate_length(&s1);
    println!("{}", s2);
    println!("The length of s1 is: {}", s1_len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

