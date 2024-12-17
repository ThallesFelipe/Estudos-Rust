fn main() {
    let (mut x, y): (i32, i32) = (1, 2);

    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    let (a, b): (i32, i32);

    (a,..) = (3, 4);
    [.., b] = [1, 2];

    assert_eq!([a, b], [3, 2]);

    println!("Sucesso!");
}