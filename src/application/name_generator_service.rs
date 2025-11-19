use crate::domain::{CoolName, NameGenerator};
use crate::domain::ports::{DomainError, WordRepository};
use std::sync::Arc;

pub struct NameGeneratorService {
    repository: Arc<dyn WordRepository>,
}

impl NameGeneratorService {
    pub fn new(repository: Arc<dyn WordRepository>) -> Self {
        Self { repository }
    }
}

impl NameGenerator for NameGeneratorService {
    fn generate(&self) -> Result<CoolName, DomainError> {
        let adjective = self.repository.get_random_adjective()?;
        let noun = self.repository.get_random_noun()?;
        Ok(CoolName::new(adjective, noun))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Adjective, Noun};

    struct MockWordRepository {
        adjective: String,
        noun: String,
    }

    impl WordRepository for MockWordRepository {
        fn get_random_adjective(&self) -> Result<Adjective, DomainError> {
            Ok(Adjective::new(self.adjective.clone()))
        }

        fn get_random_noun(&self) -> Result<Noun, DomainError> {
            Ok(Noun::new(self.noun.clone()))
        }

        fn adjectives_count(&self) -> usize {
            1
        }

        fn nouns_count(&self) -> usize {
            1
        }
    }

    #[test]
    fn test_generate_cool_name() {
        let mock_repo = Arc::new(MockWordRepository {
            adjective: "mighty".to_string(),
            noun: "phoenix".to_string(),
        });

        let service = NameGeneratorService::new(mock_repo);
        let result = service.generate().unwrap();

        assert_eq!(result.to_string(), "mighty-phoenix");
    }

    struct FailingAdjRepository;

    impl WordRepository for FailingAdjRepository {
        fn get_random_adjective(&self) -> Result<Adjective, DomainError> {
            Err(DomainError::NoAdjectivesAvailable)
        }

        fn get_random_noun(&self) -> Result<Noun, DomainError> {
            Ok(Noun::new("test".to_string()))
        }

        fn adjectives_count(&self) -> usize {
            0
        }

        fn nouns_count(&self) -> usize {
            1
        }
    }

    #[test]
    fn test_generate_fails_when_no_adjectives() {
        let mock_repo = Arc::new(FailingAdjRepository);
        let service = NameGeneratorService::new(mock_repo);
        let result = service.generate();

        assert!(matches!(result, Err(DomainError::NoAdjectivesAvailable)));
    }
}
