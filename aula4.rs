

fn main() {
    use std::io;

    println!("Digite a sequencia de fibonacci: ");

    let mut n_string: String = String::new();
    io::stdin().read_line(&mut n_string).expect("Failed to read line");

    let n_string_sem_quebra = n_string.trim().to_string();

    let n_i32 : i32 = n_string_sem_quebra.parse().expect("Failed to parse number");

    while n_i32 {
        print!("{} ", n_string);

    }





}


// mostre os n elementos da sequencia de fibo solicite o valor de n para o
// usuario implemente o aloritmo ultilizando o laco de repeticao while
