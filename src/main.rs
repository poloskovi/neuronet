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

// Простое двоичное преобразование.
// Научим нейросеть преобразовывать входящий двоичный входящий сигнал в десятичное число!
fn test_binary_to_decimal() {

    let sigmoida = matrix::Sigmoida::new();
    
    let n_input = 4; // количество входных сигналов
    let n_output = 10; // количество выходных сигналов
    let n_hidden = 200; // количество узлов скрытого слоя
    let mut neuronet = Neuronet::new(n_input, n_hidden, n_output);
    
    let mut inputdata_0 = matrix::Matrix::new(1,n_input);
    let mut inputdata_1 = matrix::Matrix::new(1,n_input);
    let mut inputdata_2 = matrix::Matrix::new(1,n_input);
    let mut inputdata_3 = matrix::Matrix::new(1,n_input);
    let mut inputdata_4 = matrix::Matrix::new(1,n_input);
    let mut inputdata_5 = matrix::Matrix::new(1,n_input);
    let mut inputdata_6 = matrix::Matrix::new(1,n_input);
    let mut inputdata_7 = matrix::Matrix::new(1,n_input);
    let mut inputdata_8 = matrix::Matrix::new(1,n_input);
    let mut inputdata_9 = matrix::Matrix::new(1,n_input);
    let mut inputdata_10 = matrix::Matrix::new(1,n_input);
    
    let mut need_output_0 = matrix::Matrix::new(1,n_output);
    let mut need_output_1 = matrix::Matrix::new(1,n_output);
    let mut need_output_2 = matrix::Matrix::new(1,n_output);
    let mut need_output_3 = matrix::Matrix::new(1,n_output);
    let mut need_output_4 = matrix::Matrix::new(1,n_output);
    let mut need_output_5 = matrix::Matrix::new(1,n_output);
    let mut need_output_6 = matrix::Matrix::new(1,n_output);
    let mut need_output_7 = matrix::Matrix::new(1,n_output);
    let mut need_output_8 = matrix::Matrix::new(1,n_output);
    let mut need_output_9 = matrix::Matrix::new(1,n_output);
    let mut need_output_10 = matrix::Matrix::new(1,n_output);
    
    let max = 255;
    
    // 0000 = 0
    {
        let inputvalue_zero = 50; // нейросеть не может обработать вход, состоящий только из нулей
        inputdata_0.set(0,3,0);
        inputdata_0.set(0,2,0);
        inputdata_0.set(0,1,0);
        inputdata_0.set(0,0,inputvalue_zero);
    }
    
    need_output_0.set(0,0,max);
    
    // 0001 = 1
    inputdata_1.set(0,3,0);
    inputdata_1.set(0,2,0);
    inputdata_1.set(0,1,0);
    inputdata_1.set(0,0,max);
    
    need_output_1.set(0,1,max);
    
    // 0010 = 2
    inputdata_2.set(0,3,0);
    inputdata_2.set(0,2,0);
    inputdata_2.set(0,1,max);
    inputdata_2.set(0,0,0);
    
    need_output_2.set(0,2,max);

    // 0011 = 3
    inputdata_3.set(0,3,0);
    inputdata_3.set(0,2,0);
    inputdata_3.set(0,1,max);
    inputdata_3.set(0,0,max);
    
    need_output_3.set(0,3,max);
    
    // 0100 = 4
    inputdata_4.set(0,3,0);
    inputdata_4.set(0,2,max);
    inputdata_4.set(0,1,0);
    inputdata_4.set(0,0,0);
    
    need_output_4.set(0,4,max);

    // 0101 = 5
    inputdata_5.set(0,3,0);
    inputdata_5.set(0,2,max);
    inputdata_5.set(0,1,0);
    inputdata_5.set(0,0,max);
    
    need_output_5.set(0,5,max);

    // 0110 = 6
    inputdata_6.set(0,3,0);
    inputdata_6.set(0,2,max);
    inputdata_6.set(0,1,max);
    inputdata_6.set(0,0,0);
    
    need_output_6.set(0,6,max);

    // 0111 = 7
    inputdata_7.set(0,3,0);
    inputdata_7.set(0,2,max);
    inputdata_7.set(0,1,max);
    inputdata_7.set(0,0,max);
    
    need_output_7.set(0,7,max);

    // 1000 = 8
    inputdata_8.set(0,3,max);
    inputdata_8.set(0,2,0);
    inputdata_8.set(0,1,0);
    inputdata_8.set(0,0,0);
    
    need_output_8.set(0,8,max);

    // 1001 = 9
    inputdata_9.set(0,3,max);
    inputdata_9.set(0,2,0);
    inputdata_9.set(0,1,0);
    inputdata_9.set(0,0,max);
    
    need_output_9.set(0,9,max);

    // 1010 = 10
    inputdata_10.set(0,3,max);
    inputdata_10.set(0,2,0);
    inputdata_10.set(0,1,max);
    inputdata_10.set(0,0,0);
    
    need_output_10.set(0,0,max);
    need_output_10.set(0,1,max);

    for _i in 0..20 {
        neuronet.training(&inputdata_0, &need_output_0, &sigmoida);
        neuronet.training(&inputdata_1, &need_output_1, &sigmoida);
        neuronet.training(&inputdata_2, &need_output_2, &sigmoida);
        neuronet.training(&inputdata_3, &need_output_3, &sigmoida);
        neuronet.training(&inputdata_4, &need_output_4, &sigmoida);
        neuronet.training(&inputdata_5, &need_output_5, &sigmoida);
        neuronet.training(&inputdata_6, &need_output_6, &sigmoida);
        neuronet.training(&inputdata_7, &need_output_7, &sigmoida);
        neuronet.training(&inputdata_8, &need_output_8, &sigmoida);
        neuronet.training(&inputdata_9, &need_output_9, &sigmoida);
        neuronet.training(&inputdata_10, &need_output_10, &sigmoida);
    }
    
    println!("Матрица весов связей Вход - Скрытый слой:");
    println!("{}", neuronet.net_01);
    println!("Матрица весов связей Скрытый слой - Выход:");
    println!("{}", neuronet.net_12);
    
    println!("Выходные значения нейросети для различных входов:");
    print!(" 0: {}", neuronet.getoutput(&inputdata_0, &sigmoida));
    print!(" 1: {}", neuronet.getoutput(&inputdata_1, &sigmoida));
    print!(" 2: {}", neuronet.getoutput(&inputdata_2, &sigmoida));
    print!(" 3: {}", neuronet.getoutput(&inputdata_3, &sigmoida));
    print!(" 4: {}", neuronet.getoutput(&inputdata_4, &sigmoida));
    print!(" 5: {}", neuronet.getoutput(&inputdata_5, &sigmoida));
    print!(" 6: {}", neuronet.getoutput(&inputdata_6, &sigmoida));
    print!(" 7: {}", neuronet.getoutput(&inputdata_7, &sigmoida));
    print!(" 8: {}", neuronet.getoutput(&inputdata_8, &sigmoida));
    print!(" 9: {}", neuronet.getoutput(&inputdata_9, &sigmoida));
    print!("10: {}", neuronet.getoutput(&inputdata_10, &sigmoida));
    
}

