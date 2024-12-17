// Podemos declarar uma nova variável com o mesmo nome que uma variável anterior.
// Esse é o conceito de Shadowing
fn main() {
    let x: i32 = 5;
    
    {
        let x: i32 = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x: i32 = 42;
    println!("{x}");
}