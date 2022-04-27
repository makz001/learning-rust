#[derive(Debug)]
struct User {
    user_name: String,
    email: String,
    password: String,
}

impl User {
    // create a new user
    fn new(user_name: String, email: String, password: String) -> User {
        User {
            user_name,
            email,
            password,
        }
    }

    // change password 
    fn change_password(&mut self, new_password: String ) {
        self.password = new_password;
    }

}


fn main() {

    let mut users = Vec::<User>::new();

    users.push(User::new(
        "makz".to_string(),
        "makz@gmail.com".to_string(),
        "12345".to_string(),
    ));

    println!("{:#?}", users[0]);

    println!("usr.password: {:?}", users[0].password);
    users[0].change_password("54321".to_string());
    println!("usr.password: {:?}", users[0].password);

}
