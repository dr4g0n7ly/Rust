use std::collections::HashMap;

#[derive(Debug)]
struct Employee {
    name: String,
    age: u32,
    department: String,
}

fn main() {
    let mut employees: HashMap<String, Vec<Employee>> = HashMap::new();

    let employee1 =  Employee {
        name: String::from("John"),
        age: 30,
        department: String::from("Sales"),
    };

    let employee2 = Employee {
        name: String::from("Smith"),
        age: 35,
        department: String::from("Marketing")
    };
    
    let employee3 = Employee {
        name: String::from("Mike"),
        age: 32,
        department: String::from("Engineering")
    };

    employees.entry(String::from("Group 1"))
        .or_insert(Vec::new())
        .push(employee1);

    employees.entry(String::from("Group 1"))
        .or_insert(Vec::new())
        .push(employee2);

    employees.entry(String::from("Group 2"))
        .or_insert(Vec::new())
        .push(employee3);


    for (group, employee_vec) in &employees {
        println!("Group: {}", group);
        for emp in employee_vec {
            println!("{:?}", emp);
        }
    }
}
