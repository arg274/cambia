pub trait Translator {
    // TODO: Should use Cow for Translator API
    fn translate(log: String) -> (String, String);
}

pub trait TranslatorCombined {
    fn translate_combined(&self) -> String;
}