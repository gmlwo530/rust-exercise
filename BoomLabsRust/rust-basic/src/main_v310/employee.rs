use crate::printinfo::PrintInfo;

#[derive(Clone)]
pub struct Employee {
    name: String,
    id: u64,
}

#[derive(Debug, Clone)]
pub struct EmployeeRecords {
    idx: usize,
    employees: Vec<Employee>,
}

impl Employee {
    pub fn new(name: String, id: u64) -> Employee {
        Employee { name, id }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> u64 {
        self.id
    }
}

impl EmployeeRecords {
    pub fn new() -> Self {
        Self {
            idx: 0,
            employee: Vec::new(),
        }
    }

    pub fn push(&mut self, employee: Employee) {
        self.employee.push(employee);
    }

    pub fn get(&self, id: u64) -> Option<&Employee> {
        self.employee.iter().find(|e| e.id == id)
    }
}

impl Iterator for EmployeeRecords {
    type Item = Employee;

    fn next(&mut self) -> Option<Self::Item> {
        if self.employees.len() > self.idx {
            let output = self.employees[self.idx].clone();
            self.idx += 1;
            return Some(output);
        } else {
            return None;
        }
    }
}

impl PrintInfo for Employee {
    fn print_info(&self) {
        println!(
            "employee's name: {}\nemployee's id: {}\n",
            self.name(),
            self.id()
        );
    }
}
