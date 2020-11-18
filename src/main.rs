
/// Массив произвольного типа размером 10
/// # Examples
///
/// ```
/// let mut m = Massiv::new(0);
/// m.set(2, 5);
/// m.prt();
/// ```
pub struct Massiv<T>{
    m: [T; 10],
}

impl Massiv<i16> {
    /// Инициализация массива и заполнение всех элементов заданным значением
    pub fn new(x: i16) -> Massiv<i16> {
        Massiv {m: [x; 10]}
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
    pub fn prt(&self) {
        for x in &self.m{
            print!("{} ", x);
        }
        println!();
    }
}

impl Copy for Massiv<i16> { }

impl Clone for Massiv<i16> {
    fn clone(&self) -> Massiv<i16> {
        *self
    }
}

/// Матрица произвольного типа размером 10x10
/// # Examples
///
/// ```
/// let mut m = Matrix::new(0);
/// m.set(2, 3, 5);
/// m.prt();
/// ```
pub struct Matrix<T>{
    m: [Massiv<T>; 10]
}

impl Matrix<i16>{
    /// Инициализация матрицы и заполнение всех элементов заданным значением
    pub fn new(x: i16) -> Matrix<i16> {
        Matrix {m: [Massiv::new(x);10] }
    }
    /// Установка значения x в ячейку (i,j)
    pub fn set(&mut self, i:usize, j:usize, x: i16) {
        self.m[j].set(i, x);
    }
    /// Вывод матрицы на экран
    pub fn prt(&self) {
        for x in &self.m{
            x.prt();
        }
    }
}

/// Функция, просто возвращающая переданное значение
pub fn foo(x: i32) -> i32 {x}


fn main() {

    let mut m = Matrix::new(1);
    m.set(2, 3, 5);
    m.prt();
    
    let x: fn(i32)->i32 = foo;
    
    println!("{}", x(4));
    
}
