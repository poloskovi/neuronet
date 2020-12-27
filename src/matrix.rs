extern crate rand;
use rand::Rng;
use std::fmt;

type Tdata = i32;
const FORMFACTOR: i32 = 256;

/// Матрица типа TData произвольного размера
/// # Examples
///
/// ```
/// let mut m = Matrix::new(0);
/// m.set(2, 3, 5);
/// m.prt();
/// ```
pub struct Matrix{
    m: Vec<Tdata>,
    pub nrow: usize,
    pub ncol: usize,
}

impl fmt::Display for Matrix {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        // для больших матриц вместо части строк и столбцов выводим ...
        let skip_rows_after: usize = 14;
        let skip_columns_after: usize = 9;
        
        let skip_rows = self.nrow > skip_rows_after + 1;
        let skip_columns = self.ncol > skip_columns_after + 1;
        
        for row in 0..self.nrow{
            
            if skip_rows {
                if row > skip_rows_after + 1 && row != self.nrow-1 {
                    continue;
                }
            }
            
            if self.nrow == 1{
                write!(f, "⟮ ")?; //U+27EE
            } else if row == 0 {
                write!(f, "⎛ ")?; //U+239B
            } else if row == self.nrow-1 {
                write!(f, "⎝ ")?;
            } else {
                write!(f, "⎜ ")?;
            }
            
            for col in 0..self.ncol{
                if skip_columns {
                    if col > skip_columns_after && col != self.ncol-1 {
                        if col == skip_columns_after + 1 {
                            write!(f, "{:4} ", " ...")?;
                        }
                        continue;
                    }
                }
                if skip_rows && row == skip_rows_after + 1 {
                    write!(f, "{:4} ", " ...")?;
                }else{
                    write!(f, "{:4} ", self.get(row,col))?;
                }
            }
            if self.nrow == 1{
                writeln!(f, " ⟯")?;
            } else if row == 0 {
                writeln!(f, " ⎞")?;
            } else if row == self.nrow-1 {
                writeln!(f, " ⎠")?;
            } else {
                writeln!(f, " ⎟")?;
            }
        }
        write!(f, "")
    }
}

impl Matrix{
    
    /// Конструктор: Инициализация матрицы и заполнение всех элементов нулем
    pub fn new(nrow: usize, ncol: usize) -> Matrix {
        Matrix {
            m: vec![0; ncol*nrow],
            nrow: nrow,
            ncol: ncol,
        }
    }
    
    /// Конструктор: Квадратная единичная матрица
    pub fn new_ed(nrowcol: usize) -> Matrix {
        let mut result = Matrix::new(nrowcol, nrowcol);
        for i in 0..nrowcol{
            result.set(i,i,1);
        }
        result
    }
    
    /// Конструктор: Матрица случайных чисел от xmin до xmax
    pub fn new_rand(nrow: usize, ncol: usize, xmin: Tdata, xmax: Tdata, nonzero: bool) -> Matrix {
        let mut result = Matrix::new(nrow, ncol);
        
        let mut rng = rand::thread_rng();

        for index in 0..result.m.len(){

            let mut x = rng.gen_range(xmin, xmax);
            if nonzero && x == 0 {
                x = 1;
            }
            result.m[index] = x;
        }
        result
    }
    
    // Возвращает значение в ячейке
    pub fn get(&self, row:usize, col:usize) -> Tdata {
        let index = row * self.ncol + col;
        self.m[index]
    }
    
    /// Устанавливает значение x в ячейку (i,j)
    pub fn set(&mut self, row:usize, col:usize, x: Tdata) {
        let index = row * self.ncol + col;
        self.m[index] = x;
    }
    
    /// Возвращает транспонированную матрицу
    pub fn t(&self) -> Matrix{
        let mut result = Matrix::new(self.ncol, self.nrow);
        for row in 0..self.nrow {
            for col in 0..self.ncol {
                result.set(col,row, self.get(row,col));
            }
        }
        result
    }
    
    // Вычитание
    pub fn sub(m1: &Matrix, m2: &Matrix) -> Matrix{
    
        if (m1.nrow != m2.nrow) || (m1.ncol != m2.ncol){
            panic!("Размерности матриц не совпадают {}x{} != {}x{}", m1.nrow,m1.ncol, m2.nrow,m2.ncol);
        }
    
        let mut result = Matrix::new(m1.nrow, m2.ncol);
        for index in 0..result.m.len(){
            result.m[index] = m1.m[index] - m2.m[index];
        }
        result
    }

