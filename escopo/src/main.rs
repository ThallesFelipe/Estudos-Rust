fn main() {
    let x: i32 = 10;
    let y:i32 = 5;

    {
        println!("O valor de x é {x} e o valor de y é {y}");
    }
    
    println!("O valor de x é {} e o valor de y é {}", x, y);
}