#![allow(dead_code, unused_variables)]
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

}

fn longest<'a>(s: &'a str, ss: &'a str) -> &'a str {
    if s.len() > ss.len() {s} else {ss}
}

// lifetime annotations in struct definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn f1() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// lifetime annotations in method definitions
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

