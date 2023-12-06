pub trait Translator {
    // TODO: Should use Cow for Translator API
    fn translate(log: String) -> (String, String);
}