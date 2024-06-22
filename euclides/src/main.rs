use std::fs;

fn main() {
    let content = fs::read_to_string("./input").expect("error al leer el archivo input");
    let list: Vec<&str> = content.split('\n').collect();

    if list.len() != 2 {
        panic!("error al leer el archivo input")
    }
    let mut a: i64 = list[0].parse().unwrap();
    let mut b: i64 = list[1].parse().unwrap();
    loop {
        if a == b {
            let result = fs::write("./output", a.to_string());
            match result{
                Ok(()) => break,
                Err(_) => panic!("error al escribir el archivo")
            }
        }
        if a < b {
            b = b-a;
        }else{
            a = a-b;
        }
    }
    println!("Algoritmo terminado exitosamente");
}
