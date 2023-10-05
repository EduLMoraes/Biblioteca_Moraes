#[path = "../src/modules/controller/login.rs"]
mod login;
use login::*;
#[path = "../src/modules/controller/structs.rs"]
mod structs;
use structs::*;

#[cfg(test)]
mod tests {
    use super::*;

    // Teste para a função `User::new`
    #[test]
    fn test_user_new() {
        let data_user = vec![
            String::from("email@example.com"),
            String::from("1"),
            String::from("3"),
            String::from("3"),
            String::from("name"),
            String::from("surname"),
        ];

        let user = User::new(data_user);

        assert_eq!(user.email, "email@example.com");
        assert_eq!(user.id, 1);
        assert_eq!(user.permission, "3");
        assert_eq!(user.book_allocated, "3");
        assert_eq!(user.name, "name");
        assert_eq!(user.surname, "surname");
    }

    // Teste para a função `login`
    #[tokio::test]
    async fn test_login() {
        // Suponha que você tenha configurado um banco de dados de teste aqui

        // Teste com usuário correto
        let user = login(String::from("17eduardo05@gmail.com"), String::from("0129")).await;
        assert!(user.is_ok());

        // Teste com senha incorreta
        let user = login(String::from("17eduardo05@gmail.com"), String::from("wrong_password")).await;
        assert!(user.is_err());

        // Teste com usuário não encontrado
        let user = login(String::from("non_existent_email@gmail.com"), String::from("0129")).await;
        assert!(user.is_err());
    }

    // Teste para a função `new_user`
    #[tokio::test]
    async fn test_new_user() {
        // Suponha que você tenha configurado um banco de dados de teste aqui

        // Teste com senhas iguais e resposta bem-sucedida
        let result = new_user((
            String::from("test"),
            String::from("Surname"),
            String::from("test@gmail.com"),
            String::from("password"),
            String::from("password"),
        ), None)
        .await;
        assert!(result.is_ok());

        // Teste com senhas diferentes
        let result = new_user((
            String::from("test"),
            String::from("Surname"),
            String::from("test@gmail.com"),
            String::from("password"),
            String::from("different_password"),
        ), None)
        .await;
        assert!(result.is_err());

        // Teste com email inválido
        let result = new_user((
            String::from("test"),
            String::from("Surname"),
            String::from("invalid_email"),
            String::from("password"),
            String::from("password"),
        ), None)
        .await;
        assert!(result.is_err());

        // Teste com usuário já existente
        let result = new_user((
            String::from("test"),
            String::from("Surname"),
            String::from("17eduardo05@gmail.com"),
            String::from("password"),
            String::from("password"),
        ), None)
        .await;
        assert!(result.is_err());
    }
}
