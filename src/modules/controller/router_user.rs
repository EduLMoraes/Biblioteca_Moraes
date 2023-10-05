/// Módulo de rotas do usuário comum.

use rocket::http::Cookies;
use rocket::response::{content::Html, Redirect};
use tera::{Tera, Context};

use crate::router::structs::*;

#[get("/home/user")]
pub fn user(mut cookies: Cookies) -> Result<Html<String>, Redirect>{
    let user_session_cookie = cookies.get_private("user_session");

    if let Some(user_session_cookie) = user_session_cookie {
        let user_session: User = serde_json::from_str(user_session_cookie.value()).unwrap();
        
        if user_session.permission == "3".to_string(){
            let db_books = user_session.search_book();

            let mut context = Context::new();

            match db_books{
                Ok(books) => context.insert("books", &books),
                Err(_error_data) => ()
            }
        
            context.insert("name", &user_session.name);
            context.insert("surname", &user_session.surname);
        
            let tera = Tera::new("./src/modules/templates/view/user/*.tera").expect("Erro ao carregar templates");
            let rendered = tera.render("home.tera", &context).expect("Erro ao renderizar template");

            Ok(Html(rendered))
        }else {
            Err(Redirect::to("/"))
        }
    } else {
        Err(Redirect::to("/"))
    }
}

