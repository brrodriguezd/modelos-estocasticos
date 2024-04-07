use core::{f64, panic};
use std::f64::consts::SQRT_2;
use std::io::stdin;
use std::str::FromStr;
use std::usize;

mod numero_real;
/*
 *FUNCIONA HASTA EL DÍGITO 15 (3.141592653589793)
*/
/* esta función halla el factorial de un número NUM pero se detiene en otro número K
* k debe ser menor que num, mayor o igual a 0
* parámetros: unsigned de 32 bits, es el mayor tamaño que puede ser convertido a floats de 64 bits
* retorna: unsigned de 32 bits
*/
fn factorial_especial(num: u32, k: u32) -> u32 {
    if num == k {
        return 1;
    } else {
        return factorial_especial(num - 1, k) * num;
    }
}
/* esta función halla el factorial de un número NUM
* parámetros: unsigned de 32 bits, es el mayor tamaño que puede ser convertido a floats de 64 bits
* retorna: unsigned de 32 bits
*/
fn factorial(num: u32) -> u32 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}

/* esta función crea el error que usamos para detener la iteración del programa según el input del
* usuario
* parámetros: usize, usize es el tipo usado por el formateador de floats
* retorna: float de 32 bits
*/
fn digitos_a_float(digitos: usize) -> f64 {
    //aproximación
    let mut num: f64 = 0.5;
    for _i in 0..digitos {
        num *= 0.1;
    }
    num
}
/* pide el input de dígitos del usuario,
* parámetros:
* retorna: tupla(usize, f64), digitos para el formato y el valor flotante para el control de la
* sumatoria infinita
*/
fn input() -> (usize, f64) {
    let mut input = String::new();
    let digitos: usize;
    let comparador_digitos: f64;
    println!("ingresa el número de dígitos");
    match stdin().read_line(&mut input) {
        Ok(_n) => {
            digitos = usize::from_str(&input.trim()).expect("número no válido");
            comparador_digitos = digitos_a_float(digitos);
            println!("dígitos: {}", comparador_digitos);
            return (digitos, comparador_digitos);
        }
        Err(error) => {
            println!("bien't {}", error);
            panic!("Adiós");
        }
    };
}

fn main() {
    /*PRUEBAS DEL TIPO NUMERO REAL
        let numero1 = numero_real::NumeroReal::new(10.0, 7);
        //9 digitos por lo que debería redondear el último
        let numero2 = numero_real::NumeroReal::new(16.123412386, 8);
        //todos los valores deben ser de 8 dígitos
        println!("numero 1: {}, numero 2: {}", numero1, numero2);
        //26.12341239;
        println!("suma: {}", numero1 + numero2);
        //6.12341239;
        println!("resta: {}", numero1 - numero2);
        // deberia ser 161.2341239 en vez de 161.23412386 por el redondeo y digitos que maneja
        println!("multiplicación n1*n2: {}", numero1 * numero2);
        println!("multiplicación n2*n1: {}", numero2 * numero1);
        //este es el unico de 7 digitos porque n1 es de 7
        println!("multiplicación n1*n1: {}", numero1 * numero1);
        // deberia ser 1.61234124 en vez de 1.612341239 por el redondeo por sobrepasar dígitos
        println!("division n1/n2: {}", numero1 / numero2);
        //nose jasjja
        println!("division n2/n1: {}", numero2 / numero1);
        //1.00000000 8 dígitos
        println!("division n2/n2: {}", numero2 / numero2);
        //1.0000000 7 dígitos
        println!("division n2/n2: {}", numero1 / numero1);
        //10000.00000000
        println!("10⁴: {}", numero1.potencia(4));
        //nose
        println!("n2⁴: {}", numero2.potencia(4));
    */
    //variables para el cálculo
    let mut estimacion_anterior: f64 = 0.0;
    let mut valor_sumatoria: f64 = 0.0;
    let mut valor_sumatoria_parcial: f64;
    let parte_constante: f64 = 9801.0 / (2.0 * SQRT_2);
    let mut estimacion_nueva: f64;
    //recibe los dígitos del usuario
    let (digitos, comparador_digitos) = input();
    let mut k: u32 = 0;
    'calculo_pi: loop {
        //de menor a mayor para más precisión
        //en la primera el denominador es 1
        let mut cont = 0;
        valor_sumatoria_parcial = f64::from(1103 + 26390 * k);
        while cont < 4 * k {
            //396⁴ es demasiado grande, por eso subdividi todas las potencias y factoriales
            valor_sumatoria_parcial /= 396.0;
            if cont % k == 0 {
                valor_sumatoria_parcial *=
                    f64::from(factorial_especial((4 - cont / k) * k, (3 - cont / k) * k));
                valor_sumatoria_parcial /= f64::from(factorial(k));
            }
            cont += 1;
        }
        valor_sumatoria += valor_sumatoria_parcial;
        //println!("valor sumatoria: {:.}", valor_sumatoria);
        k += 1;
        estimacion_nueva = parte_constante / valor_sumatoria;
        //println!("diferencia: {:.}", estimacion_nueva - estimacion_anterior);
        if (estimacion_nueva - estimacion_anterior).abs() < comparador_digitos {
            //imprime en pantalla los dígitos que se piden
            println!("valor de pi final: {:.*}", digitos, estimacion_nueva);
            break 'calculo_pi;
        }
        //println!("valor de pi:{} k: {}", estimacion_nueva, k);
        estimacion_anterior = estimacion_nueva;
    }
}
