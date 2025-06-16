use std::io;
fn parser (user_input:&mut) {
    user_input = &String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Could not read user input");
}