//include command line i/o
use std::{char, io::{self, Read}};
fn assigmentOptions() {
    println!("")
    //printf("What assigment do you want to add?\n\n");
// printf("Enter what assigment you want to input in. Q = quiz, L = Lab, W = Worksheet, R = Reading, E = Exam, P = Lab Practicums\n");
}
fn main() {
    let currentGrade: f32 = 0.0;
    let mut gradeState: String = String::new();
    let mut userInput: String = String::new();
    println!("Welcome to the Grade Calculator, your current grade is: {:.2}", currentGrade); //in c we would do %.2lf, in rust we do {:.2} -> the 2 indicating we want 2 0s behind the decimal
    println!("\n if this grade is correct then enter in 'C', if its different enter 'D'");
    io::stdin().read_line(&mut gradeState).expect("");
    //println!("{}", gradeState.chars().next().unwrap());
    if (gradeState.chars().next().unwrap() == 'C') {
        gradeState.clear(); //make this program as safe as possible.
        println!("Do you want to add an assigment, enter  'E'. If you want to view gradebook enter 'V'. If you want to leave enter 'L'");
        io::stdin().read_line(&mut userInput).expect("enter in value");
        if (userInput.chars().next().unwrap() == 'E') {
            
        }
        // } else if ()  {
    
    } else {
        println!("Nothing was detected, please try entering in a valid option, shutting down...");
    }
}
