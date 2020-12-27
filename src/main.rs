// На выходе узла сети сигнал в целых числах (+/-), Uвых(i)
// Выходной сигнал умножается на значение матрицы: Uвых(i) * A(i,j)
// Входным сигналом следующего слоя является сигмоида сумм выходных сигналов предыдущего слоя, умноженных на матрицу значений:
// sigm(СУММ(Uвых(i) * A(i,j)))
// Это значение выходного сигнала ячейки

mod matrix;

// Нейросеть.
// слои: входной, скрытый, выходной
// net_01: веса связи входной-скрытый
// net_12: веса связи скрытый-Выходной
pub struct Neuronet{
    //весовые коэффициенты связей Вход - Скрытый слой
    net_01: matrix::Matrix,
    //весовые коэффициенты связей Скрытый слой - Выход
    net_12: matrix::Matrix,
}

impl Neuronet{
    
    pub fn new(nnodes0: usize, nnodes1: usize, nnodes2: usize) -> Neuronet{
        Neuronet{
            net_01: matrix::Matrix::new_rand(nnodes0, nnodes1, -127, 127, true),
            net_12: matrix::Matrix::new_rand(nnodes1, nnodes2, -127, 127, true),
        }
    }
    
    // Значение выходного сигнала для значения входного сигнала
    pub fn getoutput(&self, input: &matrix::Matrix, sigmoida: &matrix::Sigmoida) -> matrix::Matrix {
        let hidden_output = matrix::Matrix::mul_and_sigmoida(input, &self.net_01, sigmoida); 
        let output = matrix::Matrix::mul_and_sigmoida(&hidden_output, &self.net_12, sigmoida); 
        output
    }
    
//     pub fn output_index(&self, input: &matrix::Matrix, sigmoida: &matrix::Sigmoida) -> usize {
//         let output = self.getoutput(input, sigmoida);
//         let mut max = output.m[0];
//         let mut imax = 0;
//         for i in 1..output.m.len(){
//             if output.m[i]>max {
//                 max = output.m[i];
//                 imax = i;
//             }
//         }
//         imax
//     }
    
    // Тренировка
    fn training(&mut self, input: &matrix::Matrix, target: &matrix::Matrix, sigmoida: &matrix::Sigmoida){
        
        // Получение выходного значения
        
        let hidden_output = matrix::Matrix::mul_and_sigmoida(input, &self.net_01, sigmoida); 
        let output = matrix::Matrix::mul_and_sigmoida(&hidden_output, &self.net_12, sigmoida); 
        
        // Корректировка весов связей
        
        if (output.nrow != target.nrow) || (output.ncol != target.ncol){
             panic!("Размерности матриц не совпадают {}x{} != {}x{}", 
                    output.nrow, output.ncol, target.nrow, target.ncol);
        }

        let output_errors = matrix::Matrix::sub(&target, &output);
        let hidden_errors = matrix::Matrix::mul(&self.net_12, &output_errors.t())
            .t();
        
        let m1 = matrix::Matrix::m1_correctnet(&output_errors, &output);
        let delta_net_12 = matrix::Matrix::mul(&m1.t(), &hidden_output).t();
        
        self.net_12 = matrix::Matrix::add(&self.net_12, &delta_net_12);
        
        let m1 = matrix::Matrix::m1_correctnet(&hidden_errors, &hidden_output);
        let delta_net_01 = matrix::Matrix::mul(&m1.t(), &input).t();
        
        self.net_01 = matrix::Matrix::add(&self.net_01, &delta_net_01);
        
    }
    
}

fn main() {
    
    let sigmoida = matrix::Sigmoida::new();
    
    let mut neuronet = Neuronet::new(3,100,4);
    
    let mut inputdata_1 = matrix::Matrix::new(1,3);
    let mut inputdata_2 = matrix::Matrix::new(1,3);
    let mut inputdata_3 = matrix::Matrix::new(1,3);
    let mut inputdata_4 = matrix::Matrix::new(1,3);
    
    let max = 255;
    
    inputdata_1.set(0,0,max);
    let mut need_output_1 = matrix::Matrix::new(1,4);
    need_output_1.set(0,0,max);
    
    inputdata_2.set(0,1,max);
    let mut need_output_2 = matrix::Matrix::new(1,4);
    need_output_2.set(0,1,max);

    inputdata_3.set(0,0,max);
    inputdata_3.set(0,1,max);
    let mut need_output_3 = matrix::Matrix::new(1,4);
    need_output_3.set(0,2,max);
    
    inputdata_4.set(0,2,max);
    let mut need_output_4 = matrix::Matrix::new(1,4);
    need_output_4.set(0,3,max);

    for _i in 0..20 {
        neuronet.training(&inputdata_1, &need_output_1, &sigmoida);
        neuronet.training(&inputdata_2, &need_output_2, &sigmoida);
        neuronet.training(&inputdata_3, &need_output_3, &sigmoida);
        neuronet.training(&inputdata_4, &need_output_4, &sigmoida);
    }
    
    println!("{}", neuronet.net_01);
    println!("{}", neuronet.net_12);
    
    println!("0: {}", neuronet.getoutput(&inputdata_1, &sigmoida));
    println!("1: {}", neuronet.getoutput(&inputdata_2, &sigmoida));
    println!("2: {}", neuronet.getoutput(&inputdata_3, &sigmoida));
    println!("4: {}", neuronet.getoutput(&inputdata_4, &sigmoida));
    
}
