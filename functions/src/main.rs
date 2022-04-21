fn main() {
    // another_function(5);

    // print_labeled_measurement(5, 'h');
    
    // let x = {
    //     let y = 3;
    //     y + 1
    // };
    // println!("The value of x is: {}", x);

    // let x = five();
    // println!("The value of x is: {}", x);
    
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn _another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn _print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn _five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
