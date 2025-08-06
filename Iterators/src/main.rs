#![allow(unused)]
#![allow(dead_code)]
struct Employee {
    name: String,
    salary: u32,
}
struct EmployeeRecords {
    employee_db: Vec<Employee>,
}
impl Iterator for EmployeeRecords {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.employee_db.is_empty() {
            None
        } else {
            let result = self.employee_db[0].name.clone();
            self.employee_db.remove(0);
            Some(result)
        }
    }
}
fn main() {
    println!("=====Iterators in Rust=====");
    let mut emp_1 = Employee {
        name: String::from("Alice"),
        salary: 50_000,
    };
    let mut emp_2 = Employee {
        name: String::from("Bob"),
        salary: 60_000,
    };
    let mut emp_3 = Employee {
        name: String::from("Charlie"),
        salary: 70_000,
    };
    let mut emp_db = EmployeeRecords {
        employee_db: vec![emp_1, emp_2, emp_3],
    };
    for employee in emp_db {
        println!("Employee Name: {}", employee);
    }
}
