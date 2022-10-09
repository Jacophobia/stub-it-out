pub mod structure {
    use crate::project_variable::variable::Variable;

    #[derive(PartialEq, Debug)]
    pub struct Struct {
        name: String,
        description: String,
        path: Option<String>,
        variables: Option<Vec<Variable>>,
        static_variables: Option<Vec<Variable>>,
    }

    impl Struct {
        pub fn builder() -> Builder {
            Builder::new()
        }
    }

    pub struct Builder {
        name: Option<String>,
        description: Option<String>,
        path: Option<String>,
        variables: Option<Vec<Variable>>,
        static_variables: Option<Vec<Variable>>,
    }

    impl Builder {
        fn new() -> Builder {
            Builder {
                name: None,
                description: None,
                path: None,
                variables: None,
                static_variables: None
            }
        }

        // *** SETTINGS ***

        pub fn add_name(&mut self, name: String) -> &mut Builder {
            self.name = Some(name);
            self
        }

        pub fn add_description(&mut self, description: String) -> &mut Builder {
            self.description = Some(description);
            self
        }

        pub fn add_path(&mut self, path: String) -> &mut Builder {
            self.path = Some(path);
            self
        }

        // *** VALUES ***

        pub fn add_variable(&mut self, variable: Variable) -> &mut Builder {
            match &mut self.variables {
                None => {
                    self.variables = Some(vec![variable]);
                }
                Some(variables) => {
                    variables.push(variable);
                }
            };
            self
        }

        pub fn add_static_variable(&mut self, static_variable: Variable) -> &mut Builder {
            match &mut self.static_variables {
                None => {
                    self.static_variables = Some(vec![static_variable]);
                }
                Some(static_variables) => {
                    static_variables.push(static_variable);
                }
            };
            self
        }

        // *** BUILD ***

        pub fn build(self) -> Result<Struct, String> {
            // verify that required fields are present
            let name = match self.name {
                None => {
                    return Err(String::from("Structs cannot be built without a name"));
                }
                Some(settings) => settings,
            };

            let description = match self.description {
                None => {
                    return Err(String::from(
                        "Structs cannot be built without a description",
                    ));
                }
                Some(settings) => settings,
            };

            let structure = Struct {
                name,
                description,
                path: self.path,
                variables: self.variables,
                static_variables: self.static_variables,
            };

            Ok(structure)
        }
    }
}
