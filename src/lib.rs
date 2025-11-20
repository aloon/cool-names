use rand::prelude::*;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GeneratorError {
    #[error("Failed to read file: {0}")]
    FileReadError(String),
    #[error("No adjectives available")]
    NoAdjectivesAvailable,
    #[error("No nouns available")]
    NoNounsAvailable,
}

pub struct NameGenerator {
    adjectives: Vec<String>,
    nouns: Vec<String>,
}

impl NameGenerator {
    pub fn new() -> Result<Self, GeneratorError> {
        let adjectives = Self::load_words("adjectives.txt")?;
        let nouns = Self::load_words("nouns.txt")?;

        if adjectives.is_empty() {
            return Err(GeneratorError::NoAdjectivesAvailable);
        }
        if nouns.is_empty() {
            return Err(GeneratorError::NoNounsAvailable);
        }

        Ok(Self { adjectives, nouns })
    }

    fn load_words<P: AsRef<Path>>(path: P) -> Result<Vec<String>, GeneratorError> {
        let content = fs::read_to_string(path.as_ref()).map_err(|e| {
            GeneratorError::FileReadError(format!("{}: {}", path.as_ref().display(), e))
        })?;

        Ok(content
            .lines()
            .map(|line| line.trim().to_lowercase())
            .filter(|line| !line.is_empty())
            .collect())
    }

    pub fn generate(&self) -> Result<String, GeneratorError> {
        let mut rng = rand::rng();

        let adjective = self
            .adjectives
            .choose(&mut rng)
            .ok_or(GeneratorError::NoAdjectivesAvailable)?;

        let noun = self
            .nouns
            .choose(&mut rng)
            .ok_or(GeneratorError::NoNounsAvailable)?;

        Ok(format!("{}-{}", adjective, noun))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_words() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "word1").unwrap();
        writeln!(temp_file, "Word2").unwrap();
        writeln!(temp_file, "WORD3").unwrap();
        writeln!(temp_file, "").unwrap();

        let words = NameGenerator::load_words(temp_file.path()).unwrap();
        assert_eq!(words.len(), 3);
        assert_eq!(words[0], "word1");
        assert_eq!(words[1], "word2");
        assert_eq!(words[2], "word3");
    }

    #[test]
    fn test_generate_returns_valid_format() {
        let generator = NameGenerator::new().unwrap();
        let name = generator.generate().unwrap();

        assert!(name.contains('-'));
        let parts: Vec<&str> = name.split('-').collect();
        assert_eq!(parts.len(), 2);
        assert!(!parts[0].is_empty());
        assert!(!parts[1].is_empty());
    }

    #[test]
    fn test_generate_uses_loaded_words() {
        let generator = NameGenerator::new().unwrap();
        let name = generator.generate().unwrap();
        let parts: Vec<&str> = name.split('-').collect();

        assert!(generator.adjectives.contains(&parts[0].to_string()));
        assert!(generator.nouns.contains(&parts[1].to_string()));
    }

    #[test]
    fn test_multiple_generations_work() {
        let generator = NameGenerator::new().unwrap();

        for _ in 0..10 {
            let name = generator.generate().unwrap();
            assert!(name.contains('-'));
        }
    }
}
