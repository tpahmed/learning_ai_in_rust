use std::fmt::{Formatter,Result,Debug};

use rand::prelude::*;
use libm::exp;


struct Neuron {
    a:f64,
    w:Vec<f64>,
    b:f64,
}
struct Layer {
    neurons:Vec<Neuron>
}
struct NeuralNetwork {
    layers:Vec<Layer>,
    model:Vec<usize>
}

impl NeuralNetwork{
    pub fn new(model: &Vec<usize>) -> Self{
        let mut layers:Vec<Layer> = vec![]; 
        for layer in 0..model.len(){
            let mut neurons:Vec<Neuron> = vec![];
            for neuron in 0..model[layer]{
                if layer == 0 { 
                    neurons.push(Neuron {a:0.0,w:vec![],b:0.0});
                    continue;
                }
                let mut w:Vec<f64> = vec![];
                let b:f64 = random::<f64>();
                for weight in 0..model[layer-1]{
                    let new_weight:f64 = random::<f64>();
                    w.push(new_weight);
                }
                neurons.push(Neuron {a:0.0,w,b});
                
            }

            layers.push(Layer { neurons }); 
        }

        Self { layers,model:model.clone() }
    }

    pub fn forward(&mut self){
        for layer in 1..self.layers.len(){
            for neuron in 0..self.layers[layer].neurons.len(){
                self.layers[layer].neurons[neuron].a = 0.0;
                for weight in 0..self.layers[layer].neurons[neuron].w.len(){
                    self.layers[layer].neurons[neuron].a += self.layers[layer-1].neurons[weight].a * self.layers[layer].neurons[neuron].w[weight];
                }
                self.layers[layer].neurons[neuron].a += self.layers[layer].neurons[neuron].b;
                self.layers[layer].neurons[neuron].a = sigmoid(self.layers[layer].neurons[neuron].a);
            }
        }
    }

    pub fn set_inputs(&mut self,inputs:&Vec<f64>){
        assert!(inputs.len()==self.layers[0].neurons.len());
        for input in 0..inputs.len(){
            self.layers[0].neurons[input].a = inputs[input];
        }
    }
    pub fn get_outputs(&self) -> f64{
        self.layers[self.layers.len()-1].neurons[0].a
    }

    pub fn cost(&mut self) -> f64{
        let mut result:f64 = 0.0;
        for i in 0..DATA.len(){
            let inputs = vec![DATA[i][0] as f64,DATA[i][1] as f64];
            self.set_inputs(&inputs);
            self.forward();
            result += (self.get_outputs() - DATA[i][2] as f64).powf(2.0);
        }
        return result/DATA.len() as f64;
    }
    pub fn finite_diff(&mut self,rate:&f64){
        let mut difference_nn = NeuralNetwork::new(&self.model);
        let amt = 0.01;
        for layer in 0..self.layers.len(){
            for neuron in 0..self.layers[layer].neurons.len(){
                for weight in 0..self.layers[layer].neurons[neuron].w.len(){
                    let save = self.layers[layer].neurons[neuron].w[weight].clone();
                    let cost1 = self.cost();
                    self.layers[layer].neurons[neuron].w[weight]+=amt;
                    let cost2 = self.cost();
                    difference_nn.layers[layer].neurons[neuron].w[weight] = (cost2 - cost1)/amt;
                    self.layers[layer].neurons[neuron].w[weight] = save;
                }
                let save = self.layers[layer].neurons[neuron].b.clone();
                let cost1 = self.cost();
                self.layers[layer].neurons[neuron].b+=amt;
                let cost2 = self.cost();
                difference_nn.layers[layer].neurons[neuron].b = (cost2 - cost1)/amt;
                self.layers[layer].neurons[neuron].b = save;
            }
        }
        for layer in 0..self.layers.len(){
            for neuron in 0..self.layers[layer].neurons.len(){
                for weight in 0..self.layers[layer].neurons[neuron].w.len(){
                    self.layers[layer].neurons[neuron].w[weight] -= rate*difference_nn.layers[layer].neurons[neuron].w[weight];
                }
                self.layers[layer].neurons[neuron].b -= rate*difference_nn.layers[layer].neurons[neuron].b;
            }
        }
        

    }

}
impl Debug for NeuralNetwork{
    fn fmt(&self,f:&mut Formatter) -> Result {
        write!(f, "\n{:?}", self.layers)
    }
}
impl Debug for Layer{
    fn fmt(&self,f:&mut Formatter) -> Result {
        write!(f, "\n\t {:?}", self.neurons)
    }
}
impl Debug for Neuron{
    fn fmt(&self, f:&mut Formatter) -> Result {
        write!(f, "\n\t\t a : {}, w : {:?}, b : {}", self.a, self.w, self.b)
    }
}

const DATA:[[u32;3];4] = [
    [0,0,0],
    [0,1,1],
    [1,0,1],
    [1,1,0]
];

const RATE:f64 = 0.1;

fn sigmoid(x:f64) -> f64{
    1.0/(1.0 + exp(-x))
}

fn use_nn(nn:&mut NeuralNetwork){
    for i in DATA{
        let inputs = vec![i[0] as f64,i[1] as f64];
        nn.set_inputs(&inputs);
        nn.forward();
        println!("x: {}, y: {}, output: {}",i[0],i[1],nn.get_outputs());
    }
}

fn main() {
    let model = vec![2,2,1];
    let mut nn1 = NeuralNetwork::new( &model);
    println!("cost : {}",nn1.cost());
    for i in 1..=100_000{
        nn1.finite_diff(&RATE);
        // println!("{} - cost : {}",i,nn1.cost());
    }
    println!("---------------------\ncost : {}\n---------------------",nn1.cost());
    use_nn(&mut nn1);
    let inputs = vec![1.0,0.0];
    nn1.set_inputs(&inputs);
    dbg!(&nn1);
    nn1.forward();
    dbg!(&nn1);
    
    
}
