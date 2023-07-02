use rand::prelude::*;

const DATA:[[u8;3];4] = [
    [0,0,0],
    [0,1,1],
    [1,0,1],
    [1,1,1]
];
fn cost(w:&[f64],b:&[f64]) -> f64{
    let mut result:f64 = 0.0;
    for i in 0..DATA.len(){
        result += DATA[i][0] as f64 *w[0] +DATA[i][1] as f64 *w[1] + b[0];
    }
    return result/DATA.len() as f64;
} 
fn main() {
    let mut random_float = rand::thread_rng();
    let w = [random_float.gen::<f64>(),random_float.gen::<f64>()];
    let b=[random_float.gen::<f64>()];
    println!("{}",cost(&w,&b));
    
}
