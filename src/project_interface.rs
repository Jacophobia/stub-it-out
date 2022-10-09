pub mod interface {
    use crate::project_method::method::Method;

    #[derive(PartialEq, Debug)]
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

    pub struct Builder {
        name: Option<String>,
        description: Option<String>,
        path: Option<String>,
        methods: Option<Vec<Method>>,
        static_methods: Option<Vec<Method>>,
    }

    impl Builder {
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

        // *** METHODS ***

        // methods
        pub fn add_private_method(&mut self, method: Method) -> &mut Builder {
            match &mut self.methods {
                None => {
                    self.methods = Some(vec![method]);
                }
                Some(methods) => {
                    methods.push(method);
                }
            };
            self
        }
        // static_methods
        pub fn add_public_method(&mut self, method: Method) -> &mut Builder {
            match &mut self.static_methods {
                None => {
                    self.static_methods = Some(vec![method]);
                }
                Some(methods) => {
                    methods.push(method);
                }
            };
            self
        }

        // *** BUILD ***

        pub fn build(self) -> Result<Interface, String> {
            // verify that required fields are present
            let name = match self.name {
                None => {
                    return Err(String::from("Interfaces cannot be built without a name"));
                }
                Some(settings) => settings,
            };

            let description = match self.description {
                None => {
                    return Err(String::from(
                        "Interfaces cannot be built without a description",
                    ));
                }
                Some(settings) => settings,
            };

            let interface = Interface {
                name,
                description,
                path: self.path,
                methods: self.methods,
                static_methods: self.static_methods,
            };

            Ok(interface)
        }
    }
}
