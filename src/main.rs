mod args;
mod kickable;

const NO: &'static str = "No.";
const YES: &'static str = "Yes, yes you can.";
const NO_INPUT: &'static str = "\n👟Kickable let's you know if you can kick it.\n\nUsage: {kickable} <item>\n<item> is the value to check for kick-ability.";

fn main() {
    if std::env::args().len() == 1 {
        println!("{NO_INPUT}");
        std::process::exit(exitcode::NOINPUT);
    }
    let args = args::parse();

    if kickable::validate(&args.item) {
        println!("{YES}");
        std::process::exit(exitcode::OK);
    } else {
        println!("{NO}");
        std::process::exit(exitcode::DATAERR);
    }
}
