use std::collections::HashMap;
use repl_rs::{Value, Result, Convert};

pub fn add<T>(args: HashMap<String, Value>, _context: &mut T) -> Result<Option<String>> {
    let num1: f64 = args["num1"].convert()?;
    let num2: f64 = args["num2"].convert()?;
 
    Ok(Some((num1 + num2).to_string()))
}

pub fn subtract<T>(args: HashMap<String, Value>, _context: &mut T) -> Result<Option<String>> {
    let num1: f64 = args["num1"].convert()?;
    let num2: f64 = args["num2"].convert()?;
 
    Ok(Some((num1 - num2).to_string()))
}

pub fn multiply<T>(args: HashMap<String, Value>, _context: &mut T) -> Result<Option<String>> {
    let num1: f64 = args["num1"].convert()?;
    let num2: f64 = args["num2"].convert()?;
 
    Ok(Some((num1 * num2).to_string()))
}

pub fn divide<T>(args: HashMap<String, Value>, _context: &mut T) -> Result<Option<String>> {
    let num1: f64 = args["num1"].convert()?;
    let num2: f64 = args["num2"].convert()?;
 
    Ok(Some((num1 / num2).to_string()))
}

pub fn version<T>(_args: HashMap<String, Value>, _context: &mut T) -> Result<Option<String>> {
    Ok(Some(String::from("rurekal v0.1.0")))
}