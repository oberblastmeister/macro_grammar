#[test]
fn trybuild() {
    let t = trybuild::TestCases::new();
    t.pass("trybuild/derive_language/01-recognize.rs");
}
