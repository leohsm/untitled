use std::io;

fn main() {
    let buffer = ler_string();
    println!("{}", buffer);
}

fn ler_string() -> String {
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Falha ao ler linha.");

    buffer.trim().to_string()
}
