pub(crate) struct Args {
    pub item: String,
}

impl std::fmt::Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let args: Vec<String> = std::env::args().collect();
        write!(f, "{:?}", args)
    }
}

pub(crate) fn parse() -> Args {
    let input = std::env::args().nth(1).expect("invalid input");

    Args {
        item: input,
    }
}
