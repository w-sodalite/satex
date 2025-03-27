use satex_core::expression::Expression;

#[test]
fn equals() {
    let expression = Expression::equals("hello", true);
    assert!(expression.matches(Some("hello")));
    assert!(!expression.matches(Some("Hello")));
    assert!(!expression.matches(Some("world")));
    assert!(!expression.matches(None));

    let expression = Expression::equals("hello", false);
    assert!(expression.matches(Some("hello")));
    assert!(expression.matches(Some("Hello")));
    assert!(!expression.matches(Some("world")));
    assert!(!expression.matches(None));
}

#[test]
fn not_equals() {
    let expression = Expression::not_equals("hello", true);
    assert!(!expression.matches(Some("hello")));
    assert!(expression.matches(Some("Hello")));
    assert!(expression.matches(Some("world")));
    assert!(expression.matches(None));

    let expression = Expression::not_equals("hello", false);
    assert!(!expression.matches(Some("hello")));
    assert!(!expression.matches(Some("Hello")));
    assert!(expression.matches(Some("world")));
    assert!(expression.matches(None));
}

#[test]
fn starts_with() {
    let expression = Expression::starts_with("hello", true);
    assert!(expression.matches(Some("hello world")));
    assert!(!expression.matches(Some("Hello world")));
    assert!(!expression.matches(Some("world")));
    assert!(!expression.matches(None));

    let expression = Expression::starts_with("hello", false);
    assert!(expression.matches(Some("hello world")));
    assert!(expression.matches(Some("Hello world")));
    assert!(!expression.matches(Some("world")));
    assert!(!expression.matches(None));
}

#[test]
fn not_starts_with() {
    let expression = Expression::not_starts_with("hello", true);
    assert!(!expression.matches(Some("hello world")));
    assert!(expression.matches(Some("Hello world")));
    assert!(expression.matches(Some("world")));
    assert!(expression.matches(None));

    let expression = Expression::not_starts_with("hello", false);
    assert!(!expression.matches(Some("hello world")));
    assert!(!expression.matches(Some("Hello world")));
    assert!(expression.matches(Some("world")));
    assert!(expression.matches(None));
}

#[test]
fn ends_with() {
    let expression = Expression::ends_with("world", true);
    assert!(expression.matches(Some("hello world")));
    assert!(!expression.matches(Some("hello World")));
    assert!(!expression.matches(Some("World")));
    assert!(!expression.matches(None));

    let expression = Expression::ends_with("world", false);
    assert!(expression.matches(Some("hello world")));
    assert!(expression.matches(Some("Hello World")));
    assert!(expression.matches(Some("World")));
    assert!(!expression.matches(None));
}

#[test]
fn not_ends_with() {
    let expression = Expression::not_ends_with("world", true);
    assert!(!expression.matches(Some("hello world")));
    assert!(expression.matches(Some("hello World")));
    assert!(expression.matches(Some("World")));
    assert!(expression.matches(None));

    let expression = Expression::not_ends_with("world", false);
    assert!(!expression.matches(Some("hello world")));
    assert!(!expression.matches(Some("hello World")));
    assert!(!expression.matches(Some("World")));
    assert!(expression.matches(None));
}

#[test]
fn contains() {
    let expression = Expression::contains("hello", true);
    assert!(expression.matches(Some("hello world")));
    assert!(!expression.matches(Some("Hello world")));
    assert!(!expression.matches(Some("world")));
    assert!(!expression.matches(None));

    let expression = Expression::contains("hello", false);
    assert!(expression.matches(Some("hello world")));
    assert!(expression.matches(Some("Hello world")));
    assert!(!expression.matches(Some("world")));
    assert!(!expression.matches(None));
}

#[test]
fn not_contains() {
    let expression = Expression::not_contains("hello", true);
    assert!(!expression.matches(Some("hello world")));
    assert!(expression.matches(Some("Hello world")));
    assert!(expression.matches(Some("world")));
    assert!(expression.matches(None));

    let expression = Expression::not_contains("hello", false);
    assert!(!expression.matches(Some("hello world")));
    assert!(!expression.matches(Some("Hello world")));
    assert!(expression.matches(Some("world")));
    assert!(expression.matches(None));
}

#[test]
fn exists() {
    let expression = Expression::exists();
    assert!(expression.matches(Some("hello")));
    assert!(!expression.matches(None));
}

#[test]
fn not_exists() {
    let expression = Expression::not_exists();
    assert!(!expression.matches(Some("hello")));
    assert!(expression.matches(None));
}

#[test]
fn regex() {
    let expression = Expression::regex("^hello$").unwrap();
    assert!(expression.matches(Some("hello")));
    assert!(!expression.matches(Some("world")));
}
