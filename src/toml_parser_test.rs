#[cfg(test)]
mod test {
    use crate::project_class::class::Class;
    use crate::project_enum::enumeration::Enumeration;
    use crate::project_function::function::Function;
    use crate::project_interface::interface::Interface;
    use crate::project_method::method::Method;
    use crate::project_settings::settings::Settings;
    use crate::project_struct::structure::Struct;
    use crate::project_variable::variable::Variable;
    use crate::toml_parser::toml;
    use crate::Project;

    #[test]
    fn from_string() {
        // setup
        let parser = toml::Parser::new();
        let input = each_type_toml_str();
        // execute
        let result = parser.str_to_project(input);
        // validate
        let expected_result = each_type_project();
        // println!("{:#?}", result);
        // println!("\n\nvs\n\n");
        // println!("{:#?}", expected_result);
        assert_eq!(expected_result, result);
    }

    fn each_type_project() -> Project {
        let mut project_builder = Project::builder();

        let mut settings_builder = Settings::builder();
        settings_builder
            .add_name(String::from("ProjectName"))
            .add_path(String::from("./"))
            .add_other_config_files(String::from("./other_config_file.toml"));

        let mut enum_builder = Enumeration::builder();
        enum_builder
            .add_name(String::from("EnumName"))
            .add_path(String::from("./EnumName"))
            .add_option(String::from("Option 1"))
            .add_option(String::from("Option 2"))
            .add_option(String::from("Option 3"));

        let mut param_one_builder = Variable::builder();
        let mut param_two_builder = Variable::builder();
        param_one_builder
            .add_name(String::from("ParameterOne"))
            .add_data_type(String::from("integer"));
        param_two_builder
            .add_name(String::from("ParameterTwo"))
            .add_data_type(String::from("integer"));

        let mut function_builder = Function::builder();
        function_builder
            .add_name(String::from("FunctionName"))
            .add_description(String::from("This is a function"))
            .add_path(String::from("./FunctionName"))
            .add_param(
                param_one_builder
                    .build()
                    .expect("Unable to build parameter one"),
            )
            .add_param(
                param_two_builder
                    .build()
                    .expect("Unable to build parameter two"),
            )
            .add_call(String::from("ClassName.PublicMethodName"))
            .add_call(String::from("InterfaceName.PublicMethodName"))
            .add_call(String::from("FunctionName2"))
            .add_return_type(String::from("integer"));

        let mut function_builder_2 = Function::builder();
        function_builder_2
            .add_name(String::from("FunctionName2"))
            .add_description(String::from("This is another function"));

        let mut variable_builder = Variable::builder();
        variable_builder
            .add_name(String::from("VariableName"))
            .add_data_type(String::from("integer"));

        let mut variable_builder_2 = Variable::builder();
        variable_builder_2
            .add_name(String::from("VariableName2"))
            .add_data_type(String::from("integer"));

        let mut struct_builder = Struct::builder();
        struct_builder
            .add_name(String::from("StructName"))
            .add_description(String::from("This is a struct"))
            .add_path(String::from("./StructName"))
            .add_variable(
                variable_builder
                    .build()
                    .expect("Unable to build StructName.VariableName"),
            )
            .add_static_variable(
                variable_builder_2
                    .build()
                    .expect("Unable to build StructName.VariableName2"),
            );

        // public (class)
        let mut public_method_builder_c = Method::builder();
        public_method_builder_c
            .add_name(String::from("PublicMethodName"))
            .add_description(String::from("This is a method"));

        // private (class)
        let mut private_method_builder_c = Method::builder();
        private_method_builder_c
            .add_name(String::from("PrivateMethodName"))
            .add_description(String::from("This is a method"));

        // public static (class)
        let mut public_static_method_builder_c = Method::builder();
        public_static_method_builder_c
            .add_name(String::from("PublicStaticMethodName"))
            .add_description(String::from("This is a method"));

        // private static (class)
        let mut private_static_method_builder_c = Method::builder();
        private_static_method_builder_c
            .add_name(String::from("PrivateStaticMethodName"))
            .add_description(String::from("This is a method"));

        // public (interface)
        let mut public_method_builder_i = Method::builder();
        public_method_builder_i
            .add_name(String::from("PublicMethodName"))
            .add_description(String::from("This is a method"));

        // private (interface)
        let mut private_method_builder_i = Method::builder();
        private_method_builder_i
            .add_name(String::from("PrivateMethodName"))
            .add_description(String::from("This is a method"));

        // public
        let mut public_variable_builder = Variable::builder();
        public_variable_builder
            .add_name(String::from("PublicVariableName"))
            .add_data_type(String::from("integer"));

        // private
        let mut private_variable_builder = Variable::builder();
        private_variable_builder
            .add_name(String::from("PrivateVariableName"))
            .add_data_type(String::from("integer"));

        // public static
        let mut public_static_variable_builder = Variable::builder();
        public_static_variable_builder
            .add_name(String::from("PublicStaticVariableName"))
            .add_data_type(String::from("integer"));

        // private static
        let mut private_static_variable_builder = Variable::builder();
        private_static_variable_builder
            .add_name(String::from("PrivateStaticVariableName"))
            .add_data_type(String::from("integer"));

        let mut interface_builder = Interface::builder();
        interface_builder
            .add_name(String::from("InterfaceName"))
            .add_description(String::from("This is an interface"))
            .add_public_method(
                public_method_builder_i
                    .build()
                    .expect("Unable to build public method for interface"),
            )
            .add_private_method(
                private_method_builder_i
                    .build()
                    .expect("Unable to build private method for interface"),
            );

        let mut inner_class_builder = Class::builder();
        inner_class_builder
            .add_name(String::from("InnerClassName"))
            .add_description(String::from("This is a class"));

        let mut inner_enum_builder = Enumeration::builder();
        inner_enum_builder
            .add_name(String::from("InnerEnumName"))
            .add_option(String::from("Option 1"))
            .add_option(String::from("Option 2"))
            .add_option(String::from("Option 3"));

        let mut inner_struct_builder = Struct::builder();
        inner_struct_builder
            .add_name(String::from("InnerStructName"))
            .add_description(String::from("This is a struct"));

        let mut class_builder = Class::builder();
        class_builder
            .add_name(String::from("ClassName"))
            .add_description(String::from("This is a class"))
            .add_public_method(
                public_method_builder_c
                    .build()
                    .expect("Unable to build public method for class"),
            )
            .add_public_static_method(
                public_static_method_builder_c
                    .build()
                    .expect("Unable to build public static method for class"),
            )
            .add_private_method(
                private_method_builder_c
                    .build()
                    .expect("Unable to build private method for class"),
            )
            .add_private_static_method(
                private_static_method_builder_c
                    .build()
                    .expect("Unable to build private static method for class"),
            )
            .add_public_variable(
                public_variable_builder
                    .build()
                    .expect("Unable to build public variable for class"),
            )
            .add_public_static_variable(
                public_static_variable_builder
                    .build()
                    .expect("Unable to build public static variable for class"),
            )
            .add_private_variable(
                private_variable_builder
                    .build()
                    .expect("Unable to build private variable for class"),
            )
            .add_private_static_variable(
                private_static_variable_builder
                    .build()
                    .expect("Unable to build public static variable for class"),
            )
            .add_public_class(
                inner_class_builder
                    .build()
                    .expect("Unable to build public class for class"),
            )
            .add_public_enum(
                inner_enum_builder
                    .build()
                    .expect("Unable to build public enum for class"),
            )
            .add_public_struct(
                inner_struct_builder
                    .build()
                    .expect("Unable to build public struct for class"),
            );

        project_builder
            .add_settings(
                settings_builder
                    .build()
                    .expect("Unable to build settings for project"),
            )
            .add_enum(
                enum_builder
                    .build()
                    .expect("Unable to build enum for project"),
            )
            .add_function(
                function_builder
                    .build()
                    .expect("Unable to build function for project"),
            )
            .add_function(
                function_builder_2
                    .build()
                    .expect("Unable to build function for project"),
            )
            .add_struct(
                struct_builder
                    .build()
                    .expect("Unable to build struct for project"),
            )
            .add_interface(
                interface_builder
                    .build()
                    .expect("Unable to build interface for project"),
            )
            .add_class(
                class_builder
                    .build()
                    .expect("Unable to build class for project"),
            );

        project_builder.build().expect("Unable to build project")
    }

