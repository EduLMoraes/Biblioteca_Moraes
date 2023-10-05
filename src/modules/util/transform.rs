use std::error::Error;

#[warn(dead_code)]
pub fn to_int(string: String) -> Result<i32, Box<dyn Error>>{
    
    let int = string.trim().parse::<i32>();
    
    match int {
        Ok(integer) => return Ok(integer),
        Err(_) => Err("Value invalid".into())
    }
}
