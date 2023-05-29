#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]

fn main() {
    let emp = Employee {
        name: String::from("John"),
        company: String::from("Acme Inc."),
        age: 27,
    };

    println!("{:?}", emp);
    println!("{}", emp.name);
    println!("{}", emp.fn_details());
    println!("{}", Employee::static_fn());
}

#[derive(Debug)]
struct Employee {
    name: String,
    company: String,
    age: u32,
}

impl Employee {
    fn fn_details(&self) -> String {
        format!(
            "name: {}, age: {}, company: {}",
            &self.name, &self.age, &self.company
        )
    }

    fn static_fn() -> String {
        String::from("Details of a person")
    }
}
