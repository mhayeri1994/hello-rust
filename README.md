hello.rs
```rust
fn main(){
    println!("hello");
}
```

rustc hello.rs
----------
cargo init


std_in
use std::env;
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    // torbofish ::<> is for set type
    let x: u32 = args[1].parse().unwrap();
    println!("Fibonacci({:?})", x);
    println!("{:?}", fibonacci(x));
}


regex
use regex::Regex;

fn main() {
    println!("Hello, world!");
    let re = Regex::new(r"\d");
    let Some(caps) = re.captures("hello 123 435-fsadf1234") else {
        println!("no match!");
        return;
    };
    println!("response: {}", &caps["name"]);
}

fortune-mod
fortune -s | chara say -f ferris | lolcat


curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install wasm-pack

use std::env;
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    // torbofish ::<> is for set type
    let x: u32 = args[1].parse().unwrap();
    println!("Fibonacci({:?})", x);
    println!("{:?}", fibonacci(x));
}


cargo test
fn main() {
    println!("Hello, world!");
}

mod decode_tests{
    #[test]
    fn test_basic(){
assert!(1==1);
    }
}


rurl

extern mod http;
use http::client::RequestWriter;
use http::methos::Get;
use std::os;
fn main() {
    println!("Hello, world!");
    let request = RequestWriter::new(Get, FromStr::from_str(os::args()[1]).wnwrap());
    let response = match request.read_response() {};
//    if response
}


rurl, curl
Previously, we know 'cURL' is a program to work with protocols like HTTP,FTP and etc.
And we know 'cURL' means 'Client for URL' or 'programming C URL'.

Now, We want to try rewrite this program with Rust language.

This is main line repo.
We want to go to branch to add first feature.
------------------

actix-web = "3"
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder{
    println!("visited");
    HttpResponse::Ok().body("Hello Actix")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder{
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder{
    HttpResponse::Ok().body("Hey there!")

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //println!("Hello, world!");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
------------------------
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder{
    println!("visited");
    HttpResponse::Ok().body("Hello Actix")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder{
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder{
    HttpResponse::Ok().body("Hey there!")

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //println!("Hello, world!");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder{
    println!("visited");
    HttpResponse::Ok().body("Hello Actix")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder{
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder{
    HttpResponse::Ok().body("Hey there!")

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //println!("Hello, world!");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

------------
fn fibonacci() {}
fn main() {
    let mut n0: u128 = 0;
    let mut n1: u128 = 1;
    let mut temp: u128;
    for _ in 0..5 {
        temp = n0;
        n0 = n1;
        n1 = temp + n1;
        println!("{} {}", n1, n0);
    }
    println!("Hello, world!");
}

-----------
gussing game
fn main() {
    println!("Hello, world!");
    let mut x=4;
    x = 5;
    println!("The value of x is: {}", x);
    println!("The value of x+1 is: {}", x+1);
}
---------------------
actix-web = "4"
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("hey")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //    println!("Hello, world!");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
-----------
init project by
```sh
cargo init
```
---------------------------------
fn main() {
    println!("Hello, world!");
    
    //basic format
    println!("{} basic format", "println");

    //positional format
    println!("println! positional format 
    position 0:{0}
    pos...1:{1}
    pos...0 again:{0}", "data0", "data1");

    //format by name
    println!("println! format by name
    Firstname = {firstname}
    Lastname = {lastname}",firstname="Matrix", lastname="Gram");

    //placeholder bin, hex, oct
    println!("println! place holder
    Number {0}
    bin    {0:b}
    hex    {0:x}
    oct    {0:o}",255);

    //placeholder for debug
    println!("println! placeholder for debug{:?}",(127,true,"Rust"));

    //basic math
    println!("println! basic math 127+1 = in binary {:b}",127+1);
}
--------------------
mutable

fn main() {
    let var_1="Matrix";
    let mut var_2=255;
    let v3:i32=0b1111;
    println!("{}",v3);
    var_2+=1;
    println!("Hello, world! {} {}", var_1, var_2);
}

-----------
//module mod ...
cat src/my_mod.rs
pub fn run(){
    println!("this is my_mod run function");
    }

cat src/main.rs
//mod my_mod;
use my_mod::run;
fn main() {
  //my_mod::run();
  run();  
    println!("Hello, world!");
}

-------------------
// plotting
plotters = "0.4"

use plotters::prelude::*;
use std::error::Error;

// Define the sigmoid function
fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn main() {
    println!("Hello, world!");
    // Create a drawing area with a specified size
    let root = BitMapBackend::new("sigmoid_plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Define the range of x values
    let x_values = -10.0..=10.0;

    // Create a chart context
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(5)
        .caption("Sigmoid Function", ("sans-serif", 20.0))
        .build_cartesian_2d(x_values.clone(), 0.0..=1.0)?;

    // Draw the sigmoid function
    chart.configure_mesh().draw()?;
    chart
        .draw_series(LineSeries::new(
            x_values.clone().map(|x| (x, sigmoid(x))),
            &BLUE,
        ))?
        .label("Sigmoid Function")
        .legend(|(x, y)| Path::new(vec![(x, y), (x + 20, y)], &BLUE));

    // Add legend
    chart.configure_series_labels().draw()?;

    Ok(())
}
-----------------------
ffi
--------------
hello
```sh
echo 'fn main(){println!("hello world!/");}' > hello.rs && rustc hello.rs && ./hello
```

cat makefile
all:
	rustc hello.rs

run:
	./hello

cat hello.rs
fn main(){println!("hello world!");}
-----------------
arg

use std::env;

fn main() {
    for arg in env::args() {
        println!("{}", arg);
    }
}

-----------------
vars variables
fn main() { 
    let v1=std::i32::MAX;       //implecit define default i32
    let v2:i8=std::i8::MAX;     //explecit define
    let v3:i8=std::i8::MIN;     //i8 min -128 max 127
    let v4:i128=std::i128::MAX;
    let is_active=true;         //false
    let is_greater:bool=v1>v2.into(); //use 'into' to convert into type
    let v5='a';
    let v6='\u{1F600}';
    println!("{:?}", (v1,
    v2,
    v3,v4,is_active,is_greater,v5,v6));//format for debug, show everythings
}
