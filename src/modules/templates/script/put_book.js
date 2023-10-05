<!-- Recebe dados do novo livro para adicionar e efetua put -->
document.getElementById("put-form").addEventListener("submit", function (event) {
    event.preventDefault(); // Impede o envio padrão do formulário

    // Obtém os dados do formulário
    const formData = new FormData(event.target);

    const BookJS = {
        title: formData.get("title"),
        author: formData.get("author"),
        pub_company: formData.get("pub_company"),
        category: formData.get("category"),
        edition: formData.get("edition"),
        publishing: formData.get("publishing"),
        description: formData.get("description")
    }

    // Realiza uma solicitação PUT usando XMLHttpRequest ou Fetch API
    fetch("/add_book", {
        method: "PUT",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(BookJS)
    })
        .then((response) => {
            console.log(response)
            location.reload();
        })
        .catch((error) => {
           console.log("aba")
        });

});

<!-- Recebe dados do livro para editar e efetua put -->
window.addEventListener('DOMContentLoaded', function () {
    const editButtons = document.querySelectorAll('.edit-button');

    editButtons.forEach(function (button) {
        button.addEventListener('click', function (event) {
            const bookData = JSON.parse(this.getAttribute('data-book'));
           
            let column = prompt("Qual coluna deseja alterar?", "Ex.: Título")
            if (column == null){
                return 0;

            }   

            let new_value = prompt("Qual será o novo valor?")
            if (new_value == null || new_value == ""){
                return 0;
                
            }      

            if (column == "Título"){
                bookData.title = new_value;

            }else if (column == "Autor"){
                bookData.author = new_value;
                
            }else if (column == "Categoria"){
                bookData.category = new_value;

            }else if (column == "Edição"){
                bookData.edition = new_value;

            }else if (column == "Lançamento"){
                bookData.date = new_value;

            }else if (column == "Descrição"){
                bookData.description = new_value;

            }else if (column == "Quantidade disponível"){
                bookData.qnt = new_value;

            }else if (column == "Editora"){
                bookData.publisher = new_value;

            }else if (column == "ID") {
                alert("Coluna inválida para alteração!")
                return 0;

            }
            else{
                alert("Coluna inválida!\n Verifique se inseriu corretamente a coluna que deseja alterar.")
                return 0;

            }

            
            fetch("/edit_book", {
                    method: "PUT",
                    headers: {
                        "Content-Type": "application/json"
                    },
                    body: JSON.stringify(bookData)
                })
                    .then((response) => {
                        if (response.ok){
                            alert("Livro alterado com sucesso!")
                            location.reload()
                        }
                        else{
                            console.log(response)
                            alert("Erro"+response.status+"\nQuantidade inválida! Livro inalterado.")
                        }
                    })
                    .catch((error) => {
                        console.log(error)
                    });
        });

    });
});

<!-- Recebe dados do livro para excluir e efetua put -->
window.addEventListener('DOMContentLoaded', function () {
    const editButtons = document.querySelectorAll('.clear-button');

    editButtons.forEach(function (button) {
        button.addEventListener('click', function (event) {
            const bookData = JSON.parse(this.getAttribute('data-book'));

            var choice = confirm("Você tem certeza que deseja excluir "+ bookData.title +"?")
            
            if(choice) {
                fetch("/delete_book", {
                        method: "PUT",
                        headers: {
                            "Content-Type": "application/json"
                        },
                        body: JSON.stringify(bookData)
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
