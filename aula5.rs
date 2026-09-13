
mod calculadora;
use std::io;
use crate::calculadora::operacoes::{adi, div, multi, sub};
use calculadora::menu::menu1;

fn main(){

    loop {

        menu1();

        let mut escolha: String = String::new();
        io::stdin().read_line(&mut escolha).expect("Erro");

        let mut escolha_quebra = escolha.trim().to_string();

        println!("Escolha seu primeiro numero: ");

        let mut a: String = String::new();
        io::stdin().read_line(&mut a).expect("erro");
        let a: i32 = a.trim().parse().expect("Erro");

        println!("Escolha seu segundo numero: ");

        let mut b: String = String::new();
        io::stdin().read_line(&mut b).expect("Erro");
        let b: i32 = b.trim().parse().expect("erro");

        if escolha_quebra == "1" {
            adi(a, b);
        } else if escolha_quebra == "2" {
            sub(a, b);
        } else if escolha_quebra == "3" {
            div(a, b);
        } else if escolha_quebra == "4" {
            multi(a,b);
        } else if escolha_quebra == "5" {
            break;

        }

    }



}
