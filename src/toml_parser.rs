pub mod toml {
    use crate::Project;
    use toml_edit::{Document, Value};

    pub struct Parser {}

    impl Parser {
        pub fn new() -> Parser {
            Parser {}
        }

        pub fn toml_to_project(&self, toml: Document) -> Project {
            Project::new("Not yet implemented")
        }

        pub fn str_to_project(&self, toml: String) -> Project {
            self.string_to_project(String::from(toml))
        }

        pub fn string_to_project(&self, toml: String) -> Project {
            Project::new("Not yet implemented")
        }

        pub fn path_to_project(&self, toml: String) -> Project {
            Project::new("Not yet implemented")
        }
    }
}
