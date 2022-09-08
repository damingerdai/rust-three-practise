use std::{io, collections::HashMap};

fn main() {
   let mut company: HashMap<String, Vec<String>> = HashMap::new();

   loop {
    println!("输入所在部门");
    let mut department = String::new();
    io::stdin().read_line(&mut department).expect("department error");
    println!("输入所在员工名字");
    let mut employee = String::new();
    io::stdin().read_line(&mut employee).expect("employee error");
    let employees = company.entry(department.trim_end_matches('\n').to_string()).or_insert(Vec::new());
    employees.push(employee.trim_end_matches('\n').to_string());
 
    println!("{:?}", company);
   }
  
}
