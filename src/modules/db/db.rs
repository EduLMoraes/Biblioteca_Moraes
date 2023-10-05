use mysql::{*, prelude::Queryable};
use thiserror::Error;

#[allow(dead_code)]
pub struct DataBase{
    pub pool: Pool,
}
#[allow(dead_code)]
impl  DataBase {
    pub fn new() -> DataBase {
        DataBase {pool: Pool::new("mysql://automato:0178@app-db:3306/study").unwrap() }
    }

    pub fn has_user(&self, email: &String) -> Result<bool, DataBaseError>{
        let mut conn = self.pool.get_conn()?;

        let stmt = conn.prep("SELECT email FROM users WHERE email = ?")?;
        let response: Vec<Row> = conn.exec(stmt, (email,))?;
        let mut email_exists = String::new();
        for row in response{
            email_exists = mysql::from_row(row);
        }

        Ok(email_exists == "".to_string())
    }
    pub async fn new_author(&self, name: &String) -> Result<(), DataBaseError>{
        let mut conn = self.pool.get_conn()?;

        let stmt = conn.prep("INSERT INTO author (name)VALUES (?)")?;
        conn.exec::<String, Statement, _>(stmt, (name,))?;

        Ok(())
    }
    pub async fn new_publisher(&self, name: &String) -> Result<(), DataBaseError>{
        let mut conn = self.pool.get_conn()?;

        let stmt = conn.prep("INSERT INTO publishing_company (name) VALUES (?)")?;
        conn.exec::<String, Statement, _>(stmt, (name,))?;

        Ok(())
    }
    
