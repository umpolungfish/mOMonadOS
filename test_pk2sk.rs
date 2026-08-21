extern crate pk2sk;

fn main() {
    println!("pk2sk imported successfully");
    let x = pk2sk::U256::from_u64(42);
    println!("x = {:?}", x);
}