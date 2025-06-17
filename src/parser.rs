use std::io;
pub fn parser () {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Could not read user input");
    let mut user_input:&str = user_input.trim();
    println!("{}", user_input);
    user_input = "";
    
}