    pub async fn new_book(&self, (title, cat, ed, pub_date, desc, author, pub_company):(String, String, String, String, String, String, String)) -> Result<(), DataBaseError>{
        let mut conn = self.pool.get_conn()?;

        let mut _trash = String::new();
        
        let mut authors = self.search_by(&"author".to_string(), (Some(&"name".to_string()), Some(&author))).await?;
        let mut id_author: i32 = 0;
        
        if authors == []{
            self.new_author(&author).await?;
            authors = self.search_by(&"author".to_string(), (Some(&"name".to_string()), Some(&author))).await?;
        }
        

        for author in authors{
            (id_author, _trash) = mysql::from_row(author);
        }
        
        
        let mut pub_companies = self.search_by(&"publishing_company".to_string(), (Some(&"name".to_string()), Some(&pub_company))).await?;
        let mut id_company: i32 = 0;
        

        if pub_companies == []{
            self.new_publisher(&pub_company).await?;
            pub_companies = self.search_by(&"publishing_company".to_string(), (Some(&"name".to_string()), Some(&pub_company))).await?;
        }
        

        for company in pub_companies{
            (id_company, _trash) = mysql::from_row(company);
        }
        

        let mut new_book = Vec::new();
        new_book.push((&id_company, &id_author, &title, &cat, &ed, &pub_date, &desc));
        

        let mut stmt: Statement;

        let books = self.search_by(&"books".to_string(), (Some(&"title".to_string()), Some(&title))).await?;
        let mut has_book: bool = false;

        for book in books{
            let (id , _id_company, _id_author, _title, _cat, _ed, _pub_date, _desc, qnt):
            (i32, i32, i32, String, String, String, String, String, i32) = mysql::from_row(book);
            stmt = conn.prep("UPDATE 
                books SET qnt_specimens = ? WHERE id = ?")?;

            conn.exec::<(i32, i32), Statement, _>(stmt, (qnt+1, id))?;

            has_book = true;
        }
        
        if !has_book{
            stmt = conn.prep("INSERT INTO 
            books (id_publishing_company, id_author, title, category, edition, publish_date, description, qnt_specimens)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)")?;
            
            conn.exec::<(i32, i32, String, String, String, String, String, i32), Statement, _>(stmt, (id_company, id_author, title, cat, ed, pub_date, desc, 1))?;
        }

        Ok(())
    }
    pub async fn new_user(&self, (name, surname, email, password):(String, String, String, String), permission: Option<String>) -> Result<(), DataBaseError>{
        let mut conn = self.pool.get_conn()?;

        let mut is_valid_email = false;

        for letter in email.chars(){
            if letter == '@'{
                is_valid_email = true;
                break;
            }
        }

        if !is_valid_email{
            return Err(DataBaseError::EmailInvalid("Error: Invalid email".into()))
        }else if !self.has_user(&email)?{
            return Err(DataBaseError::EmailInvalid("Error: Email already exists".into()))
        }else{
            let mut stmt: Statement = conn.prep("INSERT INTO 
                users (name, surname, password, email, id_permission)
                VALUES (?, ?, ?, ?, 3)")?;

            match permission{
                Some(perm) => {

                    stmt = conn.prep("INSERT INTO 
                        users (name, surname, password, email, id_permission)
                        VALUES (?, ?, ?, ?, ?)")?;
                    
                    let mut level = 3;

                    if perm == "admin".to_string(){
                        level = 1;
                    } else if perm == "librarian".to_string(){
                        level = 2;
                    }

                    conn.exec::<(String, String, String, String, i32), Statement, _>(stmt, (name, surname, password, email, level))?;
                    Ok(())
            },
                None => {
                    conn.exec::<(String, String, String, String, i32), Statement, _>(stmt, (name, surname, password, email))?;
                    Ok(())
                },
            }

        }
    }  
    pub async fn search_by(&self, table: &String, (column, value): (Option<&String>, Option<&String>)) -> Result<Vec<Row>, DataBaseError>{
        let mut conn = self.pool.get_conn()?;

        let rows: Vec<Row>;
        let stmt: Statement;

        match column {
            Some(column) => {
                match value {
                    Some(value) => {
                        stmt = conn.prep(format!("SELECT * FROM {} WHERE {} = (?)", table, column))?;
                        rows = conn.exec(stmt, (value,))?;
                    }
                    None => {
                        return Err(DataBaseError::ValueInvalid("Need value for column".into()));
                    }
                }
            }
            None =>{
                stmt = conn.prep(format!("SELECT * FROM {}", table))?;
                rows = conn.exec(stmt, ())?;
            }
        }
        
        Ok(rows)
    } 
    pub async fn order_by(&self, table: &String, column: &String, asc: Option<bool>) -> Result<Vec<Row>, DataBaseError>{
        let mut conn = self.pool.get_conn()?;

        let ascendent: String;

        match asc {
            Some(response) =>{
                if response {
                    ascendent = "ASC".to_string();
                }else{
                    ascendent = "DESC".to_string();
                }
            }
            None =>{
                ascendent = "ASC".to_string();
            }
        }

        let stmt = conn.prep(format!("SELECT * FROM {} ORDER BY {} {}", table, column, ascendent))?;
        let rows: Vec<Row> = conn.exec(stmt, ())?;

        Ok(rows)
    }
    pub async fn delete_by(&self, column: String, value: String, table: String) -> Result<(), DataBaseError>{
        let mut conn = self.pool.get_conn()?;
    
        let stmt = conn.prep(&format!("DELETE FROM {} WHERE {} = ?", table, column))?;
    
        if column == "id".to_string(){
            let value: i32 = value.trim().parse().unwrap();

            if self.search_by(&table, (Some(&column), Some(&value.to_string()))).await? == []{
                return Err(DataBaseError::ValueInvalid(("ID value invalid").to_string()))
            }

            conn.exec::<(i32, String, String), Statement, _>(stmt, (value,))?;
        }
        else{
            conn.exec::<(i32, String, String), Statement, _>(stmt, (value,))?;
        }
    
        Ok(())
    }
    pub async fn edit_by(&self, column: String, id: String, value: String, table: String) -> Result<(), DataBaseError>{
        let mut conn = self.pool.get_conn()?;

        if self.search_by(&table, (Some(&"id".to_string()), Some(&id))).await? == []{
            return Err(DataBaseError::ValueInvalid(("ID value invalid").to_string()))
        }
        
        let id: i32 = id.trim().parse().unwrap();
        let stmt = conn.prep(&format!("UPDATE {} SET {} = ? WHERE id = ?", table, column))?;
        
        if column == "qnt_specimens".to_string(){
            let value: i32 = value.trim().parse().unwrap();
            conn.exec::<(i32, String, String), Statement, _>(stmt, (value, id))?;
        } 
        else if column == "id_author".to_string(){

            let mut author = self.search_by(&"author".to_string(), (Some(&"name".to_string()), Some(&value))).await?;

            if author.is_empty(){
                self.new_author(&value).await?;
                author = self.search_by(&"author".to_string(), (Some(&"name".to_string()), Some(&value))).await?;
            }

            let mut id_author: i32 = 0;
            let mut _trash: String;

            for data in author{
                (id_author, _trash) = mysql::from_row(data);
            }

            conn.exec::<(i32, String, String), Statement, _>(stmt, (id_author, id))?;
        }
        else if column == "id_publishing_company".to_string(){

            let mut publishing_company = self.search_by(&"publishing_company".to_string(), (Some(&"name".to_string()), Some(&value))).await?;

            if publishing_company.is_empty(){
                self.new_publisher(&value).await?;
                publishing_company = self.search_by(&"publishing_company".to_string(), (Some(&"name".to_string()), Some(&value))).await?;
            }

            let mut id_publishing_company: i32 = 0;
            let mut _trash: String;

            for data in publishing_company{
                (id_publishing_company, _trash) = mysql::from_row(data);
            }

            conn.exec::<(i32, String, String), Statement, _>(stmt, (id_publishing_company, id))?;
        }
        else if column == "id_permission".to_string(){

            let mut permissions = self.search_by(&"permissions".to_string(), (Some(&"level".to_string()), Some(&value))).await?;

            if permissions.is_empty(){
                self.new_publisher(&value).await?;
                permissions = self.search_by(&"permissions".to_string(), (Some(&"level".to_string()), Some(&value))).await?;
            }

            let mut id_permissions: i32 = 0;
            let mut _trash: String;

            for data in permissions{
                (id_permissions, _trash) = mysql::from_row(data);
            }

            conn.exec::<(i32, String, String), Statement, _>(stmt, (id_permissions, id))?;
        }
        else{
            conn.exec::<(i32, String, String), Statement, _>(stmt, (value, id))?;
        }
    
        Ok(())
    }
}


#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum DataBaseError{
    #[error("Email is invalid: {0}")]
    ConnectionFailed(String),

    #[error("Query Error: {0}")]
    QueryError(mysql::Error),

    #[error("Email is invalid: {0}")]
    EmailInvalid(String),

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Value is invalid: {0}")]
    ValueInvalid(String),

    #[error("Permission denied: {0}")]
    InvalidPermission(String),
}


impl From<String> for DataBaseError {
    fn from(v: String) -> Self {
        Self::ConnectionFailed (v)
    }
}
impl From<mysql::Error> for DataBaseError {
    fn from(err: mysql::Error) -> Self{
        DataBaseError::QueryError(err)
    }
}
