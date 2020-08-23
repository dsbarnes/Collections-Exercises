/*

Given a list of integers (create a Vec of 25 random numbers between 1 and 10), 
use a vector and return the mean (the average value), median (when sorted,
the value in the middle position), and mode;(the value that occurs most often;
a hash map will be helpful here) of the list.

Convert strings to pig latin. The first consonant of each word is moved to
the end of the word and “ay” is added, so “first” becomes “irst-fay.”
Words that start with a vowel have “hay” added to the end instead (“apple”
becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

Using a hash map and vectors, create a text interface to allow a user to
add employee names to a department in a company. For example, “Add Sally
to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of
all people in a department or all people in the company by department,
sorted alphabetically.

Each of the above examples should be it's own module.
*/

mod pig_latin;
use pig_latin::mod_pig_latin;

use std::env;
use std::io;
use std::collections::{ HashMap, hash_map::Entry };


fn main() {
    // Challenge 2
    println!("{}", mod_pig_latin::to_pig_latin("india".to_string()));
    println!("{}", mod_pig_latin::to_pig_latin("dingus".to_string()));
    println!("\n");

    // Challenge 3
    // This is a massivly error prone solution.
    // However, we're not to the chapter on error handling yet
    // That is my excuse.

    // Initialize an HashMap, and add some departments to work with:
    let mut employee_directory: HashMap<String, Vec<String>> = HashMap::new();
    employee_directory.insert(String::from("Development"), vec![]);
    employee_directory.insert(String::from("Design"), vec![]);
    employee_directory.insert(String::from("Marketing"), vec![]);

    // Just like the guessing game
    // Should add some instructions
    loop {
        // capture the input and split it into a Vec<String>
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("");
        let commands: Vec<&str> = input.trim().split(" ").collect();

        // match the first argument to its functionality
        match commands[0] {
            // lists the departments and who's in them
            "list" => {
                // Could make this pretty... meh
                println!("{:?}", employee_directory);
            },
            // adds the employee to the department specafied
            // or crashes and burns if you give invalid input
            "add" => {
                let employee = commands[1];
                let department = employee_directory.get_mut(commands[2]).unwrap();
                department.push(employee.to_string());
                println!("{} added to {}", employee, commands[2]);
            },
            // breaks if the first command is neither list nor add
            _ => break,
        }
    }
}
