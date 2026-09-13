

use std::io;

fn main() {
    // println!("Digite o ano de nascimento:");
    //
    // let mut ano_string: String = String::new();
    // io::stdin().read_line(&mut ano_string).expect("Failed to read line");
    //
    // let ano_sem_quebra_de_linha = ano_string.trim().to_string();
    //
    // let ano_u16: u16 = ano_sem_quebra_de_linha.parse().expect("Erro ao converter");
    //
    // println!("Voce tem {} anos de idade", 2026 - ano_u16);

    exercicio5();
    exercicio4();

}

fn exercicio4() {

    println!("Quantos anos voce tem?");

    let mut idade: String = String::new();
    io::stdin().read_line(&mut idade).expect("Failed to read line");

    let idade_sem_quebra = idade.trim().to_string();

    let idade_u8: u8 = idade_sem_quebra.parse().expect("Erro ao converter");

    if idade_u8 > 18 {
        println!("Maior de idade!")
    } else {
        println!("Menor de idade!")
    }

}

// fn exercicio2() {
//     println!("Quantos dias voce quer alugar?:");
//
//     let mut dias_carro: String = String::new();
//     io::stdin().read_line(&mut dias_carro).expect("Failed to read line");
//     // io::stdin() pega a leitura do computador. read_line(&mut) espera o usuário digitar o texto e apertar Enter.
//     //&mut dias_carro: Diz ao programa onde salvar o texto digitado.
//     //.expect("Failed to read line"): É um aviso de segurança para erros.
//     // Se a leitura falhar (como o console travar), o programa fecha e mostra essa mensagem.
//
//     let dias_carro_sem_quebra = dias_carro.trim().to_string();
//
//     let carro_u16: u16 = dias_carro_sem_quebra.parse().expect("Erro ao converter");
//
//     let valor_total: f32 = 100.0 * carro_u16 as f32; // type casting
//     // Pesquisar sobre linha 31 depois
//
//     println!("Voce tem que pagar {} reais", valor_total);
// }
//
// fn exercicio3() {
//
//     println!("Qual a temperatura?: ");
//
//     // : declara tipo
//
//     // :: acesso componentes de modulos (std::io)
//
//     // :: referencia tipo fenerico de funcoes
//
//     //Acesso de membros de estrutura
//
//     let mut temperatura: String = String::new();
//     io::stdin().read_line(&mut temperatura).expect("Failed to read line");
//
//     let temperatura_sem_quebra = temperatura.trim().to_string();
//
//     let temp_f32: f32 = temperatura_sem_quebra.parse().expect("Erro ao converter");
//
//     println!("A temperatura em fahrenheit e {}", (temp_f32 * 1.8) + 32.0);
//
// }

//E, OU,ce NAO

// E: &&
// OU: ||
// NAO:

// powf(2.0) -- exponente RUST

fn exercicio5() {

    println!("Digite seu Usuario: ");
    let mut usuario: String = String::new();
    io::stdin().read_line(&mut usuario).expect("Failed to read line");
    let usuario_sem_quebra = usuario.trim().to_string();

    println!("Digite sua senha: ");
    let mut senha: String = String::new();
    io::stdin().read_line(&mut senha).expect("Failed to read line");
    let senha_sem_quebra = senha.trim().to_string();

    if usuario_sem_quebra == "admin" && senha_sem_quebra == "@admin"{
        println!("Acesso Permitido")
    } else {
        println!("Acesso Negado")
    }

}
