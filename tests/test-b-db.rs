#[path = "../src/modules/db/db.rs"]
mod db;
use db::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_author() {
        let db = DataBase::new();
        let result = db.new_author(&String::from("test")).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_new_publisher() {
        let db = DataBase::new();
        let result = db.new_publisher(&String::from("test")).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_has_user() {
        let db = DataBase::new();
        let result = db.has_user(&String::from("test@example.com"));
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_all_books() {
        let db = DataBase::new();
        let result = db.search_by(&"books".to_string(), (None, None)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_by_book() {
        let db = DataBase::new();
        let result = db.search_by(&"books".to_string(), (Some(&"id".to_string()), Some(&"1".to_string()))).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_by_author() {
        let db = DataBase::new();
        let result = db.search_by(&"author".to_string(), (Some(&"name".to_string()), Some(&"test".to_string()))).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_by_company() {
        let db = DataBase::new();
        let result = db.search_by(&"publishing_company".to_string(), (Some(&"name".to_string()), Some(&"test".to_string()))).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_by_user() {
        let db = DataBase::new();
        let result = db.search_by(&"users".to_string(), (Some(&"email".to_string()), Some(&"17eduardo05@gmail.com".to_string()))).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_new_book() {
        let db = DataBase::new();
        let book_info = (
            String::from("test"),
            String::from("test"),
            String::from("teste"),
            String::from("xxxx-xx-xx"),
            String::from("test"),
            String::from("test"),
            String::from("test"),
        );
        let result = db.new_book(book_info).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_new_user() {
        let db = DataBase::new();
        let user_info = (
            String::from("test"),
            String::from("test"),
            String::from("test@example.com"),
            String::from("test123"),
        );
        let result = db.new_user(user_info, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_order_by() {
        // Inicialize sua conexão com o banco de dados aqui, crie um pool de teste, etc.
        let db = DataBase::new();
        
        // Teste com ordem ascendente
        let result_asc = db.order_by(&"books".to_string(), &"title".to_string(), Some(true)).await;
        assert!(result_asc.is_ok());
    
        // Teste com ordem descendente
        let result_desc = db.order_by(&"books".to_string(), &"title".to_string(), Some(false)).await;
        assert!(result_desc.is_ok());
    
        // Teste com ordem ascendente padrão
        let result_default = db.order_by(&"books".to_string(), &"title".to_string(), None).await;
        assert!(result_default.is_ok());

        // Teste com tabela errada
        let result_table = db.order_by(&"t".to_string(), &"title".to_string(), None).await;
        assert!(result_table.is_err());

        // Teste com coluna errada
        let result_col = db.order_by(&"books".to_string(), &"name".to_string(), None).await;
        assert!(result_col.is_err());
    }

    #[tokio::test]
    async fn test_edit_by() {
        // Inicialize sua conexão com o banco de dados aqui, crie um pool de teste, etc.
        let db = DataBase::new();
        
        // Teste com edição de quantidade
        let result_edit = db.edit_by("qnt_specimens".to_string(), "1".to_string(), "2".to_string(), "books".to_string()).await;
        assert!(result_edit.is_ok());
    
        // Teste com edição de quantidade
        let result_edit = db.edit_by("qnt_specimens".to_string(), "1".to_string(), "1".to_string(), "books".to_string()).await;
        assert!(result_edit.is_ok());
    
        // Teste com dados errados
        let result_edit = db.edit_by("id_author".to_string(), "title".to_string(), "2".to_string(), "books".to_string()).await;
        assert!(result_edit.is_err());

        // Teste com dados errados
        let result_edit = db.edit_by("1".to_string(), "1".to_string(), "2".to_string(), "books".to_string()).await;
        assert!(result_edit.is_err());

        // Teste com dados errados
        let result_edit = db.edit_by("title".to_string(), "1".to_string(), "2".to_string(), "ken".to_string()).await;
        assert!(result_edit.is_err());
    }

}
