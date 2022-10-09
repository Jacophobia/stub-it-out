pub mod class {
    use crate::project_enum::enumeration::Enumeration;
    use crate::project_method::method::Method;
    use crate::project_struct::structure::Struct;
    use crate::project_variable::variable::Variable;

    #[derive(PartialEq, Debug)]
    pub enum ClassField {
        Variable(Variable),
        StaticVariable(Variable),
        Method(Method),
        StaticMethod(Method),
        Class(Class),
        Enumeration(Enumeration),
        Struct(Struct),
    }

    #[derive(PartialEq, Debug)]
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

    pub struct Builder {
        name: Option<String>,        // required
        description: Option<String>, // required
        path: Option<String>,
        parent: Option<String>,
        private: Option<Vec<ClassField>>,
        protected: Option<Vec<ClassField>>,
        public: Option<Vec<ClassField>>,
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

        pub fn add_parent(&mut self, parent: String) -> &mut Builder {
            self.parent = Some(parent);
            self
        }

        // *** VARIABLES ***

        // private
        pub fn add_private_variable(&mut self, variable: Variable) -> &mut Builder {
            match &mut self.private {
                None => {
                    self.private = Some(vec![ClassField::Variable(variable)]);
                }
                Some(variables) => {
                    variables.push(ClassField::Variable(variable));
                }
            };
            self
        }
        // private static
        pub fn add_private_static_variable(&mut self, variable: Variable) -> &mut Builder {
            match &mut self.private {
                None => {
                    self.private = Some(vec![ClassField::StaticVariable(variable)]);
                }
                Some(variables) => {
                    variables.push(ClassField::StaticVariable(variable));
                }
            };
            self
        }
        // public
        pub fn add_public_variable(&mut self, variable: Variable) -> &mut Builder {
            match &mut self.public {
                None => {
                    self.public = Some(vec![ClassField::Variable(variable)]);
                }
                Some(variables) => {
                    variables.push(ClassField::Variable(variable));
                }
            };
            self
        }
        // public static
        pub fn add_public_static_variable(&mut self, variable: Variable) -> &mut Builder {
            match &mut self.public {
                None => {
                    self.public = Some(vec![ClassField::StaticVariable(variable)]);
                }
                Some(variables) => {
                    variables.push(ClassField::StaticVariable(variable));
                }
            };
            self
        }

        // *** METHODS ***

        // private
        pub fn add_private_method(&mut self, method: Method) -> &mut Builder {
            match &mut self.private {
                None => {
                    self.private = Some(vec![ClassField::Method(method)]);
                }
                Some(methods) => {
                    methods.push(ClassField::Method(method));
                }
            };
            self
        }
        // private static
        pub fn add_private_static_method(&mut self, method: Method) -> &mut Builder {
            match &mut self.private {
                None => {
                    self.private = Some(vec![ClassField::StaticMethod(method)]);
                }
                Some(methods) => {
                    methods.push(ClassField::StaticMethod(method));
                }
            };
            self
        }
        // public
        pub fn add_public_method(&mut self, method: Method) -> &mut Builder {
            match &mut self.public {
                None => {
                    self.public = Some(vec![ClassField::Method(method)]);
                }
                Some(methods) => {
                    methods.push(ClassField::Method(method));
                }
            };
            self
        }
        // public static
        pub fn add_public_static_method(&mut self, method: Method) -> &mut Builder {
            match &mut self.public {
                None => {
                    self.public = Some(vec![ClassField::StaticMethod(method)]);
                }
                Some(methods) => {
                    methods.push(ClassField::StaticMethod(method));
                }
            };
            self
        }

        // *** ENUMS ***
        // private
        pub fn add_private_enum(&mut self, enumeration: Enumeration) -> &mut Builder {
            match &mut self.private {
                None => {
                    self.private = Some(vec![ClassField::Enumeration(enumeration)]);
                }
                Some(enums) => {
                    enums.push(ClassField::Enumeration(enumeration));
                }
            };
            self
        }
        // public
        pub fn add_public_enum(&mut self, enumeration: Enumeration) -> &mut Builder {
            match &mut self.public {
                None => {
                    self.public = Some(vec![ClassField::Enumeration(enumeration)]);
                }
                Some(enums) => {
                    enums.push(ClassField::Enumeration(enumeration));
                }
            };
            self
        }

        // *** STRUCTS ***
        // private
        pub fn add_private_struct(&mut self, structure: Struct) -> &mut Builder {
            match &mut self.private {
                None => {
                    self.private = Some(vec![ClassField::Struct(structure)]);
                }
                Some(structs) => {
                    structs.push(ClassField::Struct(structure));
                }
            };
            self
        }
        // public
        pub fn add_public_struct(&mut self, structure: Struct) -> &mut Builder {
            match &mut self.public {
                None => {
                    self.public = Some(vec![ClassField::Struct(structure)]);
                }
                Some(structs) => {
                    structs.push(ClassField::Struct(structure));
                }
            };
            self
        }

        // *** CLASSES ***
        // private
        pub fn add_private_class(&mut self, class: Class) -> &mut Builder {
            match &mut self.private {
                None => {
                    self.private = Some(vec![ClassField::Class(class)]);
                }
                Some(classes) => {
                    classes.push(ClassField::Class(class));
                }
            };
            self
        }
        // public
        pub fn add_public_class(&mut self, class: Class) -> &mut Builder {
            match &mut self.public {
                None => {
                    self.public = Some(vec![ClassField::Class(class)]);
                }
                Some(classes) => {
                    classes.push(ClassField::Class(class));
                }
            };
            self
        }

        pub fn build(self) -> Result<Class, String> {
            // verify that required fields are present
            let name = match self.name {
                None => {
                    return Err(String::from("Projects cannot be built without settings"));
                }
                Some(settings) => settings,
            };

            let description = match self.description {
                None => {
                    return Err(String::from(
                        "Projects cannot be built without a description",
                    ));
                }
                Some(settings) => settings,
            };

            let class = Class {
                name,
                description,
                path: self.path,
                parent: self.parent,
                private: self.private,
                protected: self.protected,
                public: self.public,
            };

            Ok(class)
        }
    }
}
