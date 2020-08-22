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

// use unicode_segmentation::UnicodeSegmentation;
use std::str;

fn main() {
    // Challenge 1: Mean, Median, Mode

    // Challenge 2: Convert to pig latin
    fn to_pig_latin(input: String) -> String {
        let mut byte_vec: Vec<u8> = input.into_bytes();

        let first_char: u8 = byte_vec.remove(0);
        let ending: Vec<u8> = String::from("-ay").into_bytes();

        byte_vec.push(first_char);
        byte_vec.extend_from_slice(&ending);

        let pig_str: &str= str::from_utf8(&byte_vec).unwrap();
        String::from(pig_str)
    }

    println!("{}", to_pig_latin("Shit".to_string()));
}
