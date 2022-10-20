use std::io;
use std::collections::HashMap;

fn main() {
    let mut apartments: HashMap<String, usize> = HashMap::new();
    let mut employees: Vec<Vec<String>> = Vec::new();
    let mut capacity: usize = 0;
    loop {
        println!("1 Add employee to an apartment");
        println!("2 Get employees from specific apartment");
        println!("3 Get all the employees");
        println!("4 Exit");
        let mut select_num = String::new();
        io::stdin()
            .read_line(&mut select_num)
            .expect("Failed to read your choice!");
        let select_num: u32 = match select_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            },
        };
        match select_num {
            1 => {
                println!("Please enter the employee's name:");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read the name!");
                if name.ends_with('\n') {
                    name.pop();
                }
                println!("Please enter the apartment:");
                let mut apartment = String::new();
                io::stdin()
                    .read_line(&mut apartment)
                    .expect("Failed to read the apartment!");
                if apartment.ends_with('\n') {
                    apartment.pop();
                }
                if !apartments.contains_key(&apartment) {
                    apartments.insert(apartment.clone(), capacity);
                    capacity += 1;
                    employees.push(Vec::new());
                }
                let index = if let Some(x) = apartments.get(&apartment) {
                    *x
                } else {
                    0 as usize
                };
                employees[index].push(name);
                println!("Complete!");
            },
            2 => {
                println!("Please enter the apartment name!");
                let mut apartment = String::new();
                io::stdin()
                    .read_line(&mut apartment)
                    .expect("Failed to read the apartment!");
                if apartment.ends_with('\n') {
                    apartment.pop();
                }
                if !apartments.contains_key(&apartment) {
                    println!("Apartment {} doesn't exist!", &apartment);
                    continue;
                }
                let index = if let Some(x) = apartments.get(&apartment) {
                    *x
                } else {
                    0 as usize
                };
                employees[index].sort();
                println!("Apartment {} has {} employees:", &apartment, employees[index].len());
                for name in &employees[index] {
                    println!("{}", name);
                }
            },
            3 => {
                let mut total: Vec<String> = Vec::new();
                for i in 0..capacity {
                    total.append(&mut employees[i]);
                }
                total.sort();
                println!("There are total {} employees in company:", total.len());
                for name in &total {
                    println!("{}", name);
                }
            },
            4 => break,
            _ => {
                println!("Please enter a number within [1, 4]!");
                continue;
            },
        }
    }
}
