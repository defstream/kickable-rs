const NO: &'static str = "No.";
const YES: &'static str = "Yes, yes you can.";

pub fn kickable(input :&str) -> bool {
    if input.to_lowercase() == "it" {
        return true;
    }
    return false;
}

pub fn can_i_kick(input :&str) -> &str {
    if kickable(input) {
        return YES;
    }
    return NO;
}
