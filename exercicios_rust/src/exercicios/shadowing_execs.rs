fn main() {
    let mut x: i32 = 1;
    println!("x: {x}");
    
    x = 7;
    println!("x: {x}");

    let mut x: i32 = x;
    x += 3; // x == 10
    println!("x: {x}");

    let y: i32 = 4;
    println!("y: {y}");

    let y: &str = "Eu também posso ser vinculada à texto!";
    println!("y: {y}");

    println!("Sucesso!");
}