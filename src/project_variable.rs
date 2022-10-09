pub mod variable {

    #[derive(Debug, PartialEq)]
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

    pub struct Builder {
        name: Option<String>,
        data_type: Option<String>,
        is_pointer: Option<bool>,
    }

    impl Builder {
        pub fn add_name(&mut self, name: String) -> &mut Builder {
            self.name = Some(name);
            self
        }

        pub fn add_data_type(&mut self, data_type: String) -> &mut Builder {
            self.data_type = Some(data_type);
            self
        }

        pub fn set_is_pointer(&mut self, is_pointer: bool) -> &mut Builder {
            self.is_pointer = Some(is_pointer);
            self
        }

        // *** BUILD ***

        pub fn build(self) -> Result<Variable, String> {
            // verify that required fields are present
            let name = match self.name {
                None => {
                    return Err(String::from("Variables cannot be built without a name"));
                }
                Some(settings) => settings,
            };

            let data_type = match self.data_type {
                None => {
                    return Err(String::from(
                        "Variables cannot be built without a data type",
                    ));
                }
                Some(settings) => settings,
            };

            let is_pointer = match self.is_pointer {
                None => false,
                Some(settings) => settings,
            };

            let structure = Variable {
                name,
                data_type,
                is_pointer,
            };

            Ok(structure)
        }
    }
}
