
/// Esse módulo contém todas as estruturas utilizadas no projeto,
/// com exceção da struct DataBase.

use rocket::FromForm;
use serde::{Serialize, Deserialize};
use std::error::Error;
use mysql::*;
use tokio::runtime;
use rocket_contrib::json::Json;


#[derive(Serialize, Deserialize, Debug)]
pub struct Book{
    pub id: i32,
    pub publisher: String,
    pub author: String,
    pub title: String,
    pub category: String,
    pub edition: String,
    pub date: String,
    pub description: String,
    pub qnt: String
}

#[path="../db/db.rs"]
pub mod db;
use db::*;

#[path = "../util/transform.rs"]
pub mod transform;
use transform::*;

#[derive(Serialize, Deserialize)]
pub struct User{
    pub email: String,
    pub id: i32,
    pub permission: String,
    pub book_allocated: String,
    pub name: String,
    pub surname: String,
} pub trait UserPermissions{
    fn connect_db(&self) -> DataBase;
    fn search_book(&self) -> Result<Vec<Book>, DataBaseError>;
    fn order_book_by(&self, column: &str) -> Result<Vec<Book>, Box<dyn Error>>; 
} pub trait LibrarianPermissions: UserPermissions{
    fn add_book(&self, book: Json<BookJS>) -> Result<(), DataBaseError>;
    fn edit_qnt(&self, id: String, new_value: String) -> Result<(), DataBaseError>;
} pub trait AdminPermissions: LibrarianPermissions{
    fn search_user(&self) -> Result<Vec<User>, DataBaseError>;
    fn order_user_by(&self, column: &str) -> Result<Vec<User>, Box<dyn Error>>;
    fn delete(&self, id: String, table: String) -> Result<(), DataBaseError>;
    fn edit(&self,column: String, id: String, new_value: String, table: String) -> Result<(), DataBaseError>;
}

