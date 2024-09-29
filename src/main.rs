use std::{array, collections::{}, ffi::CString, ops::{Deref, Mul}, os::windows::process, ptr::null, sync::WaitTimeoutResult, thread::sleep, time::Duration};
//include command line i/o

use std::{char, collections::HashMap, env::consts, io::{self, Read}};
use ordered_float::OrderedFloat;
static mut CLASS_SUB_TOPICS:Option<HashMap<String, Vec<String>>> = None;
static mut CLASS_SUB_TOPIC_CONSTANTS: Option<HashMap<String, String>> = None;

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
fn createAssigment(x:bool) {
    unsafe  {
        let sub_topics = CLASS_SUB_TOPICS.as_mut().unwrap();
        let sub_topic_constants = CLASS_SUB_TOPIC_CONSTANTS.as_mut().unwrap();
        if (x == true) { //we will set true case to

        }
    }
}
fn createClass() {
    loop {
        let mut class_name = String::new();
        println!("What is the name of the Class?");
        io::stdin().read_line(&mut class_name).expect("Failed to read line");
        let class_name = class_name.trim().to_string();

        unsafe {
            // Ensure the global variables are initialized
            let sub_topics = CLASS_SUB_TOPICS.as_mut().unwrap();
            let sub_topic_constants = CLASS_SUB_TOPIC_CONSTANTS.as_mut().unwrap();

            loop {
                // Display current subtopics
                if let Some(subtopics) = sub_topics.get(&class_name) {
                    println!("Current Sub Topics: {}", subtopics.join(", "));
                } else {
                    println!("Current Sub Topics: None");
                }

                println!("\nWhat class subtopics do you want to include (Quiz, Homework, Exam, etc)?");

                let mut user_input = String::new();
                io::stdin().read_line(&mut user_input).expect("Failed to read line");
                let user_input = user_input.trim().to_string();

                if user_input.chars().next().unwrap_or(' ') != 'G' {
                    sub_topics.entry(class_name.clone())
                        .or_insert_with(Vec::new)
                        .push(user_input.clone());

                    sub_topic_constants.insert(user_input.clone(), String::from("")); // Modify if needed
                    println!("Sub topic created in class {} called {}", &class_name, user_input);
                    println!("If you want to leave the subtopic naming enter 'G'\n\n"); // 
                } else {
                    println!("Closing subtopic naming editor");
                    break;
                }
            }

            // Constants Input Section
            println!("Enter in the constants for each subtopic (.2 = Quiz)");
            if let Some(subtopics) = sub_topics.get(&class_name) {
                for st in subtopics {
                    println!("Enter constant for {}:", st);
                    let mut constant_input = String::new();
                    io::stdin().read_line(&mut constant_input).expect("Failed to read line");
                    let constant_value = constant_input.trim_end().to_string();
                    sub_topic_constants.insert(st.clone(), constant_value);
                    println!("Constant for {} set to {}", st, sub_topic_constants[st]);
                }
                break;
            } else {
                println!("No subtopics found for class {}", class_name);
            }
        }
    }
}
fn editClass(input: &String) { //in rust we only borrow this value
    println!("Editting class: {} ", input);
    

}

fn main() {
    while (true) {
        unsafe {
            CLASS_SUB_TOPICS = Some(HashMap::new());
            CLASS_SUB_TOPIC_CONSTANTS = Some(HashMap::new());
        }
        
        let mut classes: HashMap<String, OrderedFloat<f64>> = HashMap::new(); //
        let mut classSubTopics: HashMap<String, String> = HashMap::new(); //
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
            createClass();
        } else if (proccesUserInput == 'V') {
                io::stdin().read_line(&mut userInputString).expect("msg");
                let mut i = 1;
                for (class_name, _grade) in &classes {
                    // Compare user input with the index as a string
                    if i.to_string() == userInputString.to_string().as_str().trim() { //condition is working, the solution is that the userinputstring is absorbing a space from the line break so we should call the trim function to deprecate the space
                        println!("Viewing {} gradebook", class_name);
                        println!("\tClass cummalitve grade: {: }%", _grade.0 * 100.0);
                        // println!("\tClass subtopic grades\n Quiz: {}"); // add this line eventually to the main code
                        break;
                    } 
                    i += 1;
                }

        } else if (proccesUserInput == 'E') {
            io::stdin().read_line(&mut userInputString).expect("msg");
                let mut i = 1;
                for (class_name, _) in &classes {
                    // Compare user input with the index as a string
                    if i.to_string() == userInputString.to_string().as_str().trim() { //condition is working, the solution is that the userinputstring is absorbing a space from the line break so we should call the trim function to deprecate the space
                        // println!("\tClass subtopic grades\n Quiz: {}"); // add this line eventually to the main code
                        editClass(&class_name);
                        break;
                    } 
                    i += 1;
                }
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
     } // }
}
