use rocket::{Route, get, post, http::{Status, Cookies, Cookie}};
use rocket::response::{content::Html, Redirect, status};
use rocket::request::Form;
use rocket_contrib::json::Json;
use tokio::runtime;
use mysql::serde_json;
use tera::{Tera, Context};

/// Importação dos módulos, iremos utiliza-los apenas quando necessário
/// mais adiante.
mod structs;
mod login;
mod router_admin;
mod router_librarian;
mod router_user;

#[get("/")]
fn index() -> Html<String>{
    let context = Context::new();

    let tera = Tera::new("./src/modules/templates/*.tera").expect("Erro ao carregar templates");
    let rendered = tera.render("index.tera", &context).expect("Erro ao renderizar template");

    Html(rendered)
}

use structs::{User, LoginForm};
#[post("/login", data = "<form>")]
fn login(form: Form<LoginForm>, mut cookies: Cookies) -> Redirect{
    let email = &form.email;
    let password = &form.password;

    let rt = runtime::Runtime::new().unwrap();
    let login = rt.block_on(login::login(email.to_string(), password.to_string()));

    match login{
        Ok(user_session) => { 
            cookies.add_private(Cookie::new("user_session", serde_json::to_string(&user_session).unwrap()));
            Redirect::to("/home")
        },
        Err(_) => Redirect::to("/")
    }
}

#[get("/register")]
fn register() -> Html<String>{
    let context = Context::new();

    let tera = Tera::new("./src/modules/templates/*.tera").expect("Erro ao carregar templates");
    let rendered = tera.render("register.tera", &context).expect("Erro ao renderizar template");

    Html(rendered)
}

use router_admin::*;
use router_librarian::*;
use router_user::*;
#[get("/home")]
fn redirect_user(mut cookies: Cookies) -> Result<Redirect, Redirect>{
    let user_session_cookie = cookies.get_private("user_session");

    if let Some(user_session_cookie) = user_session_cookie {
        let user_session: User = serde_json::from_str(user_session_cookie.value()).unwrap();
        
        if user_session.permission == "1".to_string(){
            Ok(Redirect::to("/home/admin"))
        }else if user_session.permission == "2".to_string(){
            Ok(Redirect::to("/home/librarian"))
        }else {
            Ok(Redirect::to("/home/user"))
        }
    } else {
        // A sessão do usuário não existe, redirecione para a página de login
        Err(Redirect::to("/"))
    }
}

use structs::db::DataBaseError;
use structs::{BookJS, LibrarianPermissions};
#[put("/add_book", data = "<form>")]
    /// Essa função serve para adicionar livros
    /// Usamos o block_on do runtime para executar funções async sem a necessidade de estar em uma.
    /// 
    /// ```
    /// let rt = runtime::Runtime::new().unwrap();
    /// let response = rt.block_on(control::new_book(form));
    /// ```
    /// 
    /// Isso nos permite usar a função assíncrona new_book() do módulo control
    /// sem a necessidade de declarar a add_book como async.
    /// Porém, utilizando do mesmo modo que uma async, aguardando a resposta da função para continuar a execução.
fn add_book(mut cookies: Cookies, form: Json<BookJS>) -> status::Custom<Result<String, String>>{
    
    let cookie = cookies.get_private("user_session");
    let mut response : Result<(), DataBaseError> = Ok(());
    
    if let Some(user_session_cookie) = cookie {
        let user_session: User = serde_json::from_str(user_session_cookie.value()).unwrap();
        
        response = user_session.add_book(form);
    }

    match response {
        Ok(()) => {
            let success_response = format!("Livro adicionado com sucesso!");
            status::Custom(Status::Ok, Ok(success_response))
        },
        Err(_e) => {
            let error_response = format!("Error 807: falha ao adicionar o livro!");
            status::Custom(Status::NotModified, Err(error_response))
        }
    }
}

use structs::{Book, transform::to_int};
use self::structs::{AdminPermissions, UserPermissions};
#[put("/edit_book", data = "<book>")]
fn edit_book(mut cookies: Cookies, book: Json<Book>) -> status::Custom<Result<String, String>> {
    
    let user_cookie = cookies.get_private("user_session");
    let inteiro = to_int(book.qnt.clone());

    match inteiro{
        Ok(_quantity) => {
            let mut response: Result<(), DataBaseError> = Ok(());

            if let Some(user_session_cookie) = user_cookie{
                let user_session: User = serde_json::from_str(user_session_cookie.value()).unwrap();

                let books = user_session.search_book().unwrap();

                for old_book in books{
                    if old_book.id == book.id{
                        if old_book.title != book.title{
                            response = user_session.edit("title".to_string(), book.id.to_string(), book.title.clone(), "books".to_string())
                        
                        }else if old_book.author != book.author{
                            response = user_session.edit("id_author".to_string(), book.id.to_string(), book.author.clone(), "books".to_string())
                        
                        }else if old_book.date != book.date{
                            response = user_session.edit("publish_date".to_string(), book.id.to_string(), book.date.clone(), "books".to_string())
                        
                        }else if old_book.category != book.category{
                            response = user_session.edit("category".to_string(), book.id.to_string(), book.category.clone(), "books".to_string())
                        
                        }else if old_book.edition != book.edition{
                            response = user_session.edit("edition".to_string(), book.id.to_string(), book.edition.clone(), "books".to_string())
                        
                        }else if old_book.qnt != book.qnt{
                            response = user_session.edit("qnt_specimens".to_string(), book.id.to_string(), book.qnt.clone(), "books".to_string())
                        
                        }else if old_book.description != book.description{
                            response = user_session.edit("description".to_string(), book.id.to_string(), book.description.clone(), "books".to_string())

                        }else if old_book.publisher != book.publisher{
                            response = user_session.edit("id_publishing_company".to_string(), book.id.to_string(), book.publisher.clone(), "books".to_string())

                        }
                        break;
                    }
                }

            }

            match response {
                Ok(_) => {
                    let success_response = format!("Livro modificado com sucesso!");
                    status::Custom(Status::Ok, Ok(success_response))

                },
                Err(err) =>{
                    status::Custom(Status::NotModified, Err(err.to_string()))

                }
            }
        },
        Err(err) => {
            status::Custom(Status::NotModified, Err(err.to_string()))
        }
    }

}

