use std::process;
use bleb;


fn main() {
    let args = bleb::Args::new();
    if let None = args {
        bleb::Args::help();
        process::exit(1);
    }
    let args = args.expect("Checked");
    if let Err(e) = bleb::run(args) {
        eprintln!("Error occured {e}");
        process::exit(1);
    }
}
