use std::io::Write;

///
/// Recieves user input and returns
///
pub fn input(display_roll_txt: bool) -> String {
    // Print a input prompt;
    if display_roll_txt {
        print!("Roll dice (type LOG to show log, or EXIT to exit) \n >> ");
    }
    std::io::stdout().flush().unwrap();

    // Write input to String and return
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp).unwrap();

    // Trim the '\n' off end of String (since
    // read_line writes end-line onto String)
    return format!("{}", inp.trim_end_matches('\n'));
}

///
/// Waits for user to press ENTER before continuing
///
pub fn wait_for_enter() {
    print!("Press ENTER to continue...");
    let _ = input(false);
}
