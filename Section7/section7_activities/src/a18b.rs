// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum EmployeeType {
    MaintenanceCrew,
    MarketingDepartmentEmployee,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnician,
}

struct isEmployed {
    employee_type: EmployeeType,
    is_employed: bool,
}

fn mayEnter(employee: &isEmployed) -> Result<(), String> {
    
    match employee.is_employed {
        // early return when error
        false => return Err("Not employed".to_owned()),
        true => (),
    }

    match employee.employee_type {
        EmployeeType::MaintenanceCrew => Ok(()),
        EmployeeType::MarketingDepartmentEmployee => Ok(()),
        EmployeeType::Manager => Ok(()),
        EmployeeType::LineSupervisor => Err("Line supervisor has no Access".to_owned()),
        EmployeeType::KitchenStaff => Err("Kitchen staff has no Access".to_owned()),
        EmployeeType::AssemblyTechnician => Err("Assembly technician has no Access".to_owned()),
        _ => Err("No Access".to_owned()),
    }
}

fn checkEmployee(employee: &isEmployed) -> Result<(), String> {

    let access = mayEnter(employee)?;
    println!("Access Granted");
    Ok(())

}

pub fn result_question_mark() {

    let kenny = isEmployed {
        employee_type: EmployeeType::Manager,
        is_employed: true,
    };

    match checkEmployee(&kenny) {
        Err(e) => println!("Access Denied: {:?}", e),
        _ => (),
    }
}
