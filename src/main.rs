mod project;
mod validator;
mod validator_test;

use std::env;

use toml_edit::{Document, Value};

use crate::project::Project;
use crate::project::Validator;

fn main() {
    let toml = r#"
[settings]
name = "Example"
path = "./example"


[enum.Color]
options = [
  "BLUE",
  "GREEN",
  "YELLOW"
]

[class]
test = "print me"
parent = "Animal"
color = "Color"
breed = "string"

[class.Dog]
parent = "Animal"
[class.Dog.private]
color = "Color"
breed = "string"
[class.Dog.public.method.getColor]
return = "Color"
[class.Dog.public.method.setColor]
params = { color = "Color" }


[class.Animal.private]
age = "integer"


[function.main]
calls = [
  "Dog",
  "Dog.setColor",
  "Dog.getColor"
]"#;
    // for arg in env::args() {
    //     println!("{}", arg);
    // }
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
