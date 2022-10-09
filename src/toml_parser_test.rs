#[cfg(test)]
mod test {
    use crate::toml_parser::toml;
    use crate::Project;

    #[test]
    fn from_string__each_top_level_type_is_present() {
        // setup
        let parser = toml::Parser::new();
        let input = each_type_toml_str();
        // execute
        let result = parser.str_to_project(input);
        // validate
        let expected_result = each_type_project();
        assert_eq!(expected_result, result);
    }

    #[test]
    fn from_string__classes_can_be_nested() {
        // setup

        // execute

        // validate
    }

    #[test]
    fn from_string__each_nested_attribute_is_converted() {
        // setup

        // execute

        // validate
    }

    fn each_type_project() -> Project {
        Project::new("Not yet implemented")
    }

    fn each_type_toml_str() -> String {
        let toml = r#"
        [settings]
        name = "Example"
        path = "./example"


        [enum.Color]
        options = [
          "BLUE",
          "GREEN",
          "YELLOW"
        ]

        [struct.StructName]
        value = "Color"

        [interface.Speakable.public.method.speak]
        return = "string"


        [class.Dog]
        parent = "Animal"
        [class.Dog.private]
        color = "Color"
        breed = "string"
        [class.Dog.public.method.getColor]
        return = "Color"
        [class.Dog.public.method.setColor]
        params = { color = "Color" }


        [class.Animal.private]
        age = "integer"


        [function.main]
        calls = [
          "Dog",
          "Dog.setColor",
          "Dog.getColor"
        ]"#;
        String::from(toml)
    }
}
