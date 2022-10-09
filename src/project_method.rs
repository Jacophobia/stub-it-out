pub mod method {
    use crate::project_variable::variable::Variable;

    #[derive(PartialEq, Debug)]
    pub struct Method {
        name: String,
        description: String,
        params: Option<Vec<Variable>>,
        calls: Option<Vec<String>>,
        return_type: Option<String>,
    }

    impl Method {
        pub fn builder() -> Builder {
            Builder::new()
        }
    }

    pub struct Builder {
        name: Option<String>,
        description: Option<String>,
        params: Option<Vec<Variable>>,
        calls: Option<Vec<String>>,
        return_type: Option<String>,
    }

    impl Builder {
        fn new() -> Builder {
            Builder {
                name: None,
                description: None,
                params: None,
                calls: None,
                return_type: None
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

        // *** VALUES ***
        pub fn add_calls(&mut self, call: String) -> &mut Builder {
            match &mut self.calls {
                None => {
                    self.calls = Some(vec![call]);
                }
                Some(calls) => {
                    calls.push(call);
                }
            };
            self
        }

        pub fn add_param(&mut self, param: Variable) -> &mut Builder {
            match &mut self.params {
                None => {
                    self.params = Some(vec![param]);
                }
                Some(params) => {
                    params.push(param);
                }
            };
            self
        }

        pub fn add_return_type(&mut self, return_type: String) -> &mut Builder {
            self.return_type = Some(return_type);
            self
        }

        // *** BUILD ***

        pub fn build(self) -> Result<Method, String> {
            // verify that required fields are present
            let name = match self.name {
                None => {
                    return Err(String::from("Methods cannot be built without a name"));
                }
                Some(settings) => settings,
            };

            let description = match self.description {
                None => {
                    return Err(String::from(
                        "Methods cannot be built without a description",
                    ));
                }
                Some(settings) => settings,
            };

            let method = Method {
                name,
                description,
                params: self.params,
                calls: self.calls,
                return_type: self.return_type,
            };

            Ok(method)
        }
    }
}
