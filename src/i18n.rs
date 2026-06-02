use fluent::{FluentBundle, FluentResource};
use rust_embed::RustEmbed;
use unic_langid::LanguageIdentifier;

#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Asset;

const YES: &str = "yes";
const NO: &str = "no";
fn translate(key: String, lang: String) -> String {
    let file = format!("{lang}.ftl");
    let ftl_data = Asset::get(&file).expect("failed to load the language");
    let ftl_string = format!("{:?}", std::str::from_utf8(ftl_data.data.as_ref()).unwrap())
        .replace("\\n", "\n")
        .trim_start_matches('\"')
        .trim_end_matches('\"')
        .to_string();

    let res = FluentResource::try_new(ftl_string).expect("failed to parse the ftl file: {file}");
    let li: LanguageIdentifier = lang.parse().expect("parsing language failed: {lang}");

    let mut bundle = FluentBundle::new(vec![li]);
    bundle
        .add_resource(&res)
        .expect("failed to add FTL resources to the bundle");

    let msg = bundle.get_message(&key).expect("key doesn't exist: {key}");

    let mut errors = vec![];
    let pattern = msg.value().expect("message has no value.");
    let value = bundle.format_pattern(pattern, None, &mut errors);

    value.to_string()
}
pub fn yes(lang: String) -> String {
    translate(YES.to_string(), lang)
}
pub fn no(lang: String) -> String {
    translate(NO.to_string(), lang)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yes_en_us() {
        assert_eq!(yes("en-US".to_string()), "Yes, you can.");
    }

    #[test]
    fn no_en_us() {
        assert_eq!(no("en-US".to_string()), "No.");
    }

    #[test]
    fn yes_en_es() {
        assert_eq!(yes("en-ES".to_string()), "Sí tu puedes.");
    }

    #[test]
    fn yes_fr_fr() {
        assert_eq!(yes("fr-FR".to_string()), "Oui, vous pouvez.");
    }

    #[test]
    fn no_fr_fr() {
        assert_eq!(no("fr-FR".to_string()), "Non.");
    }

    #[test]
    fn yes_ja_jp() {
        assert_eq!(yes("ja-JP".to_string()), "はい、できます。");
    }

    #[test]
    fn no_ja_jp() {
        assert_eq!(no("ja-JP".to_string()), "いいえ。");
    }

    #[test]
    fn yes_and_no_differ() {
        assert_ne!(yes("en-US".to_string()), no("en-US".to_string()));
    }
}
