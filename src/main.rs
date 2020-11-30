
/// Матрица типа i16 произвольного размера
/// # Examples
///
/// ```
/// let mut m = Matrix::new(0);
/// m.set(2, 3, 5);
/// m.prt();
/// ```
// #[derive(Clone, Copy)]
pub struct Matrix{
    m: Vec<i16>,
    nrow: usize,
    ncol: usize,
}

fn min(v1:usize, v2:usize)->usize{
    if v1>v2 {v2}else{v1}
}

impl Matrix{
    /// Инициализация матрицы и заполнение всех элементов заданным значением
    pub fn new(nrow: usize, ncol: usize, x: i16) -> Matrix {
        Matrix {
            m: vec![x; ncol*nrow],
            nrow: nrow,
            ncol: ncol,
        }
    }
    pub fn new_ed(nrow: usize, ncol: usize) -> Matrix {
        let mut result = Matrix::new(ncol, nrow, 0);
        for i in 0..min(nrow, ncol){
            result.set(i,i,1);
        }
        result
    }
    pub fn get(&self, i:usize, j:usize) -> i16 {
        let index = i * self.ncol + j;
        self.m[index]
    }
    /// Установка значения x в ячейку (i,j)
    pub fn set(&mut self, i:usize, j:usize, x: i16) {
        let index = i * self.ncol + j;
        self.m[index] = x;
    }
    /// Вывод матрицы на экран
    pub fn print(&self) {
        for i in 0..self.nrow{
            for j in 0..self.ncol{
                print!("{} ", self.get(i,j));
            }
            println!();
        }
    }
}

fn matrix_mult(m1: &Matrix, m2: &Matrix) -> Matrix{
    if m1.ncol != m2.nrow{
        panic!("Размерности матриц не совпадают");
    }
//     assert_eq!(m1.ncol, m2.nrow);
    let mut result = Matrix::new(m1.nrow, m2.ncol, 0);
    for i in 0..m1.nrow {
        for j in 0..m2.ncol {
            let mut cij = 0;
            for r in 0..m1.ncol {
                let air = m1.get(i,r);
                let brj = m2.get(r,j);
                cij = cij + air*brj;
            }
            result.set(i,j,cij);
        }
    }
    result
}

/// Структура для хранения заранее вычисленной сигмоиды
pub struct Sigmoida{
    index_zero: f32,
    koeff_y: f32,
    koeff_x: f32,
    m:[u16; 100]
}

impl Sigmoida{
    pub fn new() -> Sigmoida{
        let mut result = Sigmoida{
            index_zero: 50.0,
            koeff_y: 65000.0,
            koeff_x: 5.0,
            m:[0; 100],
        };
        for i in 0..result.m.len(){
            result.m[i] = result.getinitsigmoida(i);
        }
        result
    }
    fn getinitsigmoida(&self, i:usize) -> u16{
        let x: f32 = ((i as f32) - self.index_zero) / self.koeff_x;
        let exp = (-x).exp();
        let y = 1.0/ (1.0 + exp);
        (y * self.koeff_y) as u16
    }
    pub fn get(&self, x:f32) -> f32{
        let index_f: f32 = (x * self.koeff_x) + self.index_zero;
        let index: usize = if index_f < 0.0 {
            0
        } else if index_f > (self.m.len()-1) as f32{
            self.m.len()-1
        } else {
            index_f as usize
        };
        self.m[index] as f32 / self.koeff_y
    }

}

fn main() {
    
    let mut a = Matrix::new_ed(3, 3);
    let mut b = Matrix::new_ed(3, 3);
    
    a.set(0, 0, 5);
    a.set(0, 1, 6);
    
    b.set(1, 2, 5);
    b.set(2, 2, 6);
    
    a.print();
    println!("X");
    b.print();
    println!("=");
    let c = matrix_mult(&a, &b);
    c.print();
    
    let sigmoida = Sigmoida::new();
//     for i in 0..sigmoida.m.len() {
//         println!("{}", sigmoida.m[i]);
//     }
    for i in -5..6 {
        println!("{}: {}", i, sigmoida.get(i as f32));
    }
}
