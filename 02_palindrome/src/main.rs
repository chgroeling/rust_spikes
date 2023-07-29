/// Detect strings representing palindromes in two different ways.

use std::iter::zip;

/// Changes `is_palindrome` to true if `word` is a palindrome. False otherwise.
///
/// # Arguments
///
/// * `word` : a string slice that holds the word to be checked
/// * `is_palindrome` : a mutable bool slice to hold the result of this algorithmn.
fn palindrome_fct1(word: &str, is_palindrome: &mut bool) {
    *is_palindrome = true;
    for (a, b) in zip(word.chars(), word.chars().rev()) {
        if a.to_ascii_uppercase() != b.to_ascii_uppercase() {
            *is_palindrome = false;
        }
    }
}

/// Returns true if `word` is a palindrome
///
/// # Arguments
///
/// * `word` : a string slice that holds the word to be checked
fn palindrome_fct2(word: &str) -> bool {
    let mut is_palindrome = true;
    for (a, b) in zip(word.chars(), word.chars().rev()) {
        if a.to_ascii_uppercase() != b.to_ascii_uppercase() {
            is_palindrome = false;
        }
    }

    return is_palindrome;
}

fn main() {
    let a: &str = "RACeCAR";

    let mut is_palindrome = true;
    palindrome_fct1(a, &mut is_palindrome);
    print!("-> palindrom_fct1('{a}'): {is_palindrome}\n");

    let is_palindrome = palindrome_fct2(a);
    print!("-> palindrom_fct2('{a}'): {is_palindrome}\n");
}
