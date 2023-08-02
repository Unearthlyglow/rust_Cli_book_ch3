
fn main() {
    if let Err(e) = catr::get_args().and_then(catr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

//If the catr::get_args function returns an Ok(config) value, use Result​::and_then to pass the config to catr::run.
//If either get_args or run returns an Err, print it to STDERR.
//Exit the program with a nonzero value.