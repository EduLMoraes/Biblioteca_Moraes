#[path = "../src/modules/controller/structs.rs"]
mod structs;
use rocket_contrib::json::Json;

#[cfg(test)]
mod tests {
   
    use structs::*;
    use super::*;

    #[test]
    fn test_search_book() {
        // Crie um usuário de exemplo para os testes
        let user = User::new(vec![
            "exemple@gmail.com".to_string(),
            "1".to_string(),
            "1".to_string(),
            "0".to_string(),
            "Exemplar".to_string(),
            "Almeida".to_string(),
        ]);

        // Realize uma pesquisa de livros
        let result = user.search_book();

        // Verifique se a pesquisa foi bem-sucedida
        assert!(result.is_ok());

        // Obtenha os livros do resultado
        let books = result.unwrap();

        // Verifique se a lista de livros não está vazia
        assert!(!books.is_empty());

        // Verifique se o título do primeiro livro é o esperado
        assert_eq!(books[0].title, "As Armadilhas da Mente".to_string());
    }

    #[test]
    fn test_order_book_by() {
        // Crie um usuário de exemplo para os testes
        let user = User::new(vec![
            "exemple@gmail.com".to_string(),
            "1".to_string(),
            "1".to_string(),
            "0".to_string(),
            "Exemplar".to_string(),
            "Almeida".to_string(),
        ]);

        // Ordene os livros por título
        let result = user.order_book_by("title");

        // Verifique se a ordenação foi bem-sucedida
        assert!(result.is_ok());

        // Obtenha os livros ordenados do resultado
        let books = result.unwrap();

        // Verifique se a lista de livros não está vazia
        assert!(!books.is_empty());

        // Verifique se os livros estão ordenados por título
        for i in 1..books.len() {   
            assert!(books[i - 1].title <= books[i].title);
        }
    }

    #[test]
    fn test_permission_user() {
        // Crie um usuário de exemplo para os testes
        let user = User::new(vec![
            "exemple@gmail.com".to_string(),
            "999".to_string(),
            "3".to_string(),
            "0".to_string(),
            "Exemplar".to_string(),
            "Almeida".to_string(),
        ]);

        let book_js = BookJS { 
            pub_company: "test".to_string(), 
            author: "test".to_string(), 
            title: "test".to_string(), 
            category: "test".to_string(), 
            edition: "test".to_string(), 
            publishing: "test".to_string(), 
            description: "test".to_string(), 
        };

        
        let result = user.add_book(Json(book_js));
        assert!(result.is_err());
        
        let result = user.edit_qnt(1.to_string(), 2.to_string());
        assert!(result.is_err());
        
        let result = user.search_user();
        assert!(result.is_err());
        
        let result = user.order_user_by("name");
        assert!(result.is_err());
        
        let result = user.edit("qnt_specimens".to_string(), "1".to_string(), "5".to_string(), "books".to_string());
        assert!(result.is_err());

        let result = user.delete("1".to_string(), "books".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_permission_librarian() {
        // Crie um usuário de exemplo para os testes
        let user = User::new(vec![
            "exemple@gmail.com".to_string(),
            "999".to_string(),
            "2".to_string(),
            "0".to_string(),
            "Exemplar".to_string(),
            "Almeida".to_string(),
        ]);
        let book_js = BookJS { 
            pub_company: "test".to_string(), 
            author: "test".to_string(), 
            title: "test".to_string(), 
            category: "test".to_string(), 
            edition: "test".to_string(), 
            publishing: "test".to_string(), 
            description: "test".to_string(), 
        };
        
        let result = user.add_book(Json(book_js));
        assert!(result.is_ok());
        
        let result = user.edit_qnt(1.to_string(), 2.to_string());
        assert!(result.is_ok());

        let result = user.edit_qnt(1.to_string(), 1.to_string());
        assert!(result.is_ok());
        
        let result = user.search_user();
        assert!(result.is_err());
        
        let result = user.order_user_by("name");
        assert!(result.is_err());
        
        let result = user.edit("qnt_specimens".to_string(), "1".to_string(), "5".to_string(), "books".to_string());
        assert!(result.is_err());

        let result = user.delete("1".to_string(), "books".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_permission_admin() {
        // Crie um usuário de exemplo para os testes
        let user = User::new(vec![
            "exemple@gmail.com".to_string(),
            "999".to_string(),
            "1".to_string(),
            "0".to_string(),
            "Exemplar".to_string(),
            "Almeida".to_string(),
        ]);

        let book_js = BookJS { 
            pub_company: "test".to_string(), 
            author: "test".to_string(), 
            title: "test".to_string(), 
            category: "test".to_string(), 
            edition: "test".to_string(), 
            publishing: "test".to_string(), 
            description: "test".to_string(), 
        };
        
        let result = user.add_book(Json(book_js));
        assert!(result.is_ok());
        
        let result = user.edit_qnt(1.to_string(), 2.to_string());
        assert!(result.is_ok());

        let result = user.edit_qnt(1.to_string(), 1.to_string());
        assert!(result.is_ok());
        
        let result = user.search_user();
        assert!(result.is_ok());
        
        let result = user.order_user_by("name");
        assert!(result.is_ok());
        
        let result = user.edit("qnt_specimens".to_string(), "1".to_string(), "5".to_string(), "books".to_string());
        assert!(result.is_ok());

        let result = user.edit("qnt_specimens".to_string(), "1".to_string(), "1".to_string(), "books".to_string());
        assert!(result.is_ok());
    }


}
