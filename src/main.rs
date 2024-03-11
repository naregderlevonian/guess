use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colorize::AnsiColor;

fn main() {
    // ASCII art logos for the game and the win screen
    let game_logo = String::from("\n\
        ####   # ###  #   ###      ####   # ####   # \n\
        ###  #   ###  #   ###  ### ###  ##  ###  ##  \n\
        ###  ### ##  ##   ###    # ###    # ###    # \n\
        ##  #    ##  ##   ##  #### #####    #####    \n\
        ##   #   ##  #    ##  #### ## ###   ## ###   \n\
        ##     # ##     # #      # ###    # ###    # \n\
        ######## ######## #  ##### ######## ######## \n\
        ######## ######## ######## ######## ######## \n");

    let win_screen = String::from("\n\
        ####   # ###  #   ###      ####   # ####   # \n\
        YOU WIN! YOU WIN! YOU WIN! YOU WIN! YOU WIN! \n\
        ######## ######## ######## ######## ######## \n");

    // Printing the game logo in bold and grey
    println!("{}", game_logo.clone().bold().b_grey());

    // Generating a random number between 1 and 1000 (inclusive)
    let generated_number = rand::thread_rng().gen_range(1..=1000);

    // Strings for user prompts and messages
    let prompt_message = String::from("Enter number (from 0 to 1000): ");
    let incorrect_number_message = String::from("Incorrect number!");
    let small_result_message = String::from("Too small!");
    let big_result_message = String::from("Too big!");

    // Printing the cheat/debug information (generated number)
    // println!("Cheat: {generated_number}");

    // Game loop
    loop {
        // User input for guessing the number
        let mut user_input = String::new();

        // Prompting the user to enter a number
        println!("{}", prompt_message.clone().bold().b_black());
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line!");

        // Converting user input to a u32, handling errors
        let user_guess: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", incorrect_number_message.clone().red());
                continue;
            }
        };

        // Comparing user input with the generated number
        match user_guess.cmp(&generated_number) {
            Ordering::Less => println!("{}", small_result_message.clone().b_blue()),
            Ordering::Greater => println!("{}", big_result_message.clone().b_yellow()),
            Ordering::Equal => {
                // If the guess is correct, print the win screen and exit the loop
                println!("{}", win_screen.clone().bold().green());
                break;
            }
        }
    }
}

