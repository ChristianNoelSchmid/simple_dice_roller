use std::num::ParseIntError;

use rand::RngCore;

///
/// An enum that stores the possible values that can
/// be added to a roll.
///
#[derive(PartialEq, Eq)]
pub enum RollValue {
    Constant(u32),     // a constant value, added to random rolls
    DieRoll(u32, u32), // a die count, and die type (count, type)
}

///
/// Displays all the rolls in `all_rolls`
///
pub fn display_log(all_rolls: &Vec<Vec<RollValue>>) {
    if all_rolls.len() == 0 {
        println!("No rolls recorded.");
    } else {
        for (index, rolls) in all_rolls.iter().enumerate() {
            // Mark the roll index
            let mut marker = String::from("\n-- ROLL ");
            marker.push_str(&(index + 1).to_string());

            // Add latest tag if it is the last one in the list
            if *rolls == *all_rolls.last().unwrap() {
                marker.push_str(" <LATEST>");
            }

            println!("{}", marker);
            display_roll(rolls);
        }
        println!();
    }
}

///
/// Takes a string and splits it into dice rolls
/// returning error messages on unsuccessfuly parses
///
pub fn parse_roll<'a>(roll: &'a str) -> Vec<RollValue> {
    let mut roll_values = Vec::<RollValue>::new();

    // Split by '+'s
    for v in roll.split('+') {
        let val = v.trim();
        // Rolls can start with 'd', with an implied 1 die
        if val.starts_with("d") {
            let typ = val[1..].parse::<u32>();
            match typ {
                Ok(i) => roll_values.push(RollValue::DieRoll(1, i)),
                Err(_) => println!("Couldn't read value: {}", val),
            }
        // ... or can have a number before 'd' to signify die count
        } else if val.contains("d") {
            let ints = val
                .split('d')
                .map(|i| i.trim().parse::<u32>())
                .collect::<Vec<Result<u32, ParseIntError>>>();
            if ints.len() != 2 || ints.iter().any(|result| result.is_err()) {
                println!("Couldn't read value: {}", val);
            } else {
                roll_values.push(RollValue::DieRoll(
                    ints[0].clone().unwrap(),
                    ints[1].clone().unwrap(),
                ));
            }
        // ... otherwise, attempt to convert the segment to an integer
        } else {
            let i = v.trim().parse::<u32>();
            match i {
                Result::Ok(i) => roll_values.push(RollValue::Constant(i)),
                Result::Err(_) => {
                    println!("Couldn't read value: {}", v);
                }
            }
        }
    }

    return roll_values;
}

///
/// Displays a list of `RollValue`s, with the die type per die,
/// the roll value per die, and the total at the end.
///
pub fn display_roll(values: &Vec<RollValue>) {
    let mut total = 0u32;
    let mut rnd = rand::thread_rng();

    for v in values {
        match v {
            RollValue::Constant(i) => {
                println!("  + {}", i);
                total += i;
            }
            RollValue::DieRoll(c, t) => {
                for _ in 0..*c {
                    let val = rnd.next_u32() % t + 1;
                    println!("  d{} = {}", t, val);
                    total += val;
                }
            }
        }
    }

    println!("    = {}", total);
}
