#[test]
fn trybuild() {
    let t = trybuild::TestCases::new();
    t.pass("trybuild/derive_t/01-recognize.rs");
    t.pass("trybuild/derive_t/02-creates-macro.rs");
    t.compile_fail("trybuild/derive_t/03-no-__LAST.rs");
    t.compile_fail("trybuild/derive_t/04-empty-enum.rs");
    t.compile_fail("trybuild/derive_t/05-too-many-attrs.rs");
    t.compile_fail("trybuild/derive_t/06-invalid-combination.rs");
    t.compile_fail("trybuild/derive_t/07-no-attrs.rs");
}
