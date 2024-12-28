#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Matrix {
    fn transpose(&mut self) {
        let temp_value = self.1;
        self.1 = self.2;
        self.2 = temp_value;


    }
    
}

pub fn matrix(){
    
    let mut matrix = Matrix(1.1, 1.2, 2.1, 2.2);


    println!("Matrix:\n{:?}",matrix) ;

    matrix.transpose();
    println!("Transposed:\n{:?}", matrix);

}

