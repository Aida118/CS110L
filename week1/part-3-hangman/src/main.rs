// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    // Your code here! :)
    let mut count_suc = 0;
    let mut current_word: String = String::from("");
    for i in 0..secret_word_chars.len(){
        current_word.push_str("-");
    }
    let mut guess_times = 5;
    let mut guess_series: String = String::from("");
    while count_suc < secret_word_chars.len()
    {
        println!("The word so far is {:}", current_word);
        println!("You have guessed the following letters: {}", guess_series);
        println!("You have {} guesses left", guess_times);
        if guess_times == 0 {
            println!("Sorry, you ran out of guesses!");
            break;
        }
        println!("Please guess a letter: ");
        io::stdout()
            .flush()
            .expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        
        let guess_char = guess.trim().chars().next().unwrap(); 
        // 去掉首尾空白字符.遍历字符.取出第一个.把 Option<char> 类型转换成 char
        guess_series.push(guess_char);
        let mut flag = false;
        let mut current_word_chars: Vec<char> = current_word.chars().collect();
        for i in 0..secret_word_chars.len(){
            if guess_char == secret_word_chars[i]{
                current_word_chars[i] = guess_char;
                flag = true;
                count_suc += 1;
            }
        }
        current_word = current_word_chars.iter().collect(); // 把字符数组变回字符串
        if flag == false {
            guess_times -= 1;
            println!("Sorry, that letter is not in the word");
        }
    }
    if count_suc == secret_word_chars.len()
    {
        println!("Congratulations you guessed the secret word: {}!",secret_word);
    }

       
}
