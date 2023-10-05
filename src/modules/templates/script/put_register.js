<!-- Recebe dados do novo usuário para adicionar e efetua put -->
document.getElementById("put-form-register").addEventListener("submit", function (event) {
    event.preventDefault(); // Impede o envio padrão do formulário

    // Obtém os dados do formulário
    const formData = new FormData(event.target);

    const UserJS = {
        name: formData.get("name"),
        surname: formData.get("surname"),
        email: formData.get("email"),
        password: formData.get("password"),
        confirm_password: formData.get("confirm_password"),
        permission: "3"
    }

    console.log(UserJS)

    // Realiza uma solicitação PUT usando XMLHttpRequest ou Fetch API
    fetch("/add_user", {
        method: "PUT",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(UserJS)
    })
        .then((response) => {
            alert("Cadastrado com sucesso!")
            window.location.replace("/");
        })
        .catch((error) => {
           console.log("aba")
        });

});

