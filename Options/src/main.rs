#![allow(unused_variables)]
struct Student {
    name: String,
    grade: Option<u32>,
}
fn get_grade(student_name: &String, student_db: &Vec<Student>) -> Option<u32> {
    for student in student_db {
        if student.name == *student_name {
            return student.grade;
        }
    }
    None
}

fn check_student(student_name: &String, student_db: &Vec<Student>) -> Result<(), String> {
    for student in student_db {
        if student.name == *student_name {
            return Ok(());
        }
    }
    Err(format!("Student {} not found", student_name))
}
fn check_student_get_grade(
    student_name: &String,
    student_db: &Vec<Student>,
) -> Result<Option<u32>, String> {
    for student in student_db {
        if student.name == *student_name {
            return Ok(student.grade);
        }
    }
    Err(format!("Student {} not found", student_name))
}

fn main() {
    println!("=====Options in Rust=====");
    let student_db: Vec<Student> = vec![
        Student {
            name: String::from("Alice"),
            grade: Some(95),
        },
        Student {
            name: String::from("Bob"),
            grade: Some(87),
        },
        Student {
            name: String::from("Charlie"),
            grade: None,
        },
    ];
    let student_name = String::from("Alice");
    let student_grade = get_grade(&student_name, &student_db);
    match get_grade(&student_name, &student_db) {
        Some(grade) => println!("{}'s grade is {}", student_name, grade),
        None => println!("No grade found for {}", student_name),
    }
    match check_student(&student_name, &student_db) {
        Ok(_) => {
            let student_grade = get_grade(&student_name, &student_db);
            if let Some(grade) = student_grade {
                println!("{}'s grade is {}", student_name, grade);
            }
        }
        Err(e) => println!("{}", e),
    }
    let student_name = String::from("Charliee");
    match check_student_get_grade(&student_name, &student_db) {
        Ok(grade) => match grade {
            Some(g) => println!("{}'s grade is {}", student_name, g),
            None => println!("{} has no grade", student_name),
        },
        Err(e) => println!("{}", e),
    }
}
