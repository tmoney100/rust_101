// Import the String type from the standard library.
use std::string::String;

// Define a function called pattern_match that takes a name (of type &str) as an argument.
fn pattern_match(name: &str) -> String {
    // Use a match expression to determine the return value based on the name argument.
    match name {
        // If the name is "Susan", return the string "Hello, Susan".
        "Susan" => "Hello, Susan".to_string(),
        // If the name is "Karen", return the string "Goodbye, Karen".
        "Karen" => "Goodbye, Karen".to_string(),
        // For any other name, use the format! macro to return "I don't know you, [name]".
        x => format!("I don't know you, {}", x),
    }
}

fn main() {
    // Print the result of calling the pattern_match function with "Susan" as an argument.
    println!("{}", pattern_match("Susan"));
    // Print the result of calling the pattern_match function with "John" as an argument.
    println!("{}", pattern_match("John"));
}
