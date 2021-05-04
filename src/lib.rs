//! Библиотека работы с нейросетью на базе целочисленных матриц
//! и заранее вычисленной функцией сигмоиды

extern crate rand;
use rand::Rng;
use std::fmt;

extern crate matrix;
use matrix::Matrix;

/// Тип элементов матриц
pub type Tdata = i32;
pub type NeuroMatrix = Matrix<Tdata>;

/// Формфактор: коэффициент перевода значения элемента матрицы.
/// Если a(i,j) = FORMFACTOR, это значит, что a(i,j) = 1.0
pub const FORMFACTOR: i32 = 256;

// хвосты сигма-функции, чтобы не вырождалась нейросеть
const TAIL_DOWN: i32 = 4;
const TAIL_UP: i32 = FORMFACTOR - TAIL_DOWN;

pub trait MatrixAdditions{
    fn new_ed(nrowcol: usize) -> NeuroMatrix;
    fn new_rand(nrow: usize, ncol: usize, xmin: Tdata, xmax: Tdata, nonzero: bool) -> NeuroMatrix;
    fn mul_formfactor(m1: &NeuroMatrix, m2: &NeuroMatrix) -> NeuroMatrix;
    fn mul_and_sigmoida(m1: &NeuroMatrix, m2: &NeuroMatrix, sigmoida: &Sigmoida) -> NeuroMatrix;
    fn m1_correctnet(errors: &NeuroMatrix, signal: &NeuroMatrix) -> NeuroMatrix;
    fn modify(&self, procent: f32) -> NeuroMatrix;
    fn distance(&self, other:&NeuroMatrix) -> i32;
}

impl MatrixAdditions for NeuroMatrix{
    
    /// Квадратная единичная матрица
    fn new_ed(nrowcol: usize) -> NeuroMatrix {
        let mut result = Matrix::new(nrowcol, nrowcol);
        for i in 0..nrowcol{
            result.set(i,i,1);
        }
        result
    }
    
    /// Матрица случайных чисел от xmin до xmax
    fn new_rand(nrow: usize, ncol: usize, xmin: Tdata, xmax: Tdata, nonzero: bool) -> NeuroMatrix {
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

    /// Умножение с приведением к формфактору
    /// Если a = FORMFACTOR (то есть 1.0)
    /// и b = FORMFACTOR (то есть 1.0),
    /// то a*b = FORMFACTOR (то есть 1.0)
    fn mul_formfactor(m1: &NeuroMatrix, m2: &NeuroMatrix) -> NeuroMatrix{
        assert_eq!(m1.ncol, m2.nrow);
        let mut result = Matrix::new(m1.nrow, m2.ncol);
        for i in 0..m1.nrow {
            for j in 0..m2.ncol {
                let mut cij = 0;
                for r in 0..m1.ncol {
                    cij += m1.get(i,r) * m2.get(r,j);
                }
                result.set(i,j, 
                    cij/FORMFACTOR);
            }
        }
        result
    }

    /// Умножение матриц с применением к результату сигмоиды 
    fn mul_and_sigmoida(m1: &NeuroMatrix, m2: &NeuroMatrix, sigmoida: &Sigmoida) -> NeuroMatrix{
        assert_eq!(m1.ncol, m2.nrow);
        let mut result = Matrix::new(m1.nrow, m2.ncol);
        for i in 0..m1.nrow {
            for j in 0..m2.ncol {
                let mut cij = 0;
                for r in 0..m1.ncol {
                    cij += m1.get(i,r) * m2.get(r,j);
                }
                result.set(i,j,
                    sigmoida.f_one(cij/FORMFACTOR));
            }
        }
        result
    }

    fn m1_correctnet(errors: &NeuroMatrix, signal: &NeuroMatrix) -> NeuroMatrix{
        assert_eq!(errors.nrow, signal.nrow);
        assert_eq!(errors.ncol, signal.ncol);
        let mut result = Matrix::new(errors.nrow, errors.ncol);
        for index in 0..result.m.len(){
            // так у Т.Рашида: (1/koeff) * errors * signal * (1-signal)
            result.m[index] = errors.m[index] * signal.m[index] 
                * (FORMFACTOR - signal.m[index]) 
                / (FORMFACTOR * FORMFACTOR * 4);
        }
        result
    }
    
    /// слегка измененная матрица
    fn modify(&self, procent: f32) -> NeuroMatrix{
        
        let mut result = Matrix::new(self.nrow, self.ncol);
        let mut rng = rand::thread_rng();
        
        for i in 0..self.nrow {
            for j in 0..self.ncol {
                let x = self.get(i,j);
                let dx_max = (FORMFACTOR as f32 * procent / 100.0) as Tdata;
                let dx = rng.gen_range(-dx_max, dx_max);
                let mut x_new = x+dx;
                if x_new < 0{
                    x_new = 0;
                }else if x_new > FORMFACTOR{
                    x_new = FORMFACTOR;
                }
                result.set(i,j, x_new);
            }
        }
        result
        
    }
    
    fn distance(&self, other:&NeuroMatrix) -> i32{
    
        assert_eq!(self.nrow, other.nrow);
        assert_eq!(self.ncol, other.ncol);
        
        let mut result = 0;
        for row in 0..self.nrow {
            for col in 0..self.ncol {
                let d = self.get(row,col) - other.get(row,col);
                result += d*d;
            }
        }
        result
    }
}

/// Данные заранее вычисленной сигмоиды.
/// Значение сигмоиды = 0..2⁸
/// Сигмоида умножается на коэффициенты матрицы, их значения -2⁷..+2⁷
/// Результат равен -2¹⁵..+2¹⁵
/// Это число надо привести к 0..255
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
            // Коэффициент растяжения сигмоиды вдоль оси y. То же самое , что формфактор
            koeff_y: FORMFACTOR as f32,
            // Коэффициент растяжения сигмоиды вдоль оси х.
            // Значение подобрано опытным путем, 
            // чтобы на концах сигмоида плавно подходила к TAIL_DOWN и TAIL_UP
            koeff_x: 30.0,
            // Количество записей в массиве.
            len: FORMFACTOR,
            // Значения сигмоиды
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
        let res = self.m[index as usize] as Tdata;
        // Чтобы нейросеть не вырождалась, на концах оставляем дельту ~5%
        // (см. функцию m1_correctnet)
        if res < TAIL_DOWN{
            TAIL_DOWN as u8
        } else if res > TAIL_UP{
            TAIL_UP as u8
        } else {
            res as u8
        }
    }
    
    pub fn f_one(&self, input: Tdata) -> Tdata{
        self.get(input) as Tdata
    }
    
}

