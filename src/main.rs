// На выходе узла сети сигнал в целых числах (+/-), Uвых(i)
// Выходной сигнал умножается на значение матрицы: Uвых(i) * A(i,j)
// Входным сигналом следующего слоя является сигмоида сумм выходных сигналов предыдущего слоя, умноженных на матрицу значений:
// sigm(СУММ(Uвых(i) * A(i,j)))
// Это значение выходного сигнала ячейки

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

// Структура для хранения заранее вычисленной сигмоиды.
// Значение сигмоиды = 0..2^8
// Сигмоида умножается на коэффициенты матрицы, их значения -2^7..+2^7
// Результат -2^15..+2^15
// Это число надо привести к 0..255
pub struct Sigmoida{
    index_zero: i32,
    koeff_y: f32,
    koeff_x: f32,
    len: i32,
    m:[u8; 256],
}

impl Sigmoida{
    pub fn new() -> Sigmoida{
        let mut result = Sigmoida{
            index_zero: 127,
            koeff_y: 256.0,
            koeff_x: 22.0,
            len: 256,
            m:[0; 256],
        };
        let index_zero_real = result.index_zero as f32;
        for i in 0..result.m.len(){
            result.m[i] = result.getinitsigmoida(i, index_zero_real);
        }
        result
    }
    fn getinitsigmoida(&self, i:usize, index_zero_real: f32) -> u8{
        let x: f32 = (i as f32 - index_zero_real) / self.koeff_x;
        let exp = (-x).exp();
        let y = 1.0/ (1.0 + exp);
        (y * self.koeff_y) as u8
    }
    pub fn get(&self, x:i32) -> u8{
        // x - входной сигнал. Он может быть положительным и отрицательным.
        // его нужно привести к index - индексу элемента массива значений сигмоиды
        let mut index = x + self.index_zero;
        if index < 0 {
            index = 0
        }else if x >= self.len {
            index = self.len-1
        };
        self.m[index as usize]
    }
//     // получение индекса массива по величине входного сигнала
//     fn get_index(&self, x:i32) -> usize{
//         let index_real = x * self.koeff_x_real + self.index_zero_real;
//         if index_real < 0.0 {
//             0
//         } else if index_real > (self.m.len()-1) as f32{
//             self.m.len()-1
//         } else {
//             index_real as usize
//         }
//     }

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
    for i in -127..128 {
        println!("{}: {}", i, sigmoida.get(i as i32));
    }
    println!("");
    println!("{}: {}", -1000, sigmoida.get(-1000 as i32));
    println!("{}: {}", 1000, sigmoida.get(1000 as i32));
}
