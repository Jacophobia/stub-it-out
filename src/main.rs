mod project;
mod project_class;
mod project_enum;
mod project_function;
mod project_interface;
mod project_method;
mod project_settings;
mod project_struct;
mod project_variable;
mod toml_parser;
mod toml_parser_test;
mod validator;
mod validator_test;

use crate::project::project::Project;

fn main() {
    for arg in std::env::args() {
        println!("{}", arg);
    }
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
