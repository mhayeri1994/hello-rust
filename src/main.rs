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
fn f3(){
// vectors
let vec1 = [1,2,3,4];
println!("vec1 is {}",type_of(vec1));
}
//---------------------------------------------
use std::collections::HashMap;
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            match map.get(&(target - *v)) {
                Some(&i2) => return vec![i as i32, i2],
                None => map.insert(*v, i as i32),
            };
        };
        
        return vec![];
    }

fn f4(){
println!("{:?}",two_sum(vec![3,2,4], 6))
}
//------------------------------------------------
fn main() {
    //f1();
    //f2();
    //f3();
    //101,yew,rachet,rustwasm,actix,orm diesel,tokio async,ffi,bingen/binjen,
    //non blocking, goroutin vs coroutin vs future thread rust,template engins,harfbuzz,
    //concurrency, thread, process, syscall,file,large file,sql/sqlite/postgres/mysql,rfd,rfc,socket,
    // IPC, RPC, GRPC, RESTfull,websocket,cross compile,
    // rust mem manage vs runtime (garbage
    // collector) c++/java... vs manual c/c++,
    f4();
}
