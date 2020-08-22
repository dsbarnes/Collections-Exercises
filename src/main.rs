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
mod test;
use test::{foo, bar};

use import_me::custom_lib;

fn main() {
    foo::dog();
    bar::cat();

    custom_lib::function();
}