    // Сложение
    pub fn add(m1: &Matrix, m2: &Matrix) -> Matrix{
    
        if (m1.nrow != m2.nrow) || (m1.ncol != m2.ncol){
            panic!("Размерности матриц не совпадают {}x{} != {}x{}", m1.nrow,m1.ncol, m2.nrow,m2.ncol);
        }
    
        let mut result = Matrix::new(m1.nrow, m2.ncol);
        for index in 0..result.m.len(){
            result.m[index] = m1.m[index] + m2.m[index];
        }
        result
    }

    // Умножение
    pub fn mul(m1: &Matrix, m2: &Matrix) -> Matrix{
    
        if m1.ncol != m2.nrow{
            panic!("Размерности матриц не совпадают {} != {}", m1.ncol, m2.nrow);
        }
    
        let mut result = Matrix::new(m1.nrow, m2.ncol);
        for i in 0..m1.nrow {
            for j in 0..m2.ncol {
                let mut cij = 0;
                for r in 0..m1.ncol {
                    let air = m1.get(i,r);
                    let brj = m2.get(r,j);
                    cij = cij + air*brj;
                }
                result.set(i,j,cij / FORMFACTOR);
            }
        }
        result
    }

    // Умножение матриц с одновременным применением сигмоиды
    // так будет еще быстрее
    pub fn mul_and_sigmoida(m1: &Matrix, m2: &Matrix, sigmoida: &Sigmoida) -> Matrix{
    
        if m1.ncol != m2.nrow{
            panic!("Размерности матриц не совпадают {} != {}", m1.ncol, m2.nrow);
        }
    
        let mut result = Matrix::new(m1.nrow, m2.ncol);
        for i in 0..m1.nrow {
            for j in 0..m2.ncol {
                let mut cij = 0;
                for r in 0..m1.ncol {
                    let air = m1.get(i,r);
                    let brj = m2.get(r,j);
                    cij = cij + air*brj;
                }
                let cij_sigm = sigmoida.f_one(cij/FORMFACTOR);
                result.set(i,j,cij_sigm);
            }
        }
        result
    }

    pub fn m1_correctnet(errors: &Matrix, signal: &Matrix) -> Matrix {

        if (errors.nrow != signal.nrow) || (errors.ncol != signal.ncol){
            panic!("Размерности матриц не совпадают {}x{} != {}x{}", errors.nrow,errors.ncol, signal.nrow,signal.ncol);
        }
    
        let mut result = Matrix::new(errors.nrow, errors.ncol);
    
        for index in 0..result.m.len(){
        
            // так у Т.Рашида: (1/koeff) * errors * signal * (1-signal)
            result.m[index] = errors.m[index] * signal.m[index] * (FORMFACTOR - signal.m[index]) / (FORMFACTOR * FORMFACTOR);

        }
        result
    }

}

// Данные заранее вычисленной сигмоиды.
// Значение сигмоиды = 0..2^8
// Сигмоида умножается на коэффициенты матрицы, их значения -2^7..+2^7
// Результат -2^15..+2^15
// Это число надо привести к 0..255
pub struct Sigmoida{
    index_zero: Tdata,
    koeff_y: f32,
    koeff_x: f32,
    len: Tdata,
    m:[u8; FORMFACTOR as usize],
}

impl Sigmoida{
    pub fn new() -> Sigmoida{
        let mut result = Sigmoida{
            index_zero: 127,
            koeff_y: FORMFACTOR as f32,
            koeff_x: 22.0,
            len: FORMFACTOR,
            m:[0; FORMFACTOR as usize],
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
    
    pub fn get(&self, x:Tdata) -> u8{
        // x - входной сигнал. Он может быть положительным и отрицательным.
        // его нужно привести к index - индексу элемента массива значений сигмоиды
        let mut index = x + self.index_zero;
        if index < 0 {
            index = 0
        }else if index >= self.len {
            index = self.len-1
        };
//         println!("{}: {}", x, index);
        let res = self.m[index as usize] as Tdata;
        // Чтобы нейросеть не вырождалась, на концах оставляем дельту ~10%
        // (см. функцию m1_correctnet)
        let delta = FORMFACTOR / 32;
        if res < delta{
            delta as u8
        } else if res > FORMFACTOR - delta{
            (FORMFACTOR - delta) as u8
        } else {
            res as u8
        }
    }
    
    pub fn f_matrix(&self, input: &Matrix) -> Matrix{
        let mut output = Matrix::new(input.nrow, input.ncol);
        for i in 0..input.m.len(){
            output.m[i] = self.get(input.m[i]) as Tdata
        }
        output
    }

    pub fn f_one(&self, input: Tdata) -> Tdata{
        self.get(input) as Tdata
    }
    
}
