use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Adjective(String);

impl Adjective {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Adjective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Noun(String);

impl Noun {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Noun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoolName {
    adjective: Adjective,
    noun: Noun,
}

impl CoolName {
    pub fn new(adjective: Adjective, noun: Noun) -> Self {
        Self { adjective, noun }
    }

    pub fn adjective(&self) -> &Adjective {
        &self.adjective
    }

    pub fn noun(&self) -> &Noun {
        &self.noun
    }
}

impl fmt::Display for CoolName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.adjective, self.noun)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cool_name_display() {
        let adjective = Adjective::new("awesome".to_string());
        let noun = Noun::new("dragon".to_string());
        let cool_name = CoolName::new(adjective, noun);

        assert_eq!(cool_name.to_string(), "awesome-dragon");
    }

    #[test]
    fn test_adjective_creation() {
        let adj = Adjective::new("happy".to_string());
        assert_eq!(adj.as_str(), "happy");
    }

    #[test]
    fn test_noun_creation() {
        let noun = Noun::new("panda".to_string());
        assert_eq!(noun.as_str(), "panda");
    }
}
