fn main() {
    // 1)
    // Given a list of integers, use a vector and return the median
    // (when sorted, the value in the middle position) and mode 
    // (the value that occurs most often; a hash map will be helpful here)
    // of the list

    let v: Vec<i8> = vec!(43, 23, 32, 34);

    let median = median(v.clone());
    
    match median {
        MaybeTuple::Tuple(tuple) => println!("{:?}", tuple),
        MaybeTuple::NotTuple(number) => println!("{}", number),
    }

    // 2)
    // Convert strings to pig latin. The first consonant of each word is moved
    // to the end of the word and “ay” is added, so “first” becomes “irst-fay.” 
    // Words that start with a vowel have “hay” added to the end instead (“apple” 
    // becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

    // 3)
    // Using a hash map and vectors, create a text interface to allow a user to add 
    // employee names to a department in a company. For example, 
    // “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve 
    // a list of all people in a department or all people in the company by department,
    // sorted alphabetically.
}

enum MaybeTuple {
    Tuple((i8, i8)),
    NotTuple(i8),
}

fn median(mut v: Vec<i8>) -> MaybeTuple {
    // TODO: try to implement my own sort function
    v.sort();
    if v.len() % 2 == 0 {
        MaybeTuple::Tuple((v[(v.len() / 2) - 1], v[v.len() / 2]))
    } else {
        MaybeTuple::NotTuple(v[v.len() / 2])
    }

}
