use core::fmt;
use std::ops::{Add, Div, Mul, Sub};
//para no consumir el valor cuando llame +, -, *, /.
#[derive(Clone, Copy)]
pub struct NumeroReal {
    numero: f64,
    digitos: u16,
}

//para que no muestre advertencias de no uso
#[allow(dead_code)]
impl NumeroReal {
    pub fn new(numero: f64, digitos: u16) -> Self {
        NumeroReal {
            numero: (numero * f64::from(10_u32.pow(digitos.into()))).round()
                / f64::from(10_u32.pow(digitos.into())),
            digitos,
        }
    }
    pub fn get_numero(&self) -> f64 {
        self.numero
    }
    pub fn get_digitos(&self) -> u16 {
        self.digitos
    }
    pub fn potencia(self, exponente: u32) -> NumeroReal {
        match exponente {
            0 => NumeroReal {
                numero: 1.0,
                digitos: self.digitos,
            },
            1 => self,
            _ => self.potencia(exponente - 1) * self,
        }
    }
}
//tanto suma como resta no cambian el número de dígitos
//si son diferentes el numero de digitos retorna el mayor
//IMPL del símbolo +
impl Add for NumeroReal {
    type Output = Self;
    fn add(self, sumando: Self) -> Self {
        Self {
            numero: (self.numero + sumando.get_numero()),
            digitos: if self.digitos < sumando.get_digitos() {
                sumando.get_digitos()
            } else {
                self.digitos
            },
        }
    }
}

//IMPL del símbolo -
impl Sub for NumeroReal {
    type Output = Self;
    fn sub(self, sumando: Self) -> Self {
        Self {
            numero: self.numero - sumando.get_numero(),
            digitos: if self.digitos < sumando.get_digitos() {
                sumando.get_digitos()
            } else {
                self.digitos
            },
        }
    }
}

//IMPL del símbolo *
impl Mul for NumeroReal {
    type Output = Self;
    fn mul(self, sumando: Self) -> Self {
        let digitos = if self.digitos < sumando.get_digitos() {
            sumando.get_digitos()
        } else {
            self.digitos
        };
        Self {
            //redondear despues de los digitos
            numero: ((self.numero * sumando.get_numero()) * f64::from(10_u32.pow(digitos.into())))
                .round()
                / f64::from(10_u32.pow(digitos.into())),
            digitos,
        }
    }
}

//IMPL del símbolo /
impl Div for NumeroReal {
    type Output = Self;
    fn div(self, sumando: Self) -> Self {
        let digitos = if self.digitos < sumando.get_digitos() {
            sumando.get_digitos()
        } else {
            self.digitos
        };
        Self {
            //redondear despues de los digitos
            numero: ((self.numero / sumando.get_numero()) * f64::from(10_u32.pow(digitos.into())))
                .round()
                / f64::from(10_u32.pow(digitos.into())),
            digitos,
        }
    }
}
//para mostrarlo en consola
impl fmt::Display for NumeroReal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.*}", usize::from(self.digitos), self.numero)
    }
}
