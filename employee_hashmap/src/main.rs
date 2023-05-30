use std::io;
use std::collections::HashMap;

#[derive(Debug)]
struct Employee {
    id: u32,
    name: String,
}

fn main() {
    let mut emp_count: u32 = 0;
    
    let mut employees: HashMap<String, Vec<Employee>> = HashMap::new();

    add_employee("Eng".to_string(), "Ananya".to_string(), &mut emp_count, &mut employees);
    add_employee("Eng".to_string(), "Nishanth".to_string(), &mut emp_count, &mut employees);
    add_employee("Dsg".to_string(), "PRaks".to_string(), &mut emp_count, &mut employees);
    
    loop {
        println!("\nAdd employee: (1)\nQuit: (other)");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        if input == String::from("1") {
            let mut dept = String::new();
            let mut name = String::new();

            println!("Enter department: ");
            io::stdin()
                .read_line(&mut dept)
                .expect("failed to read line");

            println!("Enter name: ");
            io::stdin()
                .read_line(&mut name)
                .expect("failed to read line");

            add_employee(dept, name, &mut emp_count, &mut employees);
        }
    }

    println!("total employees: {emp_count}");
    
    for (dept, emp_vec) in &employees {
        println!("\nDepartment: {}", dept);
        for emp in emp_vec {
            println!("{:?}", emp);
        }
    }
}

fn add_employee(dept: String, emp_name: String, emp_count: &mut u32, employees: &mut HashMap<String, Vec<Employee>>) {
    *emp_count+=1;
    
    let emp_id = emp_count.clone();
    
    let emp = Employee {
        id: emp_id,
        name: emp_name,
    };
    
    employees
        .entry(dept)
        .or_insert(Vec::new())
        .push(emp);
}
