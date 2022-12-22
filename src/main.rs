mod args;
mod kickable;

const NO: &str = "No.";
const YES: &str = "Yes, yes you can.";
const NO_INPUT: &str = "\n👟Kickable let's you know if you can kick it.\n\nUsage: {kickable} <item>\n<item> is the value to check for kick-ability.";

fn main() {
    // 1. validate input
    if std::env::args().len() == 1 {
        println!("{NO_INPUT}");
        std::process::exit(exitcode::NOINPUT);
    }

    // 2. parse input
    let args = args::parse();

    // 3. validate inputs kick-ability
    if kickable::validate(&args.item) {
        println!("{YES}");
        std::process::exit(exitcode::OK);
    } else {
        println!("{NO}");
        std::process::exit(exitcode::DATAERR);
    }
}
