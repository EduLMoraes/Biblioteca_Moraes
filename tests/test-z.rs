#[path = "../src/modules/db/db.rs"]
mod db;
use db::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_delete_user_from() {
        let db = DataBase::new();
        let result = db.delete_by(String::from("name"), String::from("test"), String::from("users")).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_book_from() {
        let db = DataBase::new();
        let result = db.delete_by(String::from("title"), String::from("test"), String::from("books")).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_author_from() {
        let db = DataBase::new();
        let result = db.delete_by(String::from("name"), String::from("test"), String::from("author")).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_publishing_from() {
        let db = DataBase::new();
        let result = db.delete_by(String::from("name"), String::from("test"), String::from("publishing_company")).await;
        assert!(result.is_ok());
    }
}
