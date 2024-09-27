use std::{array, collections::{hash_map, VecDeque}, ops::Deref, os::windows::process, sync::WaitTimeoutResult, thread::sleep, time::Duration};
//include command line i/o

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
fn proccessInput(c:&[char], info:Vec<&str>) -> char {
    let mut userInput: String = String::new(); //create a string for the user input
    io::stdin().read_line(&mut userInput).expect("Please enter a valid character"); //scan the user input
    let mut userInputChar: char = userInput.chars().next().unwrap(); //unwrap it to expose the first character
    for (i, ch) in c.iter().enumerate() { // rust is gigapenis so it can take an input, whats after the in and output 2 associated datatypes, i-index and ch-char from the list
        if *ch == userInputChar { // Check if the current character matches the user input character
            if let Some(value) = info.get(i) { // some value is at the index i, let it take its data and represnt whats at i. memory safe
                println!("{}", value);  // Print the corresponding element from info
            }
        }
    }
    return userInputChar;
}

fn createClass(mut c:&str) {
    let mut classSubTopics: HashMap<String, String> = HashMap::new();
    
    while (true){
        // println!("If you want to exit the subtopic creater enter 'E'");
        // println!("Class name is {} and current subtopics are:", &c);
        let mut userScannedIn: String = String::new();
        for (i, st) in &classSubTopics {
            println!("\t{}", st);
        }
        println!("\n\n What class subtopics you want to include(Quiz, homework, Exam, etc)");
        io::stdin().read_line(&mut userScannedIn).expect("Closing...");
        c = c.trim_end();
 
        if (userScannedIn.chars().next().unwrap() != 'G') {
            classSubTopics.insert(String::from(c), String::from(userScannedIn.clone()));
            println!("Sub topic created in class {} called {}", &c, &userScannedIn); 
            println!("If you want to leave end subtopic naming enter 'G'")
        } else {
            println!("Closing subtopic editor");
            break;
        }
    }
}
fn main() {
    let mut classes: HashMap<String, OrderedFloat<f64>> = HashMap::new();
    let mut classSubTopics: HashMap<String, String> = HashMap::new();
    let mut proccesUserInput:char;
    let mut userInputString:String = String::new();
    classes.insert(String::from("EGR-112"), OrderedFloat(0.0));
    classes.insert(String::from("EGR-112-02"), OrderedFloat(0.980));

    let mut gradeState: String = String::new();
    println!("Welcome!\n\n Your current classes and gades are\n");
    let mut i: i16 = 1;
    for (value, key,) in &classes {
        println!("{}\t{}: grade is: {:.2}%", i, value, key.0 * 100.0);
        i = i + 1;
    }
    println!("\n\nDo you want to create a new class, enter 'C'. Want to edit a existing class, enter 'E'. Or view a class, enter 'V'");
    proccesUserInput = proccessInput(&['C', 'E', 'V'], vec!["What is the name of the class you want to create?: ", "Select what class you want to edit by entering 1,2,...", "Select what class you want to view by entering 1,2,..."]); 
    if (proccesUserInput == 'C') {
        io::stdin().read_line(&mut userInputString).expect("Failed to read line");
        createClass(&userInputString);
    }
    //     println!("Do you want to add an assigment, enter  'E'. If you want to view gradebook enter 'V'. If you want to leave enter 'L'");
    //     io::stdin().read_line(&mut userInput).expect("enter in value");
    //     if (userInput.chars().next().unwrap() == 'E') {
    //         println!("What assigment do you want to add?\n\n");
    //         assigmentOptions();
    //     }
    //     // } else if ()  {
    
    // } else {
    //     println!("Nothing was detected, please try entering in a valid option, shutting down...");
    // }
}
