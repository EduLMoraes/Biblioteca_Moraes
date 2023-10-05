#[path = "../../modules/db/db.rs"]
mod db;
use db::*;

#[path = "../../modules/controller/structs.rs"]
mod structs;
use structs::User;

use std::error::Error;


pub async fn login(email: String, password: String) -> Result<User, Box<dyn Error>>{
    let db = DataBase::new();
    let user = db.search_by(&"users".to_string(), (Some(&"email".to_string()), Some(&email))).await.unwrap();

    if user == []{
       return Err("User not found".into());
    }

    for data in user{
        let (id, id_permission, id_book, name, surname, password_db, email): (i32, i32, i32, String, String, String, String) = mysql::from_row(data);

        if password == password_db{
            let mut vec = Vec::new();

            vec.push(email.to_string());        
            vec.push(id.to_string());        
            vec.push(id_permission.to_string());        
            vec.push(id_book.to_string());        
            vec.push(name.to_string());        
            vec.push(surname.to_string()); 

            return Ok(User::new(vec));
        }
    }

    Err("Password incorrect.".into())
}

pub async fn new_user((name, surname, email, password, confirm_password): (String, String, String, String, String), permission: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    let db = DataBase::new();

    if password == confirm_password {
        let response: Result<(), DataBaseError> = db.new_user((name, surname, email, password), permission).await;

        match response {
            Ok(_) => Ok("User created successfully".into()),
            Err(DataBaseError::EmailInvalid(_)) => Err("Email is invalid".into()),
            _ => Err("An error occurred while creating user".into()),
        }
    }else{
        Err("Password and confirm_password is different.".into())
    }
}