#[put("/delete_book", data = "<book>")]
fn delete_book(mut cookies: Cookies, book: Json<Book>) -> status::Custom<Result<String, String>> {
    
    let user_cookie = cookies.get_private("user_session");
    let inteiro = to_int(book.qnt.clone());

    match inteiro{
        Ok(_quantity) => {
            let mut response: Result<(), DataBaseError> = Ok(());

            if let Some(user_session_cookie) = user_cookie{
                let user_session: User = serde_json::from_str(user_session_cookie.value()).unwrap();

                response = user_session.delete(book.id.to_string(), "books".to_string())
            }

            match response {
                Ok(_) => {
                    let success_response = format!("Livro deletado com sucesso!");
                    status::Custom(Status::Ok, Ok(success_response))

                },
                Err(err) =>{
                    status::Custom(Status::NotModified, Err(err.to_string()))

                }
            }
        },
        Err(err) => {
            status::Custom(Status::NotModified, Err(err.to_string()))
        }
    }

}

use structs::UserJS;
#[put("/add_user", data = "<form>")]
fn add_user(form: Json<UserJS>) -> status::Custom<Result<String, String>>{

    let rt = runtime::Runtime::new().unwrap();
    
    let register = rt.block_on(login::new_user((form.name.clone(), form.surname.clone(), form.email.clone(), form.password.clone(), form.confirm_password.clone()), Some(form.permission.clone())));

    match register{
        Ok(_) => {
            let success_response = format!("Usuário adicionado com sucesso!");
            status::Custom(Status::Ok, Ok(success_response))
        },
        Err(_e) => {
            let error_response = format!("Error 802: falha ao adicionar o usuário!");
            status::Custom(Status::NotModified, Err(error_response))
        }
    }
}

#[put("/edit_librarian", data = "<data>")]
fn edit_librarian(mut cookies: Cookies, data: Json<User>) -> status::Custom<Result<String, String>>{

    let user_cookie = cookies.get_private("user_session");
    
    let mut response: Result<(), DataBaseError> = Ok(());

    if let Some(user_session_cookie) = user_cookie{
        let user_session: User = serde_json::from_str(user_session_cookie.value()).unwrap();

        let librarians = user_session.search_user().unwrap();

        for librarian in librarians{
            if librarian.id == data.id{

                if librarian.email != data.email{
                    response = user_session.edit("email".to_string(), librarian.id.to_string(), data.email.clone(), "users".to_string());
                }else if librarian.permission != data.permission{
                    response = user_session.edit("id_permission".to_string(), librarian.id.to_string(), data.permission.clone(), "users".to_string());
                }

                break;
            }
        }
    }

    match response{
        Ok(_) => {
            let success_response = format!("Usuário editado com sucesso!");
            status::Custom(Status::Ok, Ok(success_response))
        },
        Err(_e) => {
            let error_response = format!("Error 805: falha ao editar o usuário!");
            status::Custom(Status::NotModified, Err(error_response))
        }
    }
}

#[put("/delete_librarian", data="<data>")]
fn delete_librarian(mut cookies: Cookies, data: Json<User>) -> status::Custom<Result<String, String>> {
    
    let user_cookie = cookies.get_private("user_session");

    let mut response: Result<(), DataBaseError> = Ok(());

    if let Some(user_session_cookie) = user_cookie{
        let user_session: User = serde_json::from_str(user_session_cookie.value()).unwrap();

        response = user_session.delete(data.id.to_string(), "users".to_string());
    }

    match response {
        Ok(_) => {
            let success_response = format!("Bibliotecário deletado com sucesso!");
            status::Custom(Status::Ok, Ok(success_response))

        },
        Err(err) =>{
            status::Custom(Status::NotModified, Err(err.to_string()))

        }
    }
}

#[get("/exit")]
fn user_exit(mut cookies: Cookies) -> Result<Redirect, Redirect>{
    cookies.remove_private(Cookie::named("user_session"));

    Ok(Redirect::to("/"))
}

#[get("/terms")]
fn terms() -> Html<String>{
    let context = Context::new();

    let tera = Tera::new("./src/modules/templates/*.tera").expect("Erro ao carregar templates");
    let rendered = tera.render("terms.tera", &context).expect("Erro ao renderizar template");

    Html(rendered)
}

#[get("/policy")]
fn policy() -> Html<String>{
    let context = Context::new();

    let tera = Tera::new("./src/modules/templates/*.tera").expect("Erro ao carregar templates");
    let rendered = tera.render("terms.tera", &context).expect("Erro ao renderizar template");

    Html(rendered)
}

#[get("/password")]
fn password() -> Html<String>{
    let context = Context::new();

    let tera = Tera::new("./src/modules/templates/*.tera").expect("Erro ao carregar templates");
    let rendered = tera.render("password.tera", &context).expect("Erro ao renderizar template");

    Html(rendered)
}

pub fn routes()-> Vec<Route>{
    routes![
            index, register, login, redirect_user,
            admin, librarian, user,
            user_exit, get_librarians_admin, get_books_admin,
            add_book, edit_book, delete_book, add_user, edit_librarian,
            delete_librarian, terms, policy, password
        ]
}