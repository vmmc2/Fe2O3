# Observacoes e notas a respeito de Rust.

* Nao pode usar a keyword 'let' para declarar uma variavel fora de uma funcao. Se for usar essa keyword, tem que ser obrigatoriamente 
dentro de uma funcao.
* Quando estivermos trabalhando com Rust e temos uma divisao de dois numeros inteiros e queremos que o resultado seja em floating-point, devemos indicar que os dois numeros em questao que estao envolvidos na divisao sao numeros do tipo floating-point. 
Exemplo: Se queremos dividir 9 por 5 com o intuito de obter 1.8 como resultado, devemos representar essa divisao da seguinte maneira: 9.0/5.0
