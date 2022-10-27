pub use mall::*;
pub use mall::floor;
pub use mall::floor::*;
pub use mall::floor::store;
pub use mall::floor::store::*;
pub use crate::employee::Employee;
pub use crate::employee::*;
pub use crate::guard::Guard;
pub use crate::guard::*;
pub use crate::store::Store;
pub use crate::store::*;

pub mod mall;

pub fn biggest_store(mall: Mall) -> Store {
    let mut square_meters = 0;
    let mut name = String::new();
    let mut employees: Vec<Employee> = vec![];
    for floor in mall.floors.iter() {
        for store in floor.stores.iter() {
            if store.square_meters > square_meters {
                square_meters = store.square_meters;
                name = store.name.clone();
                employees = store.employees.to_vec();
            }
        }
    }
    Store {
        name,
        square_meters,
        employees,
    }
}

pub fn highest_paid_employee(mall: Mall) -> Vec<Employee> {
    let mut paid = 0.0;
    let mut employees: Vec<Employee> = vec![];
    for floor in mall.floors.iter() {
        for store in floor.stores.clone() {
            for employee in store.employees {
                if employee.salary > paid {
                    paid = employee.salary.clone();
                    employees.clear();
                    employees.push(employee);
                } else if employee.salary == paid {
                    employees.push(employee);
                }
            }
        }
    }
    employees
}

pub fn nbr_of_employees(mall: Mall) -> usize {
    let mut nbr: usize = 0;
    for floor in mall.floors {
        for store in floor.stores {
            nbr += store.employees.len();
        }
    }

    nbr + mall.guards.len()
}

pub fn check_for_securities(mall: &mut Mall, guard: Vec<Guard>) {
    let mut square_meters = 0;
    for floor in mall.floors.iter() {
        for store in floor.stores.iter() {
            if store.square_meters > square_meters {
                square_meters += store.square_meters;
            }
        }
    }
    if square_meters % 200 > 200 {}
    let mut idx = 200;
    let mut scd = 0;
    while idx < square_meters {
        if idx / 200 > mall.guards.len() as u64 {
            mall.guards.push(guard[scd].clone());
            scd += 1;
        }
        idx += 200;
    }
    mall.guards.push(guard[scd + 1].clone());
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.iter() {
        for store in floor.stores.clone() {
            for mut employee in store.employees {
                if employee.working_hours.1 - employee.working_hours.0 >= 10 { employee.salary *= 1.10; } else { employee.salary *= 0.90; }
            }
        }
    }
}