use std::any::type_name;
fn f1() {
    //use type_of
    // type checking
    let x = 12;
    let y = 2.5;
    println!("type is {}", type_of(y));
    println!("type is {}", type_of(x));
}
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
//-------------------------------------------------
use std::any::Any;
trait Object {
    fn type_name_2(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}
fn type_name_2(x: &dyn Object) -> &str {
    x.type_name_2()
}
fn is_of_type<T: 'static>(x: &dyn Object) -> bool {
    x.as_any().is::<T>()
}
impl Object for i32 {
    fn type_name_2(&self) -> &str {
        "i32"
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn f2() {
    // type checking
    let x = 21;
    println!("f2");
    println!("{}", type_name_2(&x));
    println!("{}", is_of_type::<i32>(&x));
}

//----------------------------------------------
fn f3() {
    // vectors
    let vec1 = [1, 2, 3, 4];
    println!("vec1 is {}", type_of(vec1));
}
//---------------------------------------------
use std::collections::HashMap;
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    /// find two number in 'nums' vector sum of these are target...
    /// time complexity O(n), memory O(n)
    /// ret empty vector means not find.
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (i, v) in nums.iter().enumerate() {
        match map.get(&(target - *v)) {
            Some(&i2) => return vec![i as i32, i2],
            None => map.insert(*v, i as i32),
        };
    }

    return vec![];
}

fn f4() {
    println!("{:?}", two_sum(vec![3, 2, 4], 6))
}
//------------------------------------------------
//enum StatusToDo {
//    ToDo,
//    Doing,
//    Done,
//}

//fn f5(status: StatusToDO) {
//   match status {
//      StatusToDo::ToDo => println!("to do"),
//     StatusToDo::Doing => println!("doing"),
// }
//}
//---------------------------------------
type Matrix = [[u64; 2]; 2];

fn multiply_matrices(a: Matrix, b: Matrix) -> Matrix {
    let mut result = [[0; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            result[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j];
        }
    }
    result
}

fn matrix_power(mut matrix: Matrix, mut n: u64) -> Matrix {
    let mut result = [[1, 0], [0, 1]]; // Identity matrix
    while n > 0 {
        if n % 2 == 1 {
            result = multiply_matrices(result, matrix);
        }
        matrix = multiply_matrices(matrix, matrix);
        n /= 2;
    }
    result
}

fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let fib_matrix = [[1, 1], [1, 0]];
    let result = matrix_power(fib_matrix, n - 1);
    result[0][0]
}
fn f6(n: u64) {
    println!("Fibonacci of {} is {}", n, fibonacci(n));
}
//---------------------------------------
fn sigmoid(x: f64) -> f64 {
    // \sigma(x) = \frac{1}{1 + e^{-x}}
    1.0 / (1.0 + (-x).exp())
}
fn f7() {
    println!("{}", sigmoid(0.5));
}
//-----------------------------
fn f8() {
    // i32 -> two's complements 2^(32-1)<... <2^(32-1)-1
    //let y:i32=4294967296;
    let mut x: i32 = 2147483647;
    println!("{}", x);
    x = -2147483648;
    println!("{}", x);
}
//------------------------
use std::u8;
fn f9() {
    // Example: Handling Overflow Using Checked Arithmetic
    //
    // Checked Arithmetic: Use methods that return Option to handle overflow.
    let a1: u8 = std::u8::MAX;
    println!("{}", a1);
    let a2: u8 = 1;
    match a1.checked_add(a2) {
        Some(result) => println!("the sum is {}", result),
        None => println!("overflow"),
    }
    //Wrapping Arithmetic: Operations that wrap around on overflow.
    println!("iwrapped result: {}", a1.wrapping_add(a2));

    //Saturating Arithmetic: Operations that saturate at the numeric bounds on overflow.
    let result = a1.saturating_add(a2);
    println!("Saturated result: {}", result);

    // Panic on Overflow: The default behavior in debug mode is to panic on overflow.
    //println!("{}",a1+a2);
}
//---------------------------
//fn f10()...
//1. Using tensorflow-rust Library
//[dependencies]
//tensorflow = "0.16"
//extern crate tensorflow;
//
//use tensorflow::{Graph, ImportGraphDefOptions, Session, SessionOptions, Tensor};
//
//fn main() -> Result<(), Box<dyn std::error::Error>> {
//    // Load the pre-trained model from a file
//    let mut graph = Graph::new();
//    let model_data = include_bytes!("path_to_your_model.pb");
//    graph.import_graph_def(model_data, &ImportGraphDefOptions::new())?;
//
//    // Create a session for running the model
//    let session = Session::new(&SessionOptions::new(), &graph)?;
//
//    // Create input tensor
//    let input = Tensor::new(&[1, 28, 28, 1])
//        .with_values(&[0.0f32; 28 * 28])?; // Example input tensor (for a 28x28 image)
//
//    // Define the input and output operations
//    let input_op = graph.operation_by_name_required("input_operation_name")?;
//    let output_op = graph.operation_by_name_required("output_operation_name")?;
//
//    // Run the session
//    let mut step = tensorflow::SessionRunArgs::new();
//    step.add_feed(&input_op, 0, &input);
//    let output_token = step.request_fetch(&output_op, 0);
//    session.run(&mut step)?;
//
//    // Get the output
//    let output: Tensor<f32> = step.fetch(output_token)?;
//    println!("Model output: {:?}", output);
//
//    Ok(())
//}
//
//2. Using tract-tensorflow for Pure Rust Approach
//[dependencies]
//tract-tensorflow = "0.15"
//use tract_tensorflow::prelude::*;
//
//fn main() -> TractResult<()> {
//    // Load the TensorFlow model
//    let model = tract_tensorflow::tensorflow()
//        .model_for_path("path_to_your_model.pb")?
//        .with_input_fact(0, InferenceFact::dt_shape(f32::datum_type(), tvec!(1, 28, 28, 1)))?
//        .into_optimized()?
//        .into_runnable()?;
//
//    // Create an input tensor
//    let input = tract_ndarray::Array4::zeros((1, 28, 28, 1)).into_tensor();
//
//    // Run the model
//    let result = model.run(tvec!(input))?;
//
//    // Print the output
//    println!("Model output: {:?}", result);
//
//    Ok(())
//}
//3. Calling TensorFlow via FFI (Foreign Function Interface)
//--------------------------
// fn fibonacci(n: u128) -> u128 {
//     match n {
//         0 => 1,
//         1 => 1,
//         _ => fibonacci(n - 1) + fibonacci(n - 2),
//     }
// }

