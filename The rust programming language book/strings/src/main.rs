fn main() {
    // Creating a new, empty String:
    let mut s = String::new();

    // Using the to_string method to create a String from a string literal:
    let data = "initial contents";

    let s = data.to_string();

    // -- the method also works on a literal directly:
    let s = "initial contents".to_string();

    // Using the String::from function to create a String from a string literal:

    let s = String::from("initial contents");


    // Storing greetings in different languages in strings: 

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // Appending a string slice to a String using the push_str method:

    let mut s = String::from("foo");
    s.push_str("bar");

    // Adding one character to a String value using push:

    let mut s = String::from("lo");
    s.push('l');

    // Using the + operator to combine two String values into a new String value:
    
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // Using the + operator to combine multiple String values into a new String value:
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // Using the format! macro to combine multiple String values into a new String value:

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3 );

    // Iteraring over a String width the chars method
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // Iteraring over a String width the bytes method
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }


}
