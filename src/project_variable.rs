pub mod variable {

    #[derive(Debug, PartialEq)]
    pub struct Variable {
        name: String,
        data_type: String,
        is_pointer: bool,
    }

    impl Variable {
        pub fn builder() -> Builder {
            Builder::new()
        }
    }

    pub struct Builder {
        name: Option<String>,
        data_type: Option<String>,
        is_pointer: Option<bool>,
    }

    impl Builder {
        fn new() -> Builder {
            Builder {
                name: None,
                data_type: None,
                is_pointer: None,
            }
        }

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
