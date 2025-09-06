fn eh_primo(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let limite = n / 2;
    for i in (3..=limite) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn contar_primos(limite: i32) -> i32 {
    let mut contador = 0;
    for numero in 2..=limite {
        if eh_primo(numero) {
            contador += 1;
        }
    }
    contador
}

fn main() {
    let limite = 1_000_000;
    let resultado = contar_primos(limite);
    println!("RESULT:{}", resultado);
}
