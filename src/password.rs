pub mod consts;
pub mod generator;
mod options;

pub trait PasswordGenerator {
    fn new(length: usize) -> Self;
    fn with_special(&mut self, required: Option<bool> ) -> &mut Self;
    fn with_numbers(&mut self, required: Option<bool>) -> &mut Self;
    fn with_lowercase(&mut self, required: Option<bool>) -> &mut Self;
    fn with_uppercase(&mut self, required: Option<bool>) -> &mut Self;
    fn without_similar(&mut self) -> &mut Self;
    fn without_ambiguous(&mut self) -> &mut Self;
    fn without_sequential(&mut self) -> &mut Self;
    fn generate(&mut self) -> String;
}