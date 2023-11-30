pub trait Translator {
    fn translate(log: String) -> (String, String);
}