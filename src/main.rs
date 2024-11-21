use core::f64;
//include command line i/o
use std::{char, collections::HashMap, env::consts, hash::Hash, io::{self, Read}};
use std::ffi::CString;
use std::hash::Hasher;
use std::io::Write;
use std::path::Component::ParentDir;
use ordered_float::OrderedFloat;
mod dataManager;
use dataManager::{save_data,load_data,Data};
use webbrowser;
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

fn class_name_to_key(class_name: &String) -> i64 {

    let key: i64 = class_name.chars().map(|c| c as i64).sum();
    key
}

fn createClass(
    sub_topics: &mut HashMap<i64, Vec<(String, String, String, f64)>>,
    sub_topic_constants: &mut HashMap<i64, Vec<(String, f64)>>,
    classes: &mut HashMap<String, OrderedFloat<f64>>,  // Add this to track classes

) {
    loop {

        let mut class_name = String::new();
        print!("What is the name of the Class? ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut class_name).expect("Failed to read line");
        let class_name = class_name.trim().to_string();
        if (class_name == String::from("Waltauh")) {
            webbrowser::open("https://www.youtube.com/watch?v=L3duHdRXX04&ab_channel=JohnRonalds");

        }
        // Add the class to the classes HashMap
        classes.insert(class_name.clone(), OrderedFloat(0.0)); // Initialize with 0 grade

        let class_key = class_name_to_key(&class_name);  // Ensure this function is defined
        // println!("Class '{}' created with key: {}", class_name, class_key);

        loop {
            if let Some(subtopics) = sub_topics.get(&class_key) {
                let subtopic_names: Vec<String> = subtopics.iter().map(|(_, name, _, _)| name.clone()).collect();
                println!("Current Sub Topics: {}", subtopic_names.join(", "));
            } else {
                println!("Current Sub Topics: None");
            }

            println!("What class subtopics do you want to include (Quiz, Homework, Exam, etc)? (Enter 'G' to finish)");
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
                    .push((class_name.clone(), user_input.to_string(), String::from("NULL"),1.0));
                println!("Sub topic created in class {} called {}", &class_name, user_input);
            } else {
                println!("Closing subtopic naming editor.");
                break;
            }

        }
        println!("Enter the constants for each subtopic (e.g., 0.2 = Quiz, etc.):");
        if let Some(subtopics) = sub_topics.get(&class_key) {
            for (_,st, _, _) in subtopics {
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
        break; // Exit the outer loop to finish class creation
    }
}
fn createAssigment(name: &String,sub_topic:&String,class_name:&String) -> f64 { //might need to add this later : , classes: &HashMap<String, OrderedFloat<f64>>, mut sub_topics: HashMap<i64, Vec<(String, String, String)>>, sub_topics_constants: &HashMap<String, String>
    let mut user_input = String::new();
    println!("Creating assigment named: {} in sub topic {}", name, sub_topic);
    println!("\nDo you want to enter in the %('P') or the points('T')");
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let mut user_input = user_input.trim().to_string();
    let percent_scored:f64;
    if (user_input == String::from("P")) {
        println!("\nwhat percent did you get on the assigment?( Ex.) 98%, input 98 ):");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        percent_scored = user_input.trim().parse::<f64>().unwrap() / 100.0;
        println!("On assigment {}, you scored a {:.2}%", name, (percent_scored * 100.0).round());
        println!("\n\n\n\n\t Saving assigment {}, into sub-topic {}, into class {} ....",name,sub_topic,class_name);

    }
    else {
        println!("\nhow many points was this assigment out of?");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let total_points:f64 = user_input.trim().parse::<f64>().unwrap();
        println!("The assigment {} is out of {} points. How many points did you score?", name, user_input.trim().to_string());
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let scored_points:f64 = user_input.trim().parse::<f64>().unwrap();
        percent_scored = scored_points / total_points;
        println!("On the assigment {}, you scored a {:.2}%", name, percent_scored * 100.0);
        println!("\n\n\n\n\t Saving assigment {}, into sub-topic {}, into class {} ....",name,sub_topic,class_name);
        //add assigment saving...
    }
    return percent_scored;
}
fn editClass(
    input: &String,
    classes: &HashMap<String, OrderedFloat<f64>>,
    sub_topics: &mut HashMap<i64, Vec<(String, String, String, f64)>>,
    sub_topics_constants: &HashMap<i64, Vec<(String, f64)>>
) {
    let mut userInput = String::new();
    let mut subTopicList: Vec<String> = Vec::new();
    println!("Editing class: {}", input);
    println!("Current class subtopics:");

    // Collect subtopics for the specified class
    for (_id, topics_vec) in sub_topics.iter() {
        for (class, sub_topic, _, _) in topics_vec {
            if class == input && !subTopicList.contains(sub_topic) {
                subTopicList.push(sub_topic.clone());
            }
        }
    }

    if subTopicList.is_empty() {
        println!("No subtopics found for class '{}'.", input);
        return;
    }

    for (index, str) in subTopicList.iter().enumerate() {
        println!("{}.) {}", index + 1, str);
    }

    println!("\nPlease enter the number of the subtopic you want to edit an assignment in:");
    io::stdin().read_line(&mut userInput).expect("Failed to read line");
    let user_input = userInput.trim().to_string();

    if let Ok(selected_index) = user_input.parse::<usize>() {
        if selected_index > 0 && selected_index <= subTopicList.len() {
            let selected_subtopic = &subTopicList[selected_index - 1];
            println!("The subtopic you selected is '{}'.", selected_subtopic);
            println!("If you want to edit an assignment, enter 'E'. If you want to create an assignment, enter 'C'.");

            let mut action_input = String::new();
            io::stdin().read_line(&mut action_input).expect("Failed to read line");
            let action_input = action_input.trim();

            if action_input == "C" {
                println!("What was the name of the assignment?");
                let mut assignment_name = String::new();
                io::stdin().read_line(&mut assignment_name).expect("Failed to read line");
                let assignment_name = assignment_name.trim().to_string();

                // Ensure `createAssignment` is defined properly
                let score: f64 = createAssigment(&assignment_name, selected_subtopic, input);
                let size = sub_topics.len() as i64;
                sub_topics.insert(size, vec![(input.clone(), selected_subtopic.clone(), assignment_name, score)]);
            } else if action_input == "E" {
                let mut assignments_to_edit = vec![];
                let mut i = 1;
                println!("Here are all the current assignments under this subtopic:");

                for (id, assignments) in sub_topics.iter() {
                    for (class, sub_topic, assign, grade) in assignments {
                        if sub_topic == selected_subtopic {
                            if (assign == &String::from("NULL")) {
                                continue;
                            }
                            println!("\t{}.) {} - Grade: {}%", i, assign, grade * 100.0);
                            assignments_to_edit.push((*id, assign.clone()));
                            i += 1;
                        }
                    }
                }

                println!("Please enter the number of the assignment you want to edit:");
                userInput.clear();
                io::stdin().read_line(&mut userInput).expect("Failed to read line");
                if let Ok(choice) = userInput.trim().parse::<usize>() {
                    if choice > 0 && choice <= assignments_to_edit.len() {
                        let (assignment_id, assignment_name) = assignments_to_edit[choice - 1].clone();
                        println!("You selected '{}'. Do you want to remove the grade ('R') or edit the grade ('E')?", assignment_name);

                        userInput.clear();
                        io::stdin().read_line(&mut userInput).expect("Failed to read line");

                        if userInput.trim() == "E" {
                            println!("What percent did you score on the assignment? (Ex. 98)");
                            userInput.clear();
                            io::stdin().read_line(&mut userInput).expect("Failed to read line");
                            if let Ok(new_grade) = userInput.trim().parse::<f64>() {
                                sub_topics.insert(assignment_id, vec![(input.clone(), selected_subtopic.clone(), assignment_name, new_grade / 100.0)]);
                            } else {
                                println!("Invalid grade input.");
                            }
                        } else if userInput.trim() == "R" {
                            sub_topics.remove(&assignment_id);
                            println!("Assignment removed.");
                        } else {
                            println!("Invalid option.");
                        }
                    } else {
                        println!("Invalid assignment selection.");
                    }
                } else {
                    println!("Invalid input.");
                }
            } else {
                println!("Invalid option. Please enter 'E' to edit or 'C' to create an assignment.");
            }
        } else {
            println!("Invalid selection. Please try again.");
        }
    } else {
        println!("Invalid input. Please enter a number.");
    }
}

fn viewClass(
    user_input_string: &String,
    classes: &HashMap<String, OrderedFloat<f64>>,
    sub_topics: &HashMap<i64, Vec<(String, String, String, f64)>>,
    sub_topic_constants: &HashMap<i64, Vec<(String, f64)>>,
    sub_topic_grades: &mut HashMap<i64, Vec<(String, OrderedFloat<f64>)>>,
) {
    let mut user_input = String::new();
    let mut sub_topic_list: Vec<(String, Vec<(String, f64)>)> = Vec::new(); // Store subtopic and its assignments with grades
    let class_key = class_name_to_key(&user_input_string);

    // Check if the class exists
    if !classes.contains_key(user_input_string) {
        println!("Class '{}' does not exist.", user_input_string);
        return;
    }

    // Display the class grade (already calculated by `calculateGrade`)
    if let Some(final_grade) = classes.get(user_input_string) {
        println!("Class '{}' - Final Grade: {:.2}%", user_input_string, final_grade * 100.0);
    } else {
        println!("No grade found for class '{}'", user_input_string);
    }

    // Calculate subtopic grades and accumulate class grades
    for (_id, topics_vec) in sub_topics.iter() {
        for (class_name, sub_topic, assignment_name, grade) in topics_vec {
            if (assignment_name == &String::from("NULL")) {
                continue;
            }
            if class_name == user_input_string {
                // Add assignment to subtopic
                let entry = sub_topic_list.iter_mut().find(|(topic, _)| topic == sub_topic);
                if let Some((_, assignments)) = entry {
                    assignments.push((assignment_name.clone(), *grade)); // Add the assignment and grade
                } else {
                    sub_topic_list.push((sub_topic.clone(), vec![(assignment_name.clone(), *grade)])); // Initialize with the first assignment
                }
            }
        }
    }

    // Print subtopic list with average grades and assignments
    println!("\nCurrent class subtopics:");
    for (index, (subtopic, assignments)) in sub_topic_list.iter().enumerate() {
        let total_subtopic_score: f64 = assignments.iter().map(|(_, grade)| *grade).sum();
        let subtopic_average = total_subtopic_score / assignments.len() as f64;

        // Display subtopic grade
        println!("{}. {} - Subtopic Grade: {:.2}%", index + 1, subtopic, subtopic_average * 100.0);

        // Print assignments for the selected subtopic
        for (assignment_name, grade) in assignments {
            if assignment_name != "NULL" {
                println!("\tAssignment: {}, Grade: {:.2}%", assignment_name, grade * 100.0);
            }
        }
    }

    // Prompt for subtopic selection
    println!("\nPlease select a subtopic number to view assignments or enter 'X' to exit:");
    io::stdin().read_line(&mut user_input).expect("Failed to read input");
    let user_input = user_input.trim();

    if user_input.eq_ignore_ascii_case("X") {
        return; // Exit the function if 'X' is entered
    }

    if let Ok(selected_index) = user_input.parse::<usize>() {
        if selected_index > 0 && selected_index <= sub_topic_list.len() {
            let selected_subtopic = &sub_topic_list[selected_index - 1].0;
            println!("\nAssignments under the subtopic '{}':", selected_subtopic);

            // Print assignments for the selected subtopic
            if let Some(assignments) = sub_topic_list.iter().find(|(topic, _)| topic == selected_subtopic) {
                for (assignment_name, grade) in &assignments.1 {
                    if assignment_name != "NULL" {
                        println!("Assignment: {}, Grade: {:.2}%", assignment_name, grade * 100.0);
                    }
                }
            }
        } else {
            println!("Invalid selection. Please try again.");
        }
    } else {
        println!("Invalid input. Please enter a number or 'X' to exit.");
    }
}
fn calculateGrade(
    classes: &mut HashMap<String, OrderedFloat<f64>>, // Class grades (mutable to update with final grade)
    sub_topics: &HashMap<i64, Vec<(String, String, String, f64)>>, // Subtopics with assignments
    sub_topic_constants: &HashMap<i64, Vec<(String, f64)>>, // Constants for each subtopic
    sub_topic_grades: &mut HashMap<i64, Vec<(String, OrderedFloat<f64>)>> // Calculated subtopic grades
) {
    let mut index: i32 = 1;

    // Iterate over each class and calculate the final grade
    let mut final_grades = HashMap::new(); // Temporary hashmap to store final grades

    for (class_name, _class_grade) in classes.iter_mut() {
        let mut total_weighted_subtopic_grades = 0.0;
        let mut total_weight = 0.0; // This will hold the total sum of subtopic constants
        let mut total_assignments = 0;
        // Iterate over the subtopics and their assignments
        for (_id, topics_vec) in sub_topics.iter() {
            for (stored_class_name, sub_topic, assignment_name, grade) in topics_vec {
                if assignment_name == &String::from("NULL") {
                    continue;
                }
                
                if stored_class_name == class_name {
                    total_assignments += 1;

                    // Find the constant for the subtopic (if any)
                    let subtopic_constant = sub_topic_constants
                        .iter()
                        .find(|(_, subtopics)| subtopics.iter().any(|(topic, _)| topic == sub_topic))
                        .and_then(|(_, subtopics)| subtopics.iter().find(|(topic, _)| topic == sub_topic))
                        .map(|(_, constant)| *constant) // Dereference the constant
                        .unwrap_or(1.0); // Default to 1.0 if no constant is found
                    // Calculate the weighted grade for the assignment
                    let weighted_grade = *grade * subtopic_constant;

                    // Accumulate weighted grades and constants
                    total_weighted_subtopic_grades += weighted_grade;
                    // total_weight += subtopic_constant;
                }
            }
        }

        // Calculate the final class grade as the weighted average of the subtopics
        // let final_class_grade = if total_weight > 0.0 {
        //     total_weighted_subtopic_grades / total_weight
        // } else {
        //     0.0
        // };

        // Store the final grade temporarily in the hashmap
        let mut final_class_grade = total_weighted_subtopic_grades;
        
        final_grades.insert(class_name.clone(), OrderedFloat(final_class_grade));

        // Display the class information and final grade
        println!(
            "\t{}.) Class: {} - Final Grade: {:.2}%",
            index, class_name, total_weighted_subtopic_grades * 100.0
        );
        index += 1;
    }

    // Now, after calculating all grades, update the original classes hashmap
    for (class_name, final_grade) in final_grades {
        classes.insert(class_name, final_grade); // Insert the final grade into the original hashmap
    }
}

fn main() {
    //file loading code!
    let file_path = "data.json";
    // Load pre-existing data or initialize new data
    let mut data = load_data(file_path).unwrap_or_else(|_| Data {
        classes: HashMap::new(),
        sub_topics: HashMap::new(),
        sub_topic_constants: HashMap::new(),
        sub_topic_grades: HashMap::new(),
    });
    let mut proccesUserInput: char;
    let mut userInputString: String = String::new();
    while true {
        userInputString = String::new();
        println!("Welcome!\n\n Your current classes and grades are\n");
        let mut i: i16 = 1;
        calculateGrade(&mut data.classes,& mut data.sub_topics, &mut data.sub_topic_constants, &mut data.sub_topic_grades);
        println!("\n\nDo you want to create a new class, enter 'C'. Want to edit an existing class, enter 'E'. Or view a class, enter 'V'. If you want to leave the program enter 'L'");
        proccesUserInput = proccessInput(&['C', 'E', 'V'], vec!["What is the name of the class you want to create?: ", "Select what class you want to edit by entering 1,2,...", "Select what class you want to view by entering 1,2,..."]);

        if proccesUserInput == 'C' {
            createClass(&mut data.sub_topics, &mut data.sub_topic_constants, &mut data.classes);
        } else if proccesUserInput == 'V' {
            io::stdin().read_line(&mut userInputString).expect("Failed to read input");
            let mut i = 1;
            for (class_name, _) in &data.classes {
                if i.to_string() == userInputString.trim() {
                    viewClass(&class_name, &data.classes, &data.sub_topics.clone(), &data.sub_topic_constants.clone(), &mut data.sub_topic_grades.clone());
                    break;
                }
                i += 1;
            }
        } else if proccesUserInput == 'E' {
            io::stdin().read_line(&mut userInputString).expect("Failed to read input");
            let mut i = 1;
            for (class_name, _) in &data.classes {
                if i.to_string() == userInputString.trim() {
                    editClass(&class_name, &data.classes, &mut data.sub_topics, &data.sub_topic_constants);
                    break;
                }
                i += 1;
            }
        } else if proccesUserInput == 'L' {
            println!("Have a good day!!");
            save_data(file_path, &data).expect("Error saving data");
            break;
        }
    }

}
