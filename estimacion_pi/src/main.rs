use core::f64;
use std::f64::consts::SQRT_2;
use std::io::stdin;
use std::str::FromStr;
fn factorial_especial(num: u32, k: u32) -> u32 {
    if num == k {
        return 1;
    }
    match num {
        0 => 1,
        1 => 1,
        _ => factorial_especial(num - 1, k) * num,
    }
}
fn potencia(num: u32, exponente: u32) -> u32 {
    match exponente {
        0 => 1,
        1 => num,
        _ => potencia(num, exponente - 1) * num,
    }
}
fn digitos_a_float(digitos: u8) -> f64 {
    //aproximación
    let mut num: f64 = 0.5;
    for _i in 0..digitos {
        num *= 0.1;
    }
    num
}
fn main() {
    let mut estimacion_anterior: f64 = 0.0;
    let mut valor_sumatoria: f64 = 0.0;
    let mut valor_sumatoria_parcial: f64 = 0.0;
    let parte_constante: f64 = (2.0 * SQRT_2) / 9801.0;
    let mut estimacion_nueva: f64;
    let mut input = String::new();
    let digitos: u8;
    let comparador_digitos: f64;

    println!("ingresa el número de dígitos");
    match stdin().read_line(&mut input) {
        Ok(_n) => {
            digitos = u8::from_str(&input.trim()).expect("número no válido");
            comparador_digitos = digitos_a_float(digitos);
            println!("todo bien, dígitos: {}", comparador_digitos);
        }
        Err(error) => {
            println!("bien't {}", error);
            return;
        }
    };
    let mut k: u32 = 0;
    'calculo_pi: loop {
        println!("factorial_especial 4k: {}", factorial_especial(4 * k, k));
        println!("el resto: {}", (1103 + 26390 * k));
        //para saltar la primera división
        valor_sumatoria_parcial += f64::from(factorial_especial(4 * k, k));
        //es 396^4k es muy grande
        let mut cont = 0;
        while cont < k {
            //me salto la primera para evitar desfases por division entera
            if cont != 0 {
                valor_sumatoria_parcial /= f64::from(factorial_especial(k, 1))
            }
            valor_sumatoria_parcial /= f64::from(potencia(360, 3));
            valor_sumatoria_parcial /= 396.0;
            cont += 1;
        }
        valor_sumatoria_parcial *= f64::from(1103 + 26390 * k);
        valor_sumatoria += valor_sumatoria_parcial;
        estimacion_nueva = 1.0 / (parte_constante * valor_sumatoria);
        println!("diferencia: {}", estimacion_nueva - estimacion_anterior);
        if (estimacion_nueva - estimacion_anterior).abs() < comparador_digitos {
            println!("valor de pi final: {}", estimacion_nueva);
            break 'calculo_pi;
        }
        println!("valor de pi k:{} {}", k, estimacion_nueva);
        estimacion_anterior = estimacion_nueva;
        k += 1;
    }
}