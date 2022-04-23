fn main() {
    let s = String::from("string");
    let ss = String::from("string");

    give_ownership(s);
    let ss = return_ownership(ss);

    // println!("{}", s); // error: value moved!
    println!("{}", ss);
}

fn give_ownership(s: String) {
    println!("moved {}", s);
}

fn return_ownership(s: String) -> String {
    s
}
