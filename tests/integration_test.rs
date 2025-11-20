use cool_names::NameGenerator;

#[test]
fn test_generator_loads_and_generates() {
    let generator = NameGenerator::new().expect("Failed to create generator");
    let name = generator.generate().expect("Failed to generate name");

    assert!(name.contains('-'));
    let parts: Vec<&str> = name.split('-').collect();
    assert_eq!(parts.len(), 2);
    assert!(!parts[0].is_empty());
    assert!(!parts[1].is_empty());
}

#[test]
fn test_multiple_generations_produce_different_names() {
    let generator = NameGenerator::new().expect("Failed to create generator");
    let mut names = std::collections::HashSet::new();

    for _ in 0..20 {
        let name = generator.generate().expect("Failed to generate name");
        names.insert(name);
    }

    assert!(
        names.len() > 1,
        "Should generate different names across multiple calls"
    );
}
