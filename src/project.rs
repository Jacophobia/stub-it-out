pub mod project {
    /**
     * TODO:
     *  Place all optional fields into the Option enum
     *  Create a hash set containing all available data types which can be queried
     *   to see if a variable data type is valid. Add all default values to it
     *   manually. Maybe add a command line argument that skips this validation if
     *   the user wants to use unsupported data types.
     **/
    use crate::project_class::class::Class;
    use crate::project_enum::enumeration::Enumeration;
    use crate::project_function::function::Function;
    use crate::project_interface::interface::Interface;
    use crate::project_settings::settings::Settings;
    use crate::project_struct::structure::Struct;

    #[derive(PartialEq, Debug)]
    pub struct Project {
        settings: Settings,
        enums: Option<Vec<Enumeration>>,
        functions: Option<Vec<Function>>,
        structs: Option<Vec<Struct>>,
        interfaces: Option<Vec<Interface>>,
        classes: Option<Vec<Class>>,
    }

    impl Project {
        pub fn builder() -> Builder {
            Builder::new()
        }
        pub fn new(name: &str) -> Project {
            Project {
                settings: Settings::new(name),
                enums: None,
                functions: None,
                structs: None,
                interfaces: None,
                classes: None,
            }
        }
    }

    pub struct Builder {
        settings: Option<Settings>,
        enums: Option<Vec<Enumeration>>,
        functions: Option<Vec<Function>>,
        structs: Option<Vec<Struct>>,
        interfaces: Option<Vec<Interface>>,
        classes: Option<Vec<Class>>,
    }

    impl Builder {
        pub fn new() -> Builder {
            Builder {
                settings: None,
                enums: None,
                functions: None,
                structs: None,
                interfaces: None,
                classes: None,
            }
        }

        pub fn add_settings(&mut self, settings: Settings) -> &mut Builder {
            self.settings = Some(settings);
            self
        }

        pub fn add_enum(&mut self, enumeration: Enumeration) -> &mut Builder {
            match &mut self.enums {
                None => {
                    self.enums = Some(vec![enumeration]);
                }
                Some(enums) => {
                    enums.push(enumeration);
                }
            };
            self
        }

        pub fn add_function(&mut self, function: Function) -> &mut Builder {
            match &mut self.functions {
                None => {
                    self.functions = Some(vec![function]);
                }
                Some(functions) => {
                    functions.push(function);
                }
            };
            self
        }

        pub fn add_struct(&mut self, structure: Struct) -> &mut Builder {
            match &mut self.structs {
                None => {
                    self.structs = Some(vec![structure]);
                }
                Some(structs) => {
                    structs.push(structure);
                }
            };
            self
        }

        pub fn add_interface(&mut self, interface: Interface) -> &mut Builder {
            match &mut self.interfaces {
                None => {
                    self.interfaces = Some(vec![interface]);
                }
                Some(interfaces) => {
                    interfaces.push(interface);
                }
            };
            self
        }

        pub fn add_class(&mut self, class: Class) -> &mut Builder {
            match &mut self.classes {
                None => {
                    self.classes = Some(vec![class]);
                }
                Some(classes) => {
                    classes.push(class);
                }
            };
            self
        }

        pub fn build(self) -> Result<Project, String> {
            // settings is a required field, so the build will fail if we don't have it
            let settings = match self.settings {
                None => {
                    return Err(String::from("Project cannot be built without settings"));
                }
                Some(settings) => settings,
            };

            let project = Project {
                settings,
                enums: self.enums,
                functions: self.functions,
                structs: self.structs,
                interfaces: self.interfaces,
                classes: self.classes,
            };

            Ok(project)
        }
    }
}
