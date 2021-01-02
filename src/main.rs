use std::num::ParseIntError;
use rand::*;
use std::io::Write;

/// 
/// An enum that stores the possible values that can
/// be added to a roll.
///
enum RollValue {
    Constant(u32), // a constant value, added to random rolls
    DieRoll(u32, u32) // a die count, and die type (count, type)
}

fn main() {
    loop {
        let rolls = parse_roll(&input(true));
        display_roll(rolls);
        wait_for_enter();
        clear_screen();
    }
}

fn parse_roll<'a>(roll: &'a str) -> Vec<RollValue> {
    let mut roll_values = Vec::<RollValue>::new();  
    for v in roll.split('+').collect::<Vec<&str>>() {
            let val = v.trim();
            if val.starts_with("d") {
                let typ = val[1..].parse::<u32>();    
                match typ {
                    Ok(i) => roll_values.push(RollValue::DieRoll(1, i)),
                    Err(_) => println!("Couldn't read value: {}", val)
                }
            } else if val.contains("d") {
                let ints = val.split('d').map(|i| i.trim().parse::<u32>())
                    .collect::<Vec<Result<u32, ParseIntError>>>();
                if ints.len() != 2 || ints.iter().any(|result | { result.is_err() }) {
                    println!("Couldn't read value: {}", val);
                } else {
                    roll_values.push(RollValue::DieRoll(ints[0].clone().unwrap(), ints[1].clone().unwrap()));
                }
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

fn display_roll(values: Vec<RollValue>) {
    let mut total = 0u32;
    let mut rnd = rand::thread_rng();

    for v in values {
        match v {
            RollValue::Constant(i) => {
                println!("  + {}", i);
                total += i;
            },
            RollValue ::DieRoll(c, t) => {
                for _ in 0..c {
                    let val = rnd.next_u32() % t + 1;
                    println!("  d{} = {}", t, val);
                    total += val;
                }
            }
        }
    } 

    println!("    = {}", total);
}

///
/// Recieves user input and returns
/// 
pub fn input(display_roll_txt: bool) -> String {
    // Print a input prompt;
    if display_roll_txt {
        print!("Roll dice >> "); 
    }
    std::io::stdout().flush().unwrap();

    // Write input to String and return
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp)
        .unwrap();

    // Trim the '\n' off end of String (since
    // read_line writes end-line onto String)
    return format!("{}", inp.trim_end_matches('\n'));
}

///
/// Waits for user to press ENTER before continuing
/// 
pub fn wait_for_enter()
{
    print!("Press ENTER to continue...");
    let _ = input(false);
}

///
/// Clears the console screen. Waits for
/// terminal to call command before returning, so
/// sequential lines are deleted in the lag
/// 
pub fn clear_screen()
{
    std::process::Command::new("clear").spawn()
        .expect("Failed to clear screen!")
        .wait()
        .expect("Failed to sleep program!");
}