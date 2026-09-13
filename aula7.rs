fn main() {
    let mut x = 10;
    let mut y = 20;

    println!("x={}, y={}", x, y);

    troca(&mut x, &mut y);

    println!("x={}, y={}", x, y);
}

//:P as a placeholder in between {}

fn troca(x: &mut i32, y: &mut i32) {
    let temp = *x;
    *x = *y;
    *y = temp;
}

fn calculeOvetor([]) {

}

fn palindromo(text:&str) -> bool {

    let original = text;
    let palindromo = original.chars().rev().collect::<String>();

    if palindromo == original {
        return true;
    }
    else{
        return false;
    }

}