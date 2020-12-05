// На выходе узла сети сигнал в целых числах (+/-), Uвых(i)
// Выходной сигнал умножается на значение матрицы: Uвых(i) * A(i,j)
// Входным сигналом следующего слоя является сигмоида сумм выходных сигналов предыдущего слоя, умноженных на матрицу значений:
// sigm(СУММ(Uвых(i) * A(i,j)))
// Это значение выходного сигнала ячейки

extern crate rand;
use rand::Rng;
type Tdata = i32;
const FORMFACTOR: i32 = 256;

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
    m: Vec<Tdata>,
    nrow: usize,
    ncol: usize,
}

fn min(v1:usize, v2:usize)->usize{
    if v1>v2 {v2}else{v1}
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
    
    /// Конструктор: Единичная матрица
    pub fn new_ed(nrow: usize, ncol: usize) -> Matrix {
        let mut result = Matrix::new(ncol, nrow);
        for i in 0..min(nrow, ncol){
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
    
    /// Выводит матрицу на экран
    pub fn print(&self) {
        for row in 0..self.nrow{
            for col in 0..self.ncol{
                print!("{} ", self.get(row,col));
            }
            println!();
        }
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
}

// Умножение матриц
fn matrix_mul(m1: &Matrix, m2: &Matrix) -> Matrix{
    
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

// Вычитание матриц
fn matrix_sub(m1: &Matrix, m2: &Matrix) -> Matrix{
    
    if (m1.nrow != m2.nrow) || (m1.ncol != m2.ncol){
        panic!("Размерности матриц не совпадают {}x{} != {}x{}", m1.nrow,m1.ncol, m2.nrow,m2.ncol);
    }
    
    let mut result = Matrix::new(m1.nrow, m2.ncol);
    for index in 0..result.m.len(){
        result.m[index] = m1.m[index] - m2.m[index];
    }
    result
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
        self.m[index as usize]
    }
    pub fn f(&self, input: &Matrix) -> Matrix{
        let mut output = Matrix::new(input.nrow, input.ncol);
        for i in 0..input.m.len(){
            output.m[i] = self.get(input.m[i]) as Tdata// - 127
        }
        output
    }
}

// Нейросеть.
// слои: входной, скрытый, выходной
// net_01: веса связи входной-скрытый
// net_12: веса связи скрытый-Выходной
pub struct Neuronet{
    //весовые коэффициенты связей Вход - Скрытый слой
    net_01: Matrix,
    //весовые коэффициенты связей Скрытый слой - Выход
    net_12: Matrix,
}

impl Neuronet{
    pub fn new(nnodes0: usize, nnodes1: usize, nnodes2: usize) -> Neuronet{
        Neuronet{
            net_01: Matrix::new_rand(nnodes0, nnodes1, -127, 127, true),
            net_12: Matrix::new_rand(nnodes1, nnodes2, -127, 127, true),
        }
    }
    
    // Значение выходного сигнала для значения входного сигнала
    pub fn getoutput(&self, input: &Matrix, sigmoida: &Sigmoida) -> Matrix {
        println!("вход нейросети:");
        input.print();
        let nodes1_input = matrix_mul(input, &self.net_01);
        println!("вход скрытого слоя:");
        nodes1_input.print();
        //div_matrix_elements(nodes1_input, 256);
        let nodes1_output = sigmoida.f(&nodes1_input); 
        println!("выход скрытого слоя:");
        nodes1_output.print();
        let nodes2_input = matrix_mul(&nodes1_output, &self.net_12);
        println!("вход последнего слоя:");
        nodes2_input.print();
        let nodes2_output = sigmoida.f(&nodes2_input); 
        println!("выход нейросети:");
        nodes2_output.print();
        nodes2_output
    }
    
    // Корректировка весовых коэффициентов связей
    fn correkt_net(&self, output: &Matrix, target: &Matrix){
        
        if (output.nrow != target.nrow) || (output.ncol != target.ncol){
            panic!("Размерности матриц не совпадают {}x{} != {}x{}", output.nrow,output.ncol, target.nrow,target.ncol);
        }

        //ошибки выходного слоя
        let output_errors = matrix_sub(&target, &output);
        println!("Ошибки выходного слоя:");
        output_errors.print();
        
        //ошибки скрытого слоя:
        let hidden_errors = matrix_mul(&self.net_12, &output_errors.t())
            .t();
        println!("Ошибки скрытого слоя:");
        hidden_errors.print();
        
    }
    
    pub fn training(&self){
    }
}

fn main() {
    
    let sigmoida = Sigmoida::new();
    
//     let mut test = Matrix::new(1,3);
//     test.set(0,0,45368);
//     sigmoida.f(&test).print();
//     test.set(0,1,-7675);
//     sigmoida.f(&test).print();
//     test.set(0,2,42515);
//     sigmoida.f(&test).print();
    
    let neuronet = Neuronet::new(2,6,3);
    println!("net_01:");
    neuronet.net_01.print();
    println!("net_12:");
    neuronet.net_12.print();
    println!("-----------");
    
    let mut inputdata_1 = Matrix::new(1,2);
//     let mut inputdata_2 = Matrix::new(1,2,0);
//     let mut inputdata_3 = Matrix::new(1,2,0);
    
    inputdata_1.set(0,0,255);
    println!("вход:");
    inputdata_1.print();
    let mut need_output_1 = Matrix::new(1,3);
    need_output_1.set(0,0,255);
    println!("требуемый выход:");
    need_output_1.print();
    
//     inputdata_2.set(0,0,0);
//     inputdata_2.set(0,1,255);
// 
//     inputdata_3.set(0,0,255);
//     inputdata_3.set(0,1,255);
    
    let outputdata_1 = neuronet.getoutput(&inputdata_1, &sigmoida);
    neuronet.correkt_net(&outputdata_1, &need_output_1);
//     let outputdata_2 = neuronet.getoutput(&inputdata_2);
//     let outputdata_3 = neuronet.getoutput(&inputdata_3);
    
}
