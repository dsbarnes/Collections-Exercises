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
use std::collections::HashMap;


fn main() {
    // Challenge 2
    println!("{}", mod_pig_latin::to_pig_latin("india".to_string()));
    println!("{}", mod_pig_latin::to_pig_latin("dingus".to_string()));

    let employee_directory: HashMap<String, Vec<String>> = HashMap::new();

    loop{
        println!("Enter a command:");
        println!("    list [ department ]");
        println!("    add [ name ] [ department ]\n>>>");

        // let args: Vec<String> = env::args().collect();
        let mut args = String::new();

        io::stdin().read_line(&mut args)
            .expect("failed to read line");

        match args.as_str().trim() {
            "again" => {
                println!("doing it again!");
                // Add anything at all to the hashmaps vec
            },
            _ => {
                println!("Got {}: Breaking", args.trim());   
                break;
            }
        }
    }    
}
