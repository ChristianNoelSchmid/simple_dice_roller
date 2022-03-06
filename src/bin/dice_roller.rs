use clearscreen;
use std::process::exit;

use dice_roller::dice;
use dice_roller::io;

fn main() {
    // A record of all the rolls made
    let mut all_rolls = Vec::new();
    clearscreen::clear().expect("failed to clear screen");

    loop {
        match parse_input() {
            Action::Roll(str) => {
                let rolls = dice::parse_roll(&str);
                dice::display_roll(&rolls);
                all_rolls.push(rolls);
            }
            Action::Log => dice::display_log(&all_rolls),
            Action::Exit => exit(0),
        }

        io::wait_for_enter();
        clearscreen::clear().expect("failed to clear screen");
    }
}

///
/// The actions that can be taken by the user upon input
///
enum Action {
    Roll(String), // The user wishes to calculate the given roll
    Log,          // The user wants to see the roll log
    Exit,         // The user wants to exit the program
}

///
/// Parses the user's input, returning the appropriate
/// `Action` based on the input.
///
fn parse_input() -> Action {
    let res = io::input(true).trim().to_lowercase();
    return if res == "log" {
        Action::Log
    } else if res == "exit" {
        Action::Exit
    } else {
        Action::Roll(res)
    };
}