    fn each_type_toml_str() -> String {
        let toml = r#"
            [settings]
            name = "ProjectName"
            path = "./"
            other_config_files = "./other_config_file.toml"

            [enum.EnumName]
            path = "./EnumName"
            options = [
              "Option 1",
              "Option 2",
              "Option 3"
            ]

            [function.FunctionName]
            description = "This is a function"
            path = "./FunctionName"
            params = { ParameterOne = "integer", ParameterTwo = "integer" }
            calls = [
              "ClassName.PublicMethodName",
              "InterfaceName.PublicMethodName",
              "FunctionName2"
            ]
            return = "integer"

            [function.FunctionName2]
            description = "This is another function"

            [struct.StructName]
            description = "This is a struct"
            path = "./StructName"
            VariableName = "integer"
            [struct.StructName.static]
            VariableName2 = "integer"

            [interface.InterfaceName]
            description = "This is an interface"
            [interface.InterfaceName.public.method.PublicMethodName]
            description = "This is a method"
            [interface.InterfaceName.private.method.PrivateMethodName]
            description = "This is a method"

            [class.ClassName]
            description = "This is a class"
            [class.ClassName.private]
            PrivateVariableName = "integer"
            [class.ClassName.private.method.PrivateMethodName]
            description = "This is a method"
            [class.ClassName.private.static]
            PrivateStaticVariableName = "integer"
            [class.ClassName.private.static.method.PrivateStaticMethodName]
            description = "This is a method"
            [class.ClassName.public]
            PublicVariableName = "integer"
            [class.ClassName.public.class.InnerClassName]
            description = "This is a class"
            [class.ClassName.public.enum.InnerEnumName]
            options = [
              "Option 1",
              "Option 2",
              "Option 3"
            ]
            [class.ClassName.public.struct.InnerStructName]
            description = "This is a struct"
            [class.ClassName.public.method.PublicMethodName]
            description = "This is a method"
            [class.ClassName.public.static]
            PublicStaticVariableName = "integer"
            [class.ClassName.public.static.method.PublicStaticMethodName]
            description = "This is a method"
        "#;
        String::from(toml)
    }
}
