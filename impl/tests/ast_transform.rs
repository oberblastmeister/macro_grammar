#[test]
fn trybuild() {
    let t = trybuild::TestCases::new();
    t.pass("trybuild/ast_transform/01-recognize.rs");
}
