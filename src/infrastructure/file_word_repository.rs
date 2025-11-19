use crate::domain::{Adjective, Noun};
use crate::domain::ports::{DomainError, WordRepository};
use rand::prelude::IndexedRandom;
use std::fs;
use std::path::Path;

pub struct FileWordRepository {
    adjectives: Vec<String>,
    nouns: Vec<String>,
}

impl FileWordRepository {
    pub fn new(adjectives_path: &Path, nouns_path: &Path) -> Result<Self, DomainError> {
        let adjectives = Self::load_words(adjectives_path)?;
        let nouns = Self::load_words(nouns_path)?;

        if adjectives.is_empty() {
            return Err(DomainError::NoAdjectivesAvailable);
        }

        if nouns.is_empty() {
            return Err(DomainError::NounsAvailable);
        }

        Ok(Self { adjectives, nouns })
    }

    fn load_words(path: &Path) -> Result<Vec<String>, DomainError> {
        fs::read_to_string(path)
            .map_err(|e| DomainError::LoadError(format!("Failed to read file: {}", e)))
            .map(|content| {
                content
                    .lines()
                    .map(|line| line.trim())
                    .filter(|line| !line.is_empty())
                    .map(|line| line.to_string())
                    .collect()
            })
    }
}

impl WordRepository for FileWordRepository {
    fn get_random_adjective(&self) -> Result<Adjective, DomainError> {
        self.adjectives
            .choose(&mut rand::rng())
            .map(|word| Adjective::new(word.clone()))
            .ok_or(DomainError::NoAdjectivesAvailable)
    }

    fn get_random_noun(&self) -> Result<Noun, DomainError> {
        self.nouns
            .choose(&mut rand::rng())
            .map(|word| Noun::new(word.clone()))
            .ok_or(DomainError::NounsAvailable)
    }

    fn adjectives_count(&self) -> usize {
        self.adjectives.len()
    }

    fn nouns_count(&self) -> usize {
        self.nouns.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_temp_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file
    }

    #[test]
    fn test_load_words_success() {
        let adj_file = create_temp_file("happy\nsad\nexcited");
        let noun_file = create_temp_file("cat\ndog\nbird");

        let repo = FileWordRepository::new(adj_file.path(), noun_file.path()).unwrap();

        assert_eq!(repo.adjectives_count(), 3);
        assert_eq!(repo.nouns_count(), 3);
    }

    #[test]
    fn test_load_words_filters_empty_lines() {
        let adj_file = create_temp_file("happy\n\nsad\n  \nexcited");
        let noun_file = create_temp_file("cat\ndog");

        let repo = FileWordRepository::new(adj_file.path(), noun_file.path()).unwrap();

        assert_eq!(repo.adjectives_count(), 3);
    }

    #[test]
    fn test_get_random_adjective() {
        let adj_file = create_temp_file("awesome\ncool");
        let noun_file = create_temp_file("dragon");

        let repo = FileWordRepository::new(adj_file.path(), noun_file.path()).unwrap();
        let adjective = repo.get_random_adjective().unwrap();

        assert!(adjective.as_str() == "awesome" || adjective.as_str() == "cool");
    }

    #[test]
    fn test_get_random_noun() {
        let adj_file = create_temp_file("happy");
        let noun_file = create_temp_file("cat\ndog");

        let repo = FileWordRepository::new(adj_file.path(), noun_file.path()).unwrap();
        let noun = repo.get_random_noun().unwrap();

        assert!(noun.as_str() == "cat" || noun.as_str() == "dog");
    }

    #[test]
    fn test_empty_adjectives_file_returns_error() {
        let adj_file = create_temp_file("");
        let noun_file = create_temp_file("cat");

        let result = FileWordRepository::new(adj_file.path(), noun_file.path());

        assert!(matches!(result, Err(DomainError::NoAdjectivesAvailable)));
    }

    #[test]
    fn test_empty_nouns_file_returns_error() {
        let adj_file = create_temp_file("happy");
        let noun_file = create_temp_file("");

        let result = FileWordRepository::new(adj_file.path(), noun_file.path());

        assert!(matches!(result, Err(DomainError::NounsAvailable)));
    }
}
