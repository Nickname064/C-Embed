use std::io::{stdin, Read};

fn main() {

    let field_name = match std::env::args().nth(1) {
        Some(name) => name,
        None => {eprintln!("Please specify a C field name for the array."); return; }
    };

    let mut buffer = Vec::new();
    stdin().read_to_end(&mut buffer).expect("STDIN reading err");

    print!("char* {} = {{ ", field_name);

    let mut bytes = buffer.bytes();

    if let Some(Ok(byte)) = bytes.next(){
        print!("0x{:x}", byte);
    }

    for byte in buffer.bytes(){
        print!(", 0x{:x}", byte.expect("Byte reading err. Exiting"));
    }

    print!(" }};")
}
