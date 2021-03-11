use repl_rs::{Command, Parameter, Result, Repl};

mod commands;
 
fn main() -> Result<()> {
    let mut repl = Repl::new(())
        .with_name("rurekal")
        .with_version("v0.1.0")
        .with_description("a cli calculator written in rust.")
        .add_command(
            Command::new("add", commands::add)
                .with_parameter(Parameter::new("num1").set_required(true)?)?
                .with_parameter(Parameter::new("num2").set_required(true)?)?
                .with_help("add two numbers together.")
        )
        .add_command(
            Command::new("subtract", commands::subtract)
                .with_parameter(Parameter::new("num1").set_required(true)?)?
                .with_parameter(Parameter::new("num2").set_required(true)?)?
                .with_help("subtract a number.")
        )
        .add_command(
            Command::new("multiply", commands::multiply)
                .with_parameter(Parameter::new("num1").set_required(true)?)?
                .with_parameter(Parameter::new("num2").set_required(true)?)?
                .with_help("multiply two numbers together.")
        )
        .add_command(
            Command::new("divide", commands::divide)
                .with_parameter(Parameter::new("num1").set_required(true)?)?
                .with_parameter(Parameter::new("num2").set_required(true)?)?
                .with_help("divide a number.")
        )
        .add_command(
            Command::new("version", commands::version)
                .with_help("display version information.")
        );

    repl.run()
}
