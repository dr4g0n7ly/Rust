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
