fn main() {
    let mut n = 0;
    let mut nx = 1;

    // 1 2 3 5 8 13 21
    // result = n + nx && n = nx && nx = result

    println!("0");
    println!("1");

    for _ in 1..11 {
        let result = n + nx;
        n = nx;
        nx = result;

        println!("{}", result);
    }
}

