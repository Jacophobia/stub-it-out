/**
 * TODO:
 *  Place all optional fields into the Option enum
 *  Create a hash set containing all available data types which can be queried
 *   to see if a variable data type is valid. Add all default values to it
 *   manually. Maybe add a command line argument that skips this validation if
 *   the user wants to use unsupported data types.
 **/
pub use crate::validator::Validator;

pub struct Settings {
    name: String,
    path: Option<String>,
    other_config_files: Option<Vec<String>>,
}

impl Settings {
    pub fn new(name: &str) -> Settings {
        Settings {
            name: String::from(name),
            path: None,
            other_config_files: None,
        }
    }
}

pub struct Variable {
    name: String,
    data_type: String,
    is_pointer: bool,
}

impl Variable {
    pub fn new(name: &str, data_type: &str) -> Variable {
        Variable {
            name: String::from(name),
            data_type: String::from(data_type),
            is_pointer: false,
        }
    }
}

pub struct Enumeration {
    name: String,
    path: Option<String>,
    options: Vec<String>,
}

impl Enumeration {
    pub fn new(name: &str, options: Vec<String>) -> Enumeration {
        Enumeration {
            name: String::from(name),
            path: None,
            options,
        }
    }
}

pub struct Struct {
    name: String,
    description: String,
    path: Option<String>,
    variables: Option<Vec<Variable>>,
    static_variables: Option<Vec<Variable>>,
}

impl Struct {
    pub fn new(name: &str, description: &str) -> Struct {
        Struct {
            name: String::from(name),
            description: String::from(description),
            path: None,
            variables: None,
            static_variables: None,
        }
    }
}

pub struct Function {
    name: String,
    description: String,
    path: Option<String>,
    params: Option<Vec<Variable>>,
    calls: Option<Vec<String>>,
    return_type: Option<String>,
}

impl Function {
    pub fn new(name: &str, description: &str) -> Function {
        Function {
            name: String::from(name),
            description: String::from(description),
            path: None,
            params: None,
            calls: None,
            return_type: None,
        }
    }
}

pub struct Method {
    name: String,
    description: String,
    params: Option<Vec<Variable>>,
    calls: Option<Vec<String>>,
    return_type: Option<String>,
}

impl Method {
    pub fn new(name: &str, description: &str) -> Method {
        Method {
            name: String::from(name),
            description: String::from(description),
            params: None,
            calls: None,
            return_type: None,
        }
    }
}

pub struct Interface {
    name: String,
    description: String,
    path: Option<String>,
    methods: Option<Vec<Method>>,
    static_methods: Option<Vec<Method>>,
}

impl Interface {
    pub fn new(name: &str, description: &str) -> Interface {
        Interface {
            name: String::from(name),
            description: String::from(description),
            path: None,
            methods: None,
            static_methods: None,
        }
    }
}

pub enum ClassField {
    Variable(Variable),
    StaticVariable(Variable),
    Method(Method),
    StaticMethod(Method),
    Class(Class),
    Enumeration(Enumeration),
    Struct(Struct),
}

pub struct Class {
    name: String,
    description: String,
    path: Option<String>,
    parent: Option<String>,
    private: Option<Vec<ClassField>>,
    protected: Option<Vec<ClassField>>,
    public: Option<Vec<ClassField>>,
}

impl Class {
    pub fn new(name: &str, description: &str) -> Class {
        Class {
            name: String::from(name),
            description: String::from(description),
            path: None,
            parent: None,
            private: None,
            protected: None,
            public: None,
        }
    }
}

pub struct Project {
    settings: Settings,
    enums: Option<Vec<Enumeration>>,
    functions: Option<Vec<Function>>,
    structs: Option<Vec<Struct>>,
    interfaces: Option<Vec<Interface>>,
    classes: Option<Vec<Class>>,
}

impl Project {
    pub fn new(project_name: &str) -> Project {
        Project {
            settings: Settings::new(project_name),
            enums: None,
            functions: None,
            structs: None,
            interfaces: None,
            classes: None,
        }
    }
}
