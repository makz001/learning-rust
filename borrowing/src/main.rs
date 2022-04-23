fn main() {
    let s = String::from("string");
    let mut ss = String::from("string");

    borrow(&s);
    change(&mut ss);

    println!("{}", s);
    println!("{}", ss);
}

fn borrow(s: &String) {
    println!("borrowed {}", s);
}

fn change(s: &mut String) {
    *s = "hello".to_string();
}