/// Gerenciamento de permissões acoplada ao usuário,
/// só terá as impl aquele usuário com nível de permissão.
#[allow(dead_code)]
impl User{
    /// Creates a new [`User`].
    pub fn new(data_user: Vec<String>) -> User{
        User { 
            email: data_user[0].clone(), 
            id: data_user[1].trim().parse::<i32>().unwrap(), 
            permission: data_user[2].clone(),
            book_allocated: data_user[3].clone(),
            name: data_user[4].clone(),
            surname: data_user[5].clone(),
        }
    }
} impl UserPermissions for User {
    fn connect_db(&self) -> DataBase{
        db::DataBase::new()
    }
    fn search_book(&self) -> Result<Vec<Book>, DataBaseError>{
        let db = self.connect_db();
        let rt = runtime::Runtime::new().unwrap();


        let rows = rt.block_on(db.search_by(&"books".to_string(), (None, None))).unwrap();
        let mut collection: Vec<Book> = Vec::new();

        for row in rows{
            let (id, id_pub_company, id_author, title, cat, edit, date, desc, qnt): (i32, i32, i32, String, String, String, String, String, i32) = mysql::from_row(row);

            let authors = rt.block_on(db.search_by(&"author".to_string(), (Some(&"id".to_string()), Some(&id_author.to_string())))).unwrap();
            let pub_comp = rt.block_on(db.search_by(&"publishing_company".to_string(), (Some(&"id".to_string()), Some(&id_pub_company.to_string())))).unwrap();

            let mut name_author: String = String::new();
            let mut name_pub_company: String = String::new();

            for author in authors{
                let (_, name):(i32, String) = mysql::from_row(author);
                name_author = name;
            }
            for pub_company in pub_comp{
                let (_, name):(i32, String) = mysql::from_row(pub_company);
                name_pub_company = name;
            }

            let book: Book = Book { 
                id: id, 
                publisher:name_pub_company, 
                author: name_author, 
                title: title, 
                category: cat, 
                edition: edit, 
                date: date, 
                description: desc, 
                qnt: qnt.to_string()
            };

            collection.push(book);
        }
        
        Ok(collection)
    }
    fn order_book_by(&self, column: &str) -> Result<Vec<Book>, Box<dyn Error>> {
        let mut books = self.search_book().unwrap();

        match column {
            "edition" => books.sort_by(|a, b| a.edition.cmp(&b.edition)),
            "title" => books.sort_by(|a, b| a.title.cmp(&b.title)),
            "author" => books.sort_by(|a, b| a.author.cmp(&b.author)),
            "publisher" => books.sort_by(|a, b| a.publisher.cmp(&b.publisher)),
            "category" => books.sort_by(|a, b| a.category.cmp(&b.category)),
            _ => return Err("Column not found".into())
        }

        Ok(books)
    }
} impl LibrarianPermissions for User{
    fn add_book(&self, book: Json<BookJS>) -> Result<(), DataBaseError> {

        if !(to_int(self.permission.clone()).unwrap() <= 2){
            return Err(DataBaseError::InvalidPermission("Permission denied".to_string()));
        }

        let db = self.connect_db();
        let rt = runtime::Runtime::new().unwrap();


        rt.block_on(db.new_book((
            book.title.clone(), 
            book.category.clone(),
            book.edition.clone(),
            book.publishing.clone(),
            book.description.clone(),
            book.author.clone(),
            book.pub_company.clone() 
        ))).unwrap();

        Ok(())
    }
    fn edit_qnt(&self, id: String, new_value: String) -> Result<(), DataBaseError> {

        if !(to_int(self.permission.clone()).unwrap() <= 2){
            return Err(DataBaseError::InvalidPermission(("ID value invalid").to_string()));
        }

        let db = self.connect_db();
        let rt = runtime::Runtime::new().unwrap();

        rt.block_on(db.edit_by("qnt_specimens".to_string(), id, new_value, "books".to_string())).unwrap();

        Ok(())
    }
} impl AdminPermissions for User{
    fn search_user(&self) -> Result<Vec<User>, DataBaseError>{

        if !(to_int(self.permission.clone()).unwrap() <= 1){
            return Err(DataBaseError::InvalidPermission("Permission denied".to_string()));
        }

        let db = self.connect_db();
        let rt = runtime::Runtime::new().unwrap();


        let rows = rt.block_on(db.search_by(&"users".to_string(), (Some(&"id_permission".to_string()), Some(&"2".to_string())))).unwrap();

        let mut librarians: Vec<User> = Vec::new();

        for row in rows{
            let (id, id_permission, _book, name, surname, _pass, email): (i32, i32, i32, String, String, String, String) = mysql::from_row(row);

            let permissions = rt.block_on(db.search_by(&"permissions".to_string(), (Some(&"id".to_string()), Some(&id_permission.to_string())))).unwrap();
            
            let mut level_permission: String = String::new();

            for permission in permissions{
                let (_, level):(i32, String) = mysql::from_row(permission);
                level_permission = level;
            }

            let librarian: User = User { 
                email: email,
                id: id,
                permission: level_permission,
                book_allocated: "None".to_string(),
                name: name,
                surname: surname
            };

            librarians.push(librarian);
        }
        
        Ok(librarians)
    }
    fn order_user_by(&self, column: &str) -> Result<Vec<User>, Box<dyn Error>>{
        if !(to_int(self.permission.clone()).unwrap() <= 1){
            return Err("Permission denied".into());
        }
            let mut users = self.search_user().unwrap();
    
            match column {
                "name" => users.sort_by(|a, b| a.name.cmp(&b.name)),
                "surname" => users.sort_by(|a, b| a.surname.cmp(&b.surname)),
                "id" => users.sort_by(|a, b| a.id.cmp(&b.id)),
                "email" => users.sort_by(|a, b| a.email.cmp(&b.email)),
                _ => return Err("Column not found".into())
            }
    
            Ok(users)
    }
    fn delete(&self, id: String, table: String) -> Result<(), DataBaseError>{
        if !(to_int(self.permission.clone()).unwrap() <= 1){
            return Err(DataBaseError::InvalidPermission("Permission denied".to_string()));
        }
        
        let db = self.connect_db();
        let rt = runtime::Runtime::new().unwrap();

        rt.block_on(db.delete_by("id".to_string(), id, table)).unwrap();

        Ok(())
    }
    fn edit(&self,column: String, id: String, new_value: String, table: String) -> Result<(), DataBaseError> {
        if !(to_int(self.permission.clone()).unwrap() <= 1){
            return Err(DataBaseError::InvalidPermission("Permission denied".to_string()));
        }

        let db = self.connect_db();
        let rt = runtime::Runtime::new().unwrap();


        rt.block_on(db.edit_by(column, id, new_value, table)).unwrap();

        Ok(())
    }
}




#[derive(FromForm)]
pub struct LoginForm{
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct BookJS{
    pub title: String,
    pub author: String,
    pub pub_company: String,
    pub category: String,
    pub edition: String,
    pub publishing: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct UserJS{
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub permission: String
}