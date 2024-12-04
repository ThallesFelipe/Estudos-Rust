// Uma variável só pode ser usada somente se ela tiver sido inicializada.

fn main() {
    let x: i32 = 5; // Inicializada, pode ser utilizada.
    let y: i32;     // Não inicializada mas não utilizada, causa um 'warning' no terminal.
    let _z: i32;    // Não inicializada mas tem a marcação '_' que diz pro compilador que você ainda não utilizou.

    let mut a: i32 = 1; // Use 'mut' para marcar uma variável como mutável.
    a += 2;

    assert_eq!(x, 5);
    assert_eq!(a, 3);
    println!("Sucesso!");
}