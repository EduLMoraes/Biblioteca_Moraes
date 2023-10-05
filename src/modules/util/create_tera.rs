use tera::{Tera, Value};
use serde_json::to_string;

pub fn create() -> Tera{
    let mut tera = Tera::new("./src/modules/templates/view/admin/*.tera").expect("Erro ao carregar templates");
    
    tera.register_filter("to_json", |value: &Value, _: &std::collections::HashMap<String, Value>| {
        match to_string(&value) {
            Ok(json) => Ok(Value::String(json)),
            Err(err) => Err(tera::Error::msg(format!("Failed to convert to JSON: {}", err))),
        }
    });
    
    tera
}
