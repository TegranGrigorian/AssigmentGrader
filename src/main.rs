use core::f64;
//include command line i/o
use std::{char, collections::HashMap, env::consts, hash::Hash, io::{self, Read}};
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
fn createAssigment(x:bool,) {
   
}
//'global vars'
// static mut CLASS_SUB_TOPICS:Option<HashMap<String, Vec<String>>> = None;
// static mut CLASS_SUB_TOPIC_CONSTANTS: Option<HashMap<String, String>> = None;

fn createClass(mut sub_topics: HashMap<i64, Vec<(String, String, String)>>, mut sub_topic_constants: HashMap<String, String>) { //sub_topics:HashMap<String,String>, sub_topics_constatns:HashMap<String,String>
    loop {
        let mut class_name = String::new();
        println!("What is the name of the Class?");
        io::stdin().read_line(&mut class_name).expect("Failed to read line");
        let class_name = class_name.trim().to_string();
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
                constant_input.parse::<f32>().unwrap(); // convert str to float datatype
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
fn editClass(input: &String, classes: &HashMap<String, OrderedFloat<f64>>, mut sub_topics: HashMap<i64, Vec<(String, String, String)>>, sub_topics_constants: &HashMap<String, String>) { //in rust we only borrow this value
    let mut i = 1;    
    println!("Editting class: {} ", input);
    println!("Current class subtopics:");
    for (_id, topics_vec) in sub_topics.iter() {
        // Now `topics_vec` is a Vec<(String, String, String)>
        for (class, sub_topic, _) in topics_vec {
            // Compare `input` with the `sub_topic` field
            if input == sub_topic {
            println!("\t{:?}",sub_topic);
        } else {
            //expanded else for debugging if needed
            // println!("erm not the same class, class inputted {} class in list {}", input, class);
        }
        i += 1;
       
        }
    } 
}
fn viewClass(mut userInputString: String, mut classes: &HashMap<String, OrderedFloat<f64>>, mut sub_topics: &HashMap<i64, Vec<(String, String, String)>>) { //we need to borrow these hashmaps and inputs
    io::stdin().read_line(&mut userInputString).expect("msg");
    let mut i = 1;
    for (class_name, _grade) in classes {
        // Compare user input with the index as a string
        if i.to_string() == userInputString.to_string().as_str().trim() { //condition is working, the solution is that the userinputstring is absorbing a space from the line break so we should call the trim function to deprecate the space
            println!("Viewing {} gradebook", class_name);
            println!("\tClass cummalitve grade: {: }%", _grade.0 * 100.0);
            println!("\tSubtopic grades:"); 
            let current_class = class_name;
            for (class_name, subtopic) in sub_topics {
                if (&class_name == &current_class) {
                    println!("{:?}", subtopic);
                }

            }
            // println!("\tClass subtopic grades\n Quiz: {}"); // add this line eventually to the main code
            break;
        } 
        i += 1;
    }
    
}
fn main() {
    while (true) {
        let mut classes: HashMap<String, OrderedFloat<f64>> = HashMap::new(); //
        let mut classSubTopics: HashMap<i64, Vec<(String, String, String)>> = HashMap::new(); //
        let mut classSubTopicConstants: HashMap<String, String> = HashMap::new();
        let mut proccesUserInput:char;
        let mut userInputString:String = String::new();
        classes.insert(String::from("EGR-112"), OrderedFloat(0.0));
        classes.insert(String::from("EGR-112-02"), OrderedFloat(0.980));
        
        let mut gradeState: String = String::new();
        
        classSubTopics.insert(0, vec![(String::from("EGR-112") ,String::from("Quiz"), String::from("Quiz 1"))]);
        classSubTopics.insert(1, vec![(String::from("EGR-112"), String::from("Quiz"), String::from("Quiz 2"))]);
        classSubTopics.insert(2 ,vec![(String::from("EGR-112"), String::from("Homework"), String::from("Homework 1"))]);
        
        println!("Welcome!\n\n Your current classes and grades are\n");
        let mut i: i16 = 1;
        for (value, key,) in &classes {
            println!("{}\t{}: grade is: {:.2}%", i, value, key.0 * 100.0);
            i = i + 1;
        }
        println!("\n\nDo you want to create a new class, enter 'C'. Want to edit a existing class, enter 'E'. Or view a class, enter 'V'");
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
        }

     }
}
