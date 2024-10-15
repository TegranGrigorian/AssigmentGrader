use core::f64;
//include command line i/o
use std::{char, collections::HashMap, env::consts, hash::Hash, io::{self, Read}};
use std::ffi::CString;
use std::io::Write;
use std::path::Component::ParentDir;
use ordered_float::OrderedFloat;

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

fn class_name_to_key(class_name: &str) -> i64 {
    let mut key: i64 = 0;
    for c in class_name.chars() {
        key += c as i64;
    }
    key
}
fn createClass(mut sub_topics: HashMap<i64, Vec<(String, String, String)>>, mut sub_topic_constants: HashMap<i64, Vec<(String, f64)>>) {
    loop {
        let mut class_name = String::new();
        print!("What is the name of the Class?");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut class_name).expect("Failed to read line");
        let class_name = class_name.trim().to_string();

        let class_key = class_name_to_key(&class_name);  // Ensure this function is defined

        loop {
            if let Some(subtopics) = sub_topics.get(&class_key) {
                let subtopic_names: Vec<String> = subtopics.iter().map(|(name, _, _)| name.clone()).collect();
                println!("Current Sub Topics: {}", subtopic_names.join(", "));
            } else {
                println!("Current Sub Topics: None");
            }

            println!("What class subtopics do you want to include (Quiz, Homework, Exam, etc)?");
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Failed to read line");
            let user_input = user_input.trim();
            if user_input.is_empty() {
                println!("Invalid input. Please try again.");
                continue;
            }

            if user_input.chars().next().unwrap() != 'G' {
                sub_topics.entry(class_key)
                    .or_insert_with(Vec::new)
                    .push((user_input.to_string(), String::new(), String::new()));
                println!("Sub topic created in class {} called {}", &class_name, user_input);
                println!("If you want to leave the subtopic naming session, enter 'G'\n");
            } else {
                println!("Closing subtopic naming editor.");
                break;
            }
        }

        println!("Enter the constants for each subtopic (e.g., 0.2 = Quiz, etc.):");
        if let Some(subtopics) = sub_topics.get(&class_key) {
            for (st, _, _) in subtopics {
                println!("Enter constant for {}:", st);
                let mut constant_input = String::new();
                io::stdin().read_line(&mut constant_input).expect("Failed to read line");
                let constant_input = constant_input.trim();
                match constant_input.parse::<f64>() {
                    Ok(value) => {
                        let constants = sub_topic_constants.entry(class_key).or_insert_with(Vec::new);
                        constants.push((st.clone(), value));
                        println!("Constant for {} set to {}", st, value);
                    }
                    Err(_) => println!("Invalid input. Please enter a valid number."),
                }
            }
        } else {
            println!("No subtopics found for class {}", class_name);
        }
        break;
    }
}

