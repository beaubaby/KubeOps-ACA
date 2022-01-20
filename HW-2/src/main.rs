use std::io;
use regex::Regex;

fn main() {

}

fn check_palindrome(word: String) {
    // set all Alphanumeric to Lowercase to prepare pre-word //
    let pre_word: String = word.trim().to_lowercase();
    let palindrome_word: &str = pre_word.as_str();

    // remove
    let regex = Regex::new(r"[^a-zA-Z0-9ก-๛]").unwrap();
    let palindrome_word = &regex.replace_all(palindrome_word, "");


}
