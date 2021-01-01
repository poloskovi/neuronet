mod matrix;

// Нейросеть.
pub struct Neuronet{
    //весовые коэффициенты связей слоев
    net: Vec<matrix::Matrix>
}

impl Neuronet{
    
    // nnodes - вектор количества ячеек в слоях
    pub fn new(nnodes: &Vec<usize>) -> Neuronet{
        let mut net = Vec::<matrix::Matrix>::new();
        for i in 0..nnodes.len()-1 {
            // весовые коэффициенты связи слоев (i) и (i+1)
            net.push(matrix::Matrix::new_rand(nnodes[i], nnodes[i+1], -127, 127, true)); 
        };
        Neuronet{
            net: net,
        }
    }
    
    // Значение выходного сигнала для значения входного сигнала
    pub fn getoutput(&self, input: &matrix::Matrix, sigmoida: &matrix::Sigmoida) -> matrix::Matrix {
    
        let mut next = matrix::Matrix::new(1, 1); // фиктивное значение, чтобы компилятор не ругался на возможно неинициализированную переменную
        for i in 0..self.net.len() {
            next = matrix::Matrix::mul_and_sigmoida(
                match i {
                    0 => input,
                    _ => &next,
                }
                , &self.net[i], sigmoida
            ); 
        }
        next
    }
    
    // Тренировка
    fn training(&mut self, input: &matrix::Matrix, target: &matrix::Matrix, sigmoida: &matrix::Sigmoida){
        
        // Получение выходных значений на каждом слое
        
        // количество матриц в нейросети
        let n_layers = self.net.len();
        
        // максимальный индекс матриц
        let index_layer_max = n_layers-1; 
        
        // выходные сигналы на каждом слое
        // для 2 скрытых слоев - 3 элемента
        //let mut outputs = Vec::<&matrix::Matrix>::with_capacity(n_layers); 
        let mut outputs  = Vec::<matrix::Matrix>::new();
        
        for i in 0..self.net.len() {
            outputs.push( 
                matrix::Matrix::mul_and_sigmoida(
                    match i {
                        0 => input,
                        _ => &outputs[i-1],
                    }
                , &self.net[i], sigmoida)
            ); 
        }

        // проверка, что размер выходного сигнала совпадает с размером цели
        if (outputs[index_layer_max].nrow != target.nrow) || (outputs[index_layer_max].ncol != target.ncol){
            panic!("Размерности матриц не совпадают {}x{} != {}x{}", 
                outputs[index_layer_max].nrow, outputs[index_layer_max].ncol, target.nrow, target.ncol);
        }
        
        // Корректировка весов связей нейросети
        let mut error = matrix::Matrix::new(1, 1); // фиктивное значение, чтобы компилятор не ругался на возможно неинициализированную переменную
        for i in 0..self.net.len() {
            let index = self.net.len() - i - 1;
            error = 
                match i {
                    0 => matrix::Matrix::sub(target, &outputs[index_layer_max]),
                    _ => matrix::Matrix::mul(&self.net[index+1], &error.t()).t(),
                };
            let m1 = matrix::Matrix::m1_correctnet(&error, &outputs[index]);
            let delta = matrix::Matrix::mul(&m1.t(), 
                match index {
                    0 => input,
                    _ => &outputs[index-1],
                }
            ).t();
            self.net[index] = matrix::Matrix::add(&self.net[index], &delta);
        }

    }
}

// Простое двоичное преобразование.
// Научим нейросеть преобразовывать входящий двоичный входящий сигнал в десятичное число!
fn test_binary_to_decimal() {

    println!("Пример: двоичное преобразование.");

    let sigmoida = matrix::Sigmoida::new();
    
    let n_input = 4; // количество входных сигналов
    let n_output = 10; // количество узлов скрытого слоя
//     let n_hidden_1 = 20; // количество узлов скрытого слоя
//     let n_hidden_2 = 100; // количество узлов скрытого слоя

    let mut neuronet = Neuronet::new(&vec![n_input, 20, 100, n_output]);
    //let mut neuronet = Neuronet::new(&vec![n_input, 20, n_output]);
    
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

    for _i in 0..40 {
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
    
//     println!("Матрица весов связей Вход - Скрытый слой:");
//     println!("{}", neuronet.net_01);
//     println!("Матрица весов связей Скрытый слой - Скрытый слой:");
//     println!("{}", neuronet.net_12);
//     println!("Матрица весов связей Скрытый слой - Выход:");
//     println!("{}", neuronet.net_23);
    
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

    println!("Пример: разный выход в случае одного набора входов разной интенсивности");

    let sigmoida = matrix::Sigmoida::new();
    
    let n_input = 2; // количество входных сигналов
    let n_output = 3; // количество выходных сигналов
    //let n_hidden = 10; // количество узлов скрытого слоя
    let mut neuronet = Neuronet::new(&vec![n_input, 10, 30, 15, n_output]);
    
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
    
//     println!("Матрица весов связей Вход - Скрытый слой:");
//     println!("{}", neuronet.net_01);
//     println!("Матрица весов связей Скрытый слой - Скрытый слой:");
//     println!("{}", neuronet.net_12);
//     println!("Матрица весов связей Скрытый слой - Выход:");
//     println!("{}", neuronet.net_23);
    
    println!("Выходные значения нейросети для различных входов:");
    print!(" 50: {}", neuronet.getoutput(&inputdata_0, &sigmoida));
    print!("100: {}", neuronet.getoutput(&inputdata_1, &sigmoida));
    print!("252: {}", neuronet.getoutput(&inputdata_2, &sigmoida));
}

fn main() {
    
    test_binary_to_decimal();
    test_different_input_levels();
    
}