/// Нейросеть.
pub struct Neuronet{
    //весовые коэффициенты связей слоев
    net: Vec<NeuroMatrix>
}

impl Neuronet{
    
    // nnodes - вектор количества ячеек в слоях
    pub fn new(nnodes: Vec<usize>) -> Neuronet{
        let mut net = Vec::<NeuroMatrix>::new();
        for i in 0..nnodes.len()-1 {
            // весовые коэффициенты связи слоев (i) и (i+1)
            net.push(Matrix::new_rand(nnodes[i], nnodes[i+1], -127, 127, true)); 
        };
        Neuronet{
            net: net,
        }
    }
    
    /// Значение выходного сигнала нейросети для значения входного сигнала
    pub fn getoutput(&self, input: &NeuroMatrix, sigmoida: &Sigmoida) -> NeuroMatrix{
    
        let mut next = Matrix::new(1, 1);   // фиктивное значение, чтобы компилятор не ругался 
                                            //на возможно неинициализированную переменную
        for i in 0..self.net.len() {
            next = Matrix::mul_and_sigmoida(
                match i {
                    0 => input,
                    _ => &next,
                }
                , &self.net[i], sigmoida
            ); 
        }
        next
    }
    
    /// Тренировка нейросети
    pub fn training(&mut self, input: &NeuroMatrix, target: &NeuroMatrix, sigmoida: &Sigmoida){
    
        let n_layers = self.net.len();// количество матриц в нейросети
        let index_layer_max = n_layers-1; // максимальный индекс матриц
        
        // Получение выходных значений на каждом слое
        let mut outputs  = Vec::<NeuroMatrix>::new();
        for i in 0..self.net.len() {
            outputs.push( 
                Matrix::mul_and_sigmoida(
                    match i {
                        0 => input,
                        _ => &outputs[i-1],
                    }
                , &self.net[i], sigmoida)
            ); 
        }

        // проверка, что размер выходного сигнала совпадает с размером цели
        assert_eq!(outputs[index_layer_max].nrow, target.nrow);
        assert_eq!(outputs[index_layer_max].ncol, target.ncol);
        
        // Корректировка весов связей нейросети
        let mut error = Matrix::new(1, 1); // фиктивное значение, чтобы компилятор не ругался
                                           // на возможно неинициализированную переменную
        for i in 0..self.net.len() {
            let index = self.net.len() - i - 1;
            error = 
                match i {
                    0 => Matrix::sub(target, &outputs[index_layer_max]),
                    _ => Matrix::mul_formfactor(&self.net[index+1], &error.t()).t(),
                };
            let m1 = Matrix::m1_correctnet(&error, &outputs[index]);
            let delta = Matrix::mul_formfactor(&m1.t(), 
                match index {
                    0 => input,
                    _ => &outputs[index-1],
                }
            ).t();
            self.net[index] = Matrix::add(&self.net[index], &delta);
        }
    }

    /// Количество соединений в нейросети (равно количеству ячеек матриц)
    pub fn count_of_connection(&self) -> usize{
        let mut result = 0;
        for matrix in &self.net {
            result += matrix.count_of_cells()
        }
        result
    }
    
    /// количество входов нейросети
    pub fn n_inputs(&self) -> usize{
        self.net[0].nrow
    }
        
    /// количество выходов нейросети
    pub fn n_outputs(&self) -> usize{
        self.net[self.net.len()-1].ncol
    }
        
}

impl fmt::Display for Sigmoida {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for index in 0..self.m.len() {
            writeln!(f, "{} ", self.m[index])?;
        }
        writeln!(f, "")
    }
}

