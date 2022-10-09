pub mod settings {

    #[derive(PartialEq, Debug)]
    pub struct Settings {
        name: String,
        path: Option<String>,
        other_config_files: Option<Vec<String>>,
    }

    impl Settings {
        pub fn builder() -> Builder {
            Builder::new()
        }
    }

    pub struct Builder {
        name: Option<String>,
        path: Option<String>,
        other_config_files: Option<Vec<String>>,
    }

    impl Builder {
        fn new() -> Builder {
            Builder {
                name: None,
                path: None,
                other_config_files: None
            }
        }

        // *** SETTINGS ***

        pub fn add_name(&mut self, name: String) -> &mut Builder {
            self.name = Some(name);
            self
        }

        pub fn add_path(&mut self, path: String) -> &mut Builder {
            self.path = Some(path);
            self
        }

        // *** VALUES ***

        pub fn add_other_config_files(&mut self, other_config_file: String) -> &mut Builder {
            match &mut self.other_config_files {
                None => {
                    self.other_config_files = Some(vec![other_config_file]);
                }
                Some(other_config_files) => {
                    other_config_files.push(other_config_file);
                }
            };
            self
        }

        // *** BUILD ***

        pub fn build(self) -> Result<Settings, String> {
            // verify that required fields are present
            let name = match self.name {
                None => {
                    return Err(String::from("Methods cannot be built without a name"));
                }
                Some(settings) => settings,
            };

            let settings = Settings {
                name,
                path: self.path,
                other_config_files: self.other_config_files,
            };

            Ok(settings)
        }
    }
}
