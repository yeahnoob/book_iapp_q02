extern crate rcalc;

use rcalc::Interpreter;

fn main() {
    let expr = "8*86";
    let mut pr = Interpreter::from(expr);
    let result= pr.interpret().unwrap();
    println!("{}",result);

}
