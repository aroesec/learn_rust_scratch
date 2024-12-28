#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

pub fn scratch(){
    

    let pair = (1,true);


    let too_long_tuple = (1, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,14);

    println!("Too long tuple: {:?}", too_long_tuple.0 )
}


fn reverse(pair: (i32,bool)) -> (bool,i32) {
    let (int_param, bool_param) = pair;

    (bool_param,int_param)
}



