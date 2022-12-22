mod args;
mod kickable;

fn main() {
    if std::env::args().len() == 1 {
        std::process::exit(exitcode::NOINPUT);
    }
    let args = args::parse();
    let response = kickable::can_i_kick(&args.item);
    println!("{response}");
}