// fn f10(n: u128) {
//     println!("fibonacci_({}) = {}",n,fibonacci(n));
// }
//-------------------------
// use std::env;
// fn fibonacci(n: u32) -> u32 {
//     match n {
//         0 => 1,
//         1 => 1,
//         _ => fibonacci(n - 1) + fibonacci(n - 2),
//     }
// }
// fn f11() {
//     let args: Vec<String> = env::args().collect();
//     // torbofish ::<> is for set type
//     let x: u32 = args[1].parse().unwrap();
//     println!("Fibonacci({:?})", x);
//     println!("{:?}", fibonacci(x));
// }
//-------------------------
// pub fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// // This is a really bad adding function, its purpose is to fail in this
// // example.
// #[allow(dead_code)]
// fn bad_add(a: i32, b: i32) -> i32 {
//     a - b
// }

// #[cfg(test)]
// mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;

//     #[test]
//     fn test_add() {
//         assert_eq!(add(1, 2), 3);
//     }

//     #[test]
//     fn test_bad_add() {
//         // This assert would fire and test will fail.
//         // Please note, that private functions can be tested too!
//         assert_eq!(bad_add(1, 2), 3);
//     }
// }
//---------------------
// Downloader in rust
// use error_chain::error_chain;
// use std::path::Path;
// use std::fs::File;
// use std::io::prelude::*;

// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         HttpRequest(reqwest::Error);
//     }
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     //println!("Hello, world!");
//     return None;
// }
//---------------------------------
fn main() {
    //f1();
    //f2();
    //f3();
    //101,yew,rachet,rustwasm,actix,orm diesel,tokio async,ffi,bingen/binjen,
    //non blocking, goroutin vs coroutin vs future thread rust,template engins,harfbuzz,
    //concurrency, thread, process, syscall,file,large file,sql/sqlite/postgres/mysql,rfd,rfc,socket,
    // IPC, RPC, GRPC, RESTfull,websocket,cross compile,
    // rust mem manage vs runtime (garbage
    // collector) c++/java... vs manual c/c++,cargo add fft,tokio,realfft
    // doc.rs, tokio full/fs/io-util/...
    // llvm rust,gcc rust,play.rust-lang.org
    // cargo fmt, sdr++  <- wasm,
    // webassemblygames.com
    // remove dopler effect,fft,ffi
    // lapce, rustrover,vscode
    //
    //f4();
    //f5(StatusToDo::Doing);
    //f6(9);
    //f7();
    //f8();
    //f9();
    //f10(10);
    // f11(10);
}
