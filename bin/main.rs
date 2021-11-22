fn main() {
    if let Err(err) = mychip8::run() {
        println!("Application Error: {}", err);
        if let Some(source) = err.source() {
            println!("Source: {}", source);
        }
        std::process::exit(1);
    }
}
