
<!-- Recebe dados do novo funcionário para adicionar e efetua put -->
document.getElementById("put-form-admin").addEventListener("submit", function (event) {
    event.preventDefault(); // Impede o envio padrão do formulário

    // Obtém os dados do formulário
    const formData = new FormData(event.target);

    const UserJS = {
        name: formData.get("name"),
        surname: formData.get("surname"),
        email: formData.get("email"),
        password: formData.get("password"),
        confirm_password: formData.get("confirm_password"),
        permission: formData.get("permission")
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
            location.reload()
        })
        .catch((error) => {
           console.log("aba")
        });

});

<!-- Recebe dados do funcionário para editar e efetua put -->
window.addEventListener('DOMContentLoaded', function () {
    const editButtons = document.querySelectorAll('.edit-button');

    editButtons.forEach(function (button) {
        button.addEventListener('click', function (event) {
            const librarianData = JSON.parse(this.getAttribute('data-librarian'));
            
            librarianData.password = ""
            librarianData.confirm_password = ""

            let column = prompt("Qual das duas colunas quer mudar? Email ou Permissões", "Ex.: Email")
            if (column == null){
                return 0;

            }   

            let new_value = prompt("Qual será o novo valor?")
            if (new_value == null || new_value == ""){
                return 0;
                
            }      

            if (column == "Email"){
                librarianData.email = new_value;

            }else if (column == "Nível de permissão"){
                librarianData.permission = new_value;
                
            }else if (column == "ID") {
                alert("Coluna inválida para alteração!")
                return 0;

            }
            else{
                alert("Coluna inválida!\n Verifique se inseriu corretamente a coluna que deseja alterar.")
                return 0;

            }

            
            fetch("/edit_librarian", {
                    method: "PUT",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify(librarianData)
                })
                    .then((response) => {
                        if (response.ok){
                            alert(column+" alterado com sucesso!")
                            location.reload()
                        }
                        else{
                            console.log(response)
                            alert("Erro"+response.status+"\nValor inválido!"+column+" inalterado.")
                        }
                    })
                    .catch((error) => {
                        console.log(error)
                    });
        });

    });
});

<!-- Recebe dados do funcionário para excluir e efetua put -->
window.addEventListener('DOMContentLoaded', function () {
    const editButtons = document.querySelectorAll('.clear-button');

    editButtons.forEach(function (button) {
        button.addEventListener('click', function (event) {
            const librarianData = JSON.parse(this.getAttribute('data-librarian'));

            var choice = confirm("Você tem certeza que deseja excluir "+ librarianData.title +"?")
            
            if(choice) {
                fetch("/delete_librarian", {
                        method: "PUT",
                        headers: {
                            "Content-Type": "application/json"
                        },
                        body: JSON.stringify(librarianData)
                    })
                        .then((response) => {
                            if (response.ok){
                                location.reload()
                            }
                            else{
                                console.log(response)
                                alert("Erro"+response.status+"\n Falha ao excluir o livro.")
                            }
                        })
                        .catch((error) => {
                            console.log(error)
                        });

            }
            
        });

    });
});