// Попробуем делать разный выход в случае одного набора входов разной интенсивности
fn test_different_input_levels() {

    let sigmoida = matrix::Sigmoida::new();
    
    let n_input = 2; // количество входных сигналов
    let n_output = 3; // количество выходных сигналов
    let n_hidden = 5; // количество узлов скрытого слоя
    let mut neuronet = Neuronet::new(n_input, n_hidden, n_output);
    
    let mut inputdata_0 = matrix::Matrix::new(1,n_input);
    let mut inputdata_1 = matrix::Matrix::new(1,n_input);
    let mut inputdata_2 = matrix::Matrix::new(1,n_input);
    
    let mut need_output_0 = matrix::Matrix::new(1,n_output);
    let mut need_output_1 = matrix::Matrix::new(1,n_output);
    let mut need_output_2 = matrix::Matrix::new(1,n_output);
    
    inputdata_0.set(0,0,50);
    need_output_0.set(0,0,252);

    inputdata_1.set(0,0,100);
    need_output_1.set(0,1,252);
    
    inputdata_2.set(0,0,252);
    need_output_2.set(0,2,252);
    
    for _i in 0..1500 {
        neuronet.training(&inputdata_0, &need_output_0, &sigmoida);
        neuronet.training(&inputdata_1, &need_output_1, &sigmoida);
        neuronet.training(&inputdata_2, &need_output_2, &sigmoida);
    }
    
    println!("Матрица весов связей Вход - Скрытый слой:");
    println!("{}", neuronet.net_01);
    println!("Матрица весов связей Скрытый слой - Выход:");
    println!("{}", neuronet.net_12);
    
    println!("Выходные значения нейросети для различных входов:");
    print!(" 0: {}", neuronet.getoutput(&inputdata_0, &sigmoida));
    print!(" 1: {}", neuronet.getoutput(&inputdata_1, &sigmoida));
    print!(" 2: {}", neuronet.getoutput(&inputdata_2, &sigmoida));
}

fn main() {
    
//     test_binary_to_decimal();
    test_different_input_levels();
    
}
