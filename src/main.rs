use std::ops::Add;
use std::ops::Mul;

const N:usize = 3;

/// Массив типа i16 размером N
/// # Examples
///
/// ```
/// let mut m = Massiv::new(0);
/// m.set(2, 5);
/// m.prt();
/// ```
#[derive(Clone, Copy)]
pub struct Massiv{
    m: [i16; N],
}

impl Massiv {
    /// Инициализация массива и заполнение всех элементов заданным значением
    pub fn new(x: i16) -> Massiv {
        Massiv {m: [x; N]}
    }
    pub fn get(&self, i:usize) -> i16 {
        self.m[i]
    }
    /// Установка значения x в ячейку (i)
    /// # Examples
    ///
    /// ```
    /// m.set(2, 5);
    /// ```
    pub fn set(&mut self, i:usize, x: i16) {
        self.m[i] = x;
    }
    /// Вывод матрицы на экран
    /// # Examples
    ///
    /// ```
    /// m.prt();
    /// ```
    pub fn print(&self) {
        for x in &self.m{
            print!("{} ", x);
        }
        println!();
    }
}

impl Add for Massiv {
    type Output = Massiv;

    fn add(self, other: Massiv) -> Massiv{
        let mut result = Massiv::new(0);
        for i in 0..N {
            let ai = self.get(i);
            let bi = other.get(i);
            result.set(i, ai+bi);
        }
        result
    }
}

/// Матрица типа i16 размером NxN
/// # Examples
///
/// ```
/// let mut m = Matrix::new(0);
/// m.set(2, 3, 5);
/// m.prt();
/// ```
#[derive(Clone, Copy)]
pub struct Matrix{
    m: [Massiv; N]
}

impl Matrix{
    /// Инициализация матрицы и заполнение всех элементов заданным значением
    pub fn new(x: i16) -> Matrix {
        Matrix {m: [Massiv::new(x);N] }
    }
    pub fn new_ed() -> Matrix {
        let mut result = Matrix::new(0);
        for i in 0..N{
            result.set(i,i,1);
        }
        result
    }
    pub fn get(&self, i:usize, j:usize) -> i16 {
        self.m[i].get(j)
    }
    /// Установка значения x в ячейку (i,j)
    pub fn set(&mut self, i:usize, j:usize, x: i16) {
        self.m[i].set(j, x);
    }
    pub fn get2(&self, i:usize) -> Massiv {
        self.m[i]
    }
    pub fn set2(&mut self, i:usize, x: Massiv) {
        self.m[i] = x;
    }
    /// Вывод матрицы на экран
    pub fn print(&self) {
        for x in &self.m{
            x.print();
        }
    }
}

impl Add for Matrix {
    type Output = Matrix;

    fn add(self, other: Matrix) -> Matrix{
        let mut result = Matrix::new(0);
        for j in 0..N {
            result.set2(j, self.get2(j)+other.get2(j));
        }
        result
    }
}

impl Mul for Matrix{
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Matrix{
        let mut result = Matrix::new(0);
        for i in 0..N {
            for j in 0..N {
                let mut cij = 0;
                for r in 0..N {
                    let air = self.get(i,r);
                    let brj = other.get(r,j);
                    cij = cij + air*brj;
                }
                result.set(i,j,cij);
            }
        }
        result
    }
    
//     fn mul2(self, other: Massiv) -> Matrix{
//         let mut result = Matrix::new(0);
//         for i in 0..N {
//             for j in 0..0 {
//                 let mut cij = 0;
//                 for r in 0..N {
//                     let air = self.get(i,r);
//                     let brj = other.get(r);
//                     cij = cij + air*brj;
//                 }
//                 result.set(i,j,cij);
//             }
//         }
//         result
//     }
}

fn main() {

    let mut m = Matrix::new(1);
    m.set(1, 2, 5);
    m.set(2, 2, 6);
    
    let mut ed = Matrix::new_ed();
    ed.set(0,1,1);
    
    let k = ed*m;
    
    ed.print();
    println!("X");
    m.print();
    println!("=");
    k.print();
}
