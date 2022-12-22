
pub struct Args{
    pub item: String,
}

pub fn parse() -> Args {
    let input = std::env::args().nth(1).expect("invalid input");
    return Args{item: input};
}