fn editClass(input: &String, classes: &HashMap<String, OrderedFloat<f64>>, mut sub_topics: HashMap<i64, Vec<(String, String, String)>>, sub_topics_constants: &HashMap<i64, Vec<String, f64>>) { //in rust we only borrow this value
    let mut i = 1;    
    let mut userInput = String::new();
    let mut subTopicList: Vec<String> = Vec::new();
    println!("Editting class: {} ", input);
    println!("Current class subtopics:");
    for (_id, topics_vec) in sub_topics.iter() {
        // Now `topics_vec` is a Vec<(String, String, String)>
        for (class, sub_topic, _) in topics_vec {
            // Compare `input` with the `sub_topic` field
            if (!subTopicList.iter().any(|topic| topic == sub_topic)) { //check if sub_topic is in the list
                // println!("{}.) {}", i, sub_topic);
                subTopicList.push(sub_topic.clone());  // Add to the list
                i += 1;
            } else {
                println!("Skipping duplicate: {}", sub_topic);
            }
        }
    }
    for (index,str) in subTopicList.iter().enumerate() {
        println!("{}.) {}", index + 1, str)
    }
    println!("\nPlease enter what subtopic you want to edit an assigment in:");
    io::stdin().read_line(&mut userInput).expect("Failed to read line");
    let mut user_input = userInput.trim().to_string();
    for (index, subtopic) in subTopicList.iter().enumerate() {
        if (user_input == (index + 1).to_string()) {
            println!("The sub topic you selected is {}", subtopic);
            println!("If you want to edit an assigment enter 'E'. If you want to create an assigment enter 'C'");
            let mut user_input = String::new(); //wipe old data from var
            io::stdin().read_line(&mut user_input).expect("Failed to read line");
            let mut user_input = user_input.trim().to_string();
            println!("{}", user_input);
            if (user_input == String::from("C")) {
                println!("What was the name of the assignment?");
                let mut user_input = String::new();
                io::stdin().read_line(&mut user_input).expect("Err");
                let mut user_input = user_input.trim().to_string();
                createAssigment(&user_input, &subtopic, input); //input is the class name
            } else {
                println!("Here are all the current assigments under this class: ");

            }
        }
    }
}
fn createAssigment(name: &String,sub_topic:&String,class_name:&String) { //might need to add this later : , classes: &HashMap<String, OrderedFloat<f64>>, mut sub_topics: HashMap<i64, Vec<(String, String, String)>>, sub_topics_constants: &HashMap<String, String>
    let mut user_input = String::new();
    println!("Creating assigment named: {} in sub topic {}", name, sub_topic);
    println!("\nDo you want to enter in the %('P') or the points('T')");
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let mut user_input = user_input.trim().to_string();
    if (user_input == String::from("P")) {
        println!("\nwhat percent did you get on the assigment?( Ex.) 98%, input 98 ):");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let percent_scored:f32 = user_input.trim().parse::<f32>().unwrap() / 100.0;
        println!("On assigment {}, you scored a {}%", name, String::from((percent_scored * 100.0).to_string()));
        println!("\n\n\n\n\t Saving assigment {}, into sub-topic {}, into class {} ....",name,sub_topic,class_name);

    }
    else {
        println!("\nhow many points was this assigment out of?");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let total_points:f32 = user_input.trim().parse::<f32>().unwrap();
        println!("The assigment {} is out of {} points. How many points did you score?", name, user_input.trim().to_string());
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let scored_points:f32 = user_input.trim().parse::<f32>().unwrap();
        let percent_scored:f32 = scored_points / total_points;
        println!("On the assigment {}, you scored a {}%", name, percent_scored * 100.0);
        println!("\n\n\n\n\t Saving assigment {}, into sub-topic {}, into class {} ....",name,sub_topic,class_name);
        //add assigment saving...
    }

}
fn viewClass(mut user_input_string: String, classes: &HashMap<String, OrderedFloat<f64>>, sub_topics: &HashMap<i64, Vec<(String, String, String)>>) {
    io::stdin().read_line(&mut user_input_string).expect("Failed to read input");
    let user_input_string = user_input_string.trim(); // Remove extra spaces and line breaks
    let mut i = 1;
    for (class_name, grade) in classes {
        // Compare user input with the index as a string
        if i.to_string() == user_input_string {
            println!("Viewing {} gradebook", class_name);
            println!("\tClass cumulative grade: {:.2}%", grade.0 * 100.0); // Two decimal formatting
            println!("\tSubtopic grades:");

            let class_key = class_name_to_key(class_name);

            if let Some(subtopics) = sub_topics.get(&class_key) {
                for subtopic in subtopics {
                    println!("{:?}", subtopic);
                }
            } else {
                println!("\tNo subtopics found for this class.");
            }
            // We break because we found the class we were looking for
            break;
        }
        i += 1;
    }
}
fn main() {
    let mut clHash: HashMap<String, OrderedFloat<f64>> = HashMap::new(); //class name and cummalitive grade are stored in this hashmap
    let mut clSTHas: HashMap<i64, Vec<(String, String, String)>> = HashMap::new(); //index, class name subtopic, assigment name
    let mut clSTCHas: HashMap<i64, Vec<String, f64>> = HashMap::new(); // index, class name and, constant. We find what constant the subtopic goes to by keeping it in a numberical order.
    let mut clSTG: HashMap<String, Vec<String,ordered_float::OrderedFloat<f64>>> = HashMap::new(); // this hashmap contains the class name, class subtopic and grade of subtopic
    clHash.insert(String::from("EGR-112"), OrderedFloat(0.0));
    clHash.insert(String::from("EGR-112-02"), OrderedFloat(0.980));
    clSTHas.insert(0, vec![(String::from("EGR-112") ,String::from("Quiz"), String::from("Quiz 1"))]);
    clSTHas.insert(1, vec![(String::from("EGR-112"), String::from("Quiz"), String::from("Quiz 2"))]);
    clSTHas.insert(2 ,vec![(String::from("EGR-112"), String::from("Homework"), String::from("Homework 1"))]);

    while (true) {
        let mut proccesUserInput:char;
        let mut userInputString:String = String::new();
        let mut gradeState: String = String::new();
        let mut classes: HashMap<String, OrderedFloat<f64>> = clHash.clone();
        let classSubTopics = clSTHas.clone();
        let classSubTopicConstants= clSTCHas.clone();

        println!("Welcome!\n\n Your current classes and grades are\n");
        let mut i: i16 = 1;
        for (value, key,) in &classes {
            println!("{}\t{}: grade is: {:.2}%", i, value, key.0 * 100.0);
            i = i + 1;
        }
        println!("\n\nDo you want to create a new class, enter 'C'. Want to edit a existing class, enter 'E'. Or view a class, enter 'V'. If you want to leave the program enter 'L'");
        proccesUserInput = proccessInput(&['C', 'E', 'V'], vec!["What is the name of the class you want to create?: ", "Select what class you want to edit by entering 1,2,...", "Select what class you want to view by entering 1,2,..."]); 
        if (proccesUserInput == 'C') {
            createClass(classSubTopics, classSubTopicConstants);
        } else if (proccesUserInput == 'V') {
            viewClass(userInputString, &classes, &classSubTopics);
        } else if (proccesUserInput == 'E') {
            io::stdin().read_line(&mut userInputString).expect("msg");
                let mut i = 1;
                for (class_name, _) in &classes {
                    // Compare user input with the index as a string
                    if i.to_string() == userInputString.to_string().as_str().trim() { //condition is working, the solution is that the userinputstring is absorbing a space from the line break so we should call the trim function to deprecate the space
                        // println!("\tClass subtopic grades\n Quiz: {}"); // add this line eventually to the main code
                        editClass(&class_name, &classes, classSubTopics,&classSubTopicConstants);
                        break;
                    } 
                    i += 1;
                }
        } if (proccesUserInput == 'L') {
            println!("Have a good day!!");
            break;
        }
     }
}
