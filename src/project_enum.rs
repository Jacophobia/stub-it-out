pub mod enumeration {

    #[derive(PartialEq, Debug)]
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

    pub struct Builder {
        name: Option<String>,
        path: Option<String>,
        options: Option<Vec<String>>,
    }

    impl Builder {
        // *** SETTINGS ***
        pub fn add_name(&mut self, name: String) -> &mut Builder {
            self.name = Some(name);
            self
        }

        pub fn add_path(&mut self, path: String) -> &mut Builder {
            self.path = Some(path);
            self
        }

        pub fn add_option(&mut self, option: String) -> &mut Builder {
            match &mut self.options {
                None => {
                    self.options = Some(vec![option]);
                }
                Some(options) => {
                    options.push(option);
                }
            };
            self
        }

        // *** BUILD ***

        pub fn build(self) -> Result<Enumeration, String> {
            // verify that required fields are present
            let name = match self.name {
                None => {
                    return Err(String::from("Enums cannot be built without a name"));
                }
                Some(settings) => settings,
            };

            let options = match self.options {
                None => {
                    return Err(String::from("Enums cannot be built without options"));
                }
                Some(options) => options,
            };

            let enumeration = Enumeration {
                name,
                path: self.path,
                options,
            };

            Ok(enumeration)
        }
    }
}
