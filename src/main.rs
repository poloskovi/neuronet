
/// Массив типа i16 размером 10
/// # Examples
///
/// ```
/// let mut m = Massiv::new(0);
/// m.set(2, 5);
/// m.prt();
/// ```
#[derive(Clone, Copy)]
pub struct Massiv{
    m: [i16; 10],
}

impl Massiv {
    /// Инициализация массива и заполнение всех элементов заданным значением
    pub fn new(x: i16) -> Massiv {
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

/// Матрица типа i16 размером 10x10
/// # Examples
///
/// ```
/// let mut m = Matrix::new(0);
/// m.set(2, 3, 5);
/// m.prt();
/// ```
pub struct Matrix{
    m: [Massiv; 10]
}

impl Matrix{
    /// Инициализация матрицы и заполнение всех элементов заданным значением
    pub fn new(x: i16) -> Matrix {
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

fn main() {

    let mut m = Matrix::new(1);
    m.set(2, 3, 5);
    m.set(3, 3, 6);
    m.prt();
    
}
