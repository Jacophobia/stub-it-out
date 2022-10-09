pub mod function {
    use crate::project_variable::variable::Variable;

    #[derive(PartialEq, Debug)]
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

    pub struct Builder {}

    impl Builder {}
}
