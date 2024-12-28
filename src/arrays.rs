use std::mem;

pub fn empty_arrays(){
    let empty_array: [u32;3] = [1,2,3];
    println!("Array occupies {} bytes", mem::size_of_val(&empty_array));

    let mut vec = empty_array.to_vec();

    vec.splice(0..0,[1,2,3,4]);

    println!("{:?}", vec)
}