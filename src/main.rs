use std::{array, collections::hash_map};
//include command line i/o
#[allow(non_snake_case)]

use std::{char, collections::HashMap, env::consts, io::{self, Read}};
use ordered_float::OrderedFloat;


fn assigmentOptions() {
    let mut assigmentInput: String = String::new();
    println!("Enter what assigment you want to input in. Q = quiz, L = Lab, W = Worksheet, R = Reading, E = Exam, P = Lab Practicums\n");
    io::stdin().read_line(&mut assigmentInput).expect("please enter a valid character!");
    let mut assigmentInput: char = assigmentInput.chars().next().unwrap();
    match assigmentInput {
        'Q' => {quizInfo();}
        'L'=>println!("urmom"),
        _=>println!("dude")
    };
}
fn quizInfo() {
    let mut quizAssigmentsGrades: HashMap<String, OrderedFloat<f64>> = HashMap::new();
    

    let QC: f32 = 0.2; //constant assoicated with quizzes
    println!("The Current Quiz Grades:");
    quizAssigmentsGrades.insert(String::from("Quiz 1"), OrderedFloat(0.97));
    for (value, key,) in &quizAssigmentsGrades {
        // key is of type OrderedFloat<f64>, so we dereference it to get the inner f64 value
        println!("\t{}: Grade = {:.2}%", value, key.0 * 100.0);
    }

}
fn proccessInput(c:&mut[String], info:&mut String) {
    let mut userInput: String = String::new();
    io::stdin().read_line(&mut userInput).expect("Please enter a valid character");
    for (i ) in c.iter().enumerate() {
        if (c.contains(&userInput)) {
            println!("")
        }
    }
}
fn main() {
    let mut classes: HashMap<String, OrderedFloat<f64>> = HashMap::new();
    classes.insert(String::from("EGR-112"), OrderedFloat(0.0));
    classes.insert(String::from("EGR-112-02"), OrderedFloat(0.980));

    let mut gradeState: String = String::new();
    println!("Welcome!\n\n Your current classes and gades are\n");
    let mut i: i16 = 1;
    for (value, key,) in &classes {
        println!("{}\t{}: grade is: {:.2}%", i, value, key.0 * 100.0);
        i = i + 1;
    }
    println!("\n\nDo you want to create a new class, enter 'C'. Want to edit a existing class, enter 'E'");
    io::stdin().read_line(&mut userInput).expect("please enter valid character");
    if (userInput.chars().next().unwrap() == 'C') {

    }
     //in c we would do %.2lf, in rust we do {:.2} -> the 2 indicating we want 2 0s behind the decimal
    println!("\n if this grade is correct then enter in 'C', if its different enter 'D'");
    io::stdin().read_line(&mut gradeState).expect("");
    //println!("{}", gradeState.chars().next().unwrap());
    if (gradeState.chars().next().unwrap() == 'C') {
        gradeState.clear(); //make this program as safe as possible.
        println!("Do you want to add an assigment, enter  'E'. If you want to view gradebook enter 'V'. If you want to leave enter 'L'");
        io::stdin().read_line(&mut userInput).expect("enter in value");
        if (userInput.chars().next().unwrap() == 'E') {
            println!("What assigment do you want to add?\n\n");
            assigmentOptions();
        }
        // } else if ()  {
    
    } else {
        println!("Nothing was detected, please try entering in a valid option, shutting down...");
    }
}
