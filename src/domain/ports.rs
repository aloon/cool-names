use super::entities::{Adjective, Noun, CoolName};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum DomainError {
    #[error("No adjectives available")]
    NoAdjectivesAvailable,

    #[error("No nouns available")]
    NounsAvailable,

    #[error("Failed to load words: {0}")]
    LoadError(String),
}

pub trait WordRepository: Send + Sync {
    fn get_random_adjective(&self) -> Result<Adjective, DomainError>;
    fn get_random_noun(&self) -> Result<Noun, DomainError>;
    fn adjectives_count(&self) -> usize;
    fn nouns_count(&self) -> usize;
}

pub trait NameGenerator: Send + Sync {
    fn generate(&self) -> Result<CoolName, DomainError>;
}
