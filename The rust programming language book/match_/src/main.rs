#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
   let coin = Coin::Nickel;
   println!("{}", value_in_cents(coin));

   let coin = Coin::Quarter(UsState::Alaska);
   println!("{}", value_in_cents(coin));

   // Catch-all Patterns and the _ Placeholder
   
   let dice_roll = 9;
   match dice_roll {
       3 => add_fancy_hat(),
       7 => remove_fancy_hat(),
       other => move_player(other),
   }

   //   _ Placeholder

   let dice_roll = 9;
   match dice_roll {
       3 => add_fancy_hat(),
       7 => remove_fancy_hat(),
       _ => reroll(),
   }

   let dice_roll = 9;
   match dice_roll {
       3 => add_fancy_hat(),
       7 => remove_fancy_hat(),
       _ => (),
   }

   // if let syntax
   
   //   with match:

   let config_max = Some(3u8);
   match config_max {
       Some(max) => println!("The maximun is configured to be {}", max),
       _ => (),
   }

   //   with if let:
   
   let config_max = Some(3u8);
   if let Some(max) = config_max {
       println!("The maximun is configured to be {}", max);
   }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

fn reroll() {}
