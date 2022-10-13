pub mod toml {
    use crate::project_settings::settings::Settings;
    use crate::Project;
    use toml_edit::{Document, Value};

    pub struct Parser {}

    impl Parser {
        pub fn new() -> Parser {
            Parser {}
        }

        // pub fn toml_to_project(&self, toml: Document) -> Project {
        //     Project::new("Not yet implemented")
        // }
        //
        pub fn str_to_project(&self, toml: String) -> Project {
            self.string_to_project(String::from(toml))
        }
        //
        pub fn string_to_project(&self, toml: String) -> Project {
            let mut settings_builder = Settings::builder();
            settings_builder.add_name(String::from("Not yet implemented"));
            let settings = settings_builder.build().expect("Unable to build settings");
            let mut project_builder = Project::builder();
            project_builder.add_settings(settings);
            project_builder.build().expect("Unable to build project")
        }
        //
        // pub fn path_to_project(&self, toml: String) -> Project {
        //     Project::new("Not yet implemented")
        // }
    }
}
