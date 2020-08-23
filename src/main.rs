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

enum Command {
    List,
    Add,
}

fn main() {
    // Challenge 2
    println!("{}", mod_pig_latin::to_pig_latin("india".to_string()));
    println!("{}", mod_pig_latin::to_pig_latin("dingus".to_string()));

    let mut sls = vec!["Joe", "Ben", "Matt", "Caleb"];
    let mut dev = vec!["Kevin"];
    let mut des = vec!["Mark"];
    let mut mkt = vec!["Sally"];
    let mut mgt = vec!["Aaron"];


    // See the guessing game for how to continually take input
    // and push that new input to the vecs

    loop{
        let args: Vec<String> = env::args().collect();

        let command = match args[1].as_str() {
            "add" => Command::Add,
            "list" => Command::List,
            _ => panic!(),
        };


        match command {
            Command::Add => {
                // let name = args[2].as_str();
                match args[3].as_str() {
                    "sales" => {
                        sls.push(args[2].as_str());
                        println!("{} was added to {}", args[2], args[3]);
                        println!("{:?}", sls)
                    },

                    "development" => {
                        dev.push(args[2].as_str());
                        println!("{} was added to {}", args[2], args[3].as_str());
                        println!("{:?}", dev)
                    },

                    "design" => {
                        des.push(args[2].as_str());
                        println!("{} was added to {}", args[2], args[3].as_str());
                        println!("{:?}", des);
                    },

                    "marketing" => {
                        mkt.push(args[2].as_str());
                        println!("{} was added to {}", args[2], args[3].as_str());
                        println!("{:?}", mkt);
                    },

                    "management" => {
                        mgt.push(args[2].as_str());
                        println!("{} was added to {}", args[2], args[3].as_str());
                        println!("{:?}", mgt);
                    },
                    _ => panic!(),
                }
            },

            Command::List => {
                println!("Sales\n{:?}", sls);
                println!("Development\n{:?}", dev);
                println!("Design\n{:?}", des);
                println!("Marketing\n{:?}", mkt);
                println!("Management\n{:?}", mgt);
            },
        }
    }    
}
