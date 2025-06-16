use std::io;
pub fn parser () {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Could not read user input");
    
    print!("{}", "Enter a Integer: ");
    println!("{}", user_input);
}
