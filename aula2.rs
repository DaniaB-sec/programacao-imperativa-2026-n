use std::io;

fn main() {

    let mut x = 5;
    let y: i32 = 11;

    x = "Ola";
}

let mut nome: String = String::new();
println!("Digite seu nome: ");

io::stdin()
.read_line(&mut nome)
.expect("Failed to read line");

println!("O seu nome e {nome}");

let mut x = 5;

println!("O valor de x e {x}");

x = 5 * 2;
println!("Agora o valor de x e {x}");