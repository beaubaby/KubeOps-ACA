use std::io;
use regex::Regex;

fn main() {
    let mut word = String::new();
    println!("Please enter the word to check is it Palindrome or not : ");
    io::stdin().read_line(&mut word).expect("The word is incorrect, Please input again");
    check_palindrome(word);
}

fn check_palindrome(word: String) {
    // set all alphanumeric to Lowercase to prepare pre-word //
    let pre_word: String = word.trim().to_lowercase();
    let palindrome_word: &str = pre_word.as_str();

    // remove non-alphanumeric from the word
    let regex = Regex::new(r"[^a-zA-Z0-9ก-ฮ]").unwrap();
    let palindrome_word = &regex.replace_all(palindrome_word, "");

    let mut palindrome: bool = true;

    for (a1, a2) in palindrome_word.chars().enumerate() {
    if a1 == palindrome_word.len() / 2 {
        break;
        }
    if a2 != palindrome_word.chars().rev().nth(a1).unwrap() {
        palindrome = false;
        break;
        }
    }

    if palindrome {
        println!("\"{}\" is a palindrome word", word.trim());
    } else {
        println!("\"{}\" is not a palindrome word", word.trim());
    }
}
