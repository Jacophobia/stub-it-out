use crate::project_class::class::Class;
use crate::project_enum::enumeration::Enumeration;
use crate::project_function::function::Function;
use crate::project_interface::interface::Interface;
use crate::project_method::method::Method;
use crate::project_settings::settings::Settings;
use crate::project_struct::structure::Struct;
use crate::project_variable::variable::Variable;
use crate::Project;

use std::collections::HashSet;

pub struct Validator {
    require_descriptions: bool,
    data_types: HashSet<String>,
    used_data_types: Vec<String>,
}

impl Validator {
    pub fn new() -> Validator {
        Validator {
            require_descriptions: true,
            data_types: HashSet::new(),
            used_data_types: vec![
                String::from("function"),
                String::from("class"),
                String::from("struct"),
                String::from("interface"),
                String::from("enum"),
                String::from("method"),
                String::from("list"),
                String::from("array"),
                String::from("map"),
                String::from("set"),
                String::from("queue"),
                String::from("deque"),
                String::from("stack"),
                String::from("vector"),
                String::from("integer"),
                String::from("double"),
                String::from("char"),
                String::from("string"),
                String::from("bool"),
            ],
        }
    }

    fn validate_class(&self, class: &Class) -> bool {
        true
    }

    fn validate_classes(&self, classes: Vec<Class>) -> bool {
        true
    }

    fn validate_struct(&self, structure: &Struct) -> bool {
        true
    }

    fn validate_structs(&self, structs: Vec<&Struct>) -> bool {
        true
    }

    fn validate_interface(&self, interface: &Interface) -> bool {
        true
    }

    fn validate_interfaces(&self, interfaces: Vec<&Interface>) -> bool {
        true
    }

    fn validate_enum(&self, enumeration: &Enumeration) -> bool {
        true
    }

    fn validate_enums(&self, enumerations: Vec<&Enumeration>) -> bool {
        true
    }

    fn validate_function(&self, function: &Function) -> bool {
        true
    }

    fn validate_functions(&self, functions: Vec<&Function>) -> bool {
        true
    }

    fn validate_method(&self, method: &Method) -> bool {
        true
    }

    fn validate_variable(&self, variable: &Variable) -> bool {
        true
    }

    fn validate_settings(&self, settings: &Settings) -> bool {
        true
    }

    fn validate_project(&self, project: &Project) -> bool {
        true
    }

    pub fn validate(self, project: &Project) -> bool {
        true
    }
}
