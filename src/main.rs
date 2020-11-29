
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
//     if m1.ncol <> m2.nrow{
//         panic!("Размерности матриц не совпадают");
//     }
    assert_eq!(m1.ncol, m2.nrow);
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
    
}
