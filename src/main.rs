use std::io;

fn main() {
    let chosen = menu(vec!["A", "B", "C", "D", "E"]);
    println!("Chosen: {}", chosen);
}

fn read_string() -> String {
    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");

    buffer.trim().to_string()
}

fn read_u8() -> u8 {
    let buffer = loop {
        match read_string().parse::<u8>() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };
    buffer
}

fn menu(options: Vec<&str>) -> u8 {
    let buffer = options.len();
    for n in 0..buffer {
        println!("{}) {}", n, options[n]);
    }
    loop {
        let chosen = read_u8();
        if chosen < buffer as u8 {
            break chosen;
        }
    }
}
