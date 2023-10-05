/// Módulo apenas das rotas de admin.

use rocket::http::Cookies;
use rocket::response::{content::Html, Redirect};
use tera::{Tera, Context};

use crate::router::structs::*;

#[path = "../util/create_tera.rs"]
mod create_tera;
use create_tera::*;

#[get("/home/admin")]
pub fn admin(mut cookies: Cookies) -> Result<Html<String>, Redirect>{
    let user_session_cookie = cookies.get_private("user_session");

    if let Some(user_session_cookie) = user_session_cookie {
        let user_session: User = serde_json::from_str(user_session_cookie.value()).unwrap();
        
        if user_session.permission == "1".to_string(){
            let db_librarians = user_session.search_user();
            let db_books = user_session.search_book();

            let mut context = Context::new();

            match db_books{
                Ok(books) => context.insert("books", &books),
                Err(_error_data) => ()
            }

            match db_librarians{
                Ok(librarians) => context.insert("librarians", &librarians),
                Err(_error_data) => ()
            }
        
        
            context.insert("name", &user_session.name);
            context.insert("surname", &user_session.surname);
        
            let tera = Tera::new("./src/modules/templates/view/admin/*.tera").expect("Erro ao carregar templates");
            let rendered = tera.render("home.tera", &context).expect("Erro ao renderizar template");

            Ok(Html(rendered))
        }else {
            Err(Redirect::to("/"))
        }
    } else {
        Err(Redirect::to("/"))
    }
}

#[get("/home/admin/librarians")]
pub fn get_librarians_admin(mut cookies: Cookies) -> Result<Html<String>, Redirect>{
    let user_session_cookie = cookies.get_private("user_session");

    if let Some(user_session_cookie) = user_session_cookie {
        let user_session: User = serde_json::from_str(user_session_cookie.value()).unwrap();
        
        if user_session.permission == "1".to_string(){
            let db_librarians = user_session.search_user();

            let mut context = Context::new();

            match db_librarians{
                Ok(librarians) => context.insert("librarians", &librarians),
                Err(_error_data) => ()
            }
        
            context.insert("name", &user_session.name);
            context.insert("surname", &user_session.surname);
        
            let tera = create();
            let rendered = tera.render("librarians.tera", &context).expect("Erro ao renderizar template");

            Ok(Html(rendered))
        }else {
            Err(Redirect::to("/"))
        }
    } else {
        Err(Redirect::to("/"))
    }
}

#[get("/home/admin/books")]
pub fn get_books_admin(mut cookies: Cookies) -> Result<Html<String>, Redirect>{
    let user_session_cookie = cookies.get_private("user_session");

    if let Some(user_session_cookie) = user_session_cookie {
        let user_session: User = serde_json::from_str(user_session_cookie.value()).unwrap();
        
        if user_session.permission == "1".to_string(){
            let db_books = user_session.search_book();

            let mut context = Context::new();

            match db_books{
                Ok(books) => context.insert("books", &books),
                Err(_error_data) => ()
            }
            context.insert("name", &user_session.name);
            context.insert("surname", &user_session.surname);
        
            let tera = create();
            let rendered = tera.render("books.tera", &context).expect("Erro ao renderizar template");

            Ok(Html(rendered))
        }else {
            Err(Redirect::to("/"))
        }
    } else {
        Err(Redirect::to("/"))
    }
}
