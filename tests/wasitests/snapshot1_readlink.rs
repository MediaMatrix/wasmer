// !!! THIS IS A GENERATED FILE !!!
// ANY MANUAL EDITS MAY BE OVERWRITTEN AT ANY TIME
// Files autogenerated with cargo build.

#[test]
fn test_snapshot1_readlink() {
    assert_wasi_output!(
        "../wasi_test_resources/snapshot1/readlink.wasm",
        "snapshot1_readlink",
        vec![],
        vec![(
            ".".to_string(),
            ::std::path::PathBuf::from("tests/wasi_test_resources/test_fs/hamlet")
        ),],
        vec![],
        "../wasi_test_resources/readlink.out"
    );
}