use wasmer_interface_types::{
    ast::*, decoders::binary::parse, encoders::binary::ToBytes, interpreter::Instruction,
};

/// Tests an AST to binary, then binary to AST roundtrip.
#[test]
fn test_binary_encoding_decoding_roundtrip() {
    let original_ast = Interfaces {
        exports: vec![Export {
            name: "ab",
            input_types: vec![InterfaceType::I32],
            output_types: vec![InterfaceType::I32],
        }],
        types: vec![Type::new(
            "ab",
            vec!["cd", "e"],
            vec![InterfaceType::I32, InterfaceType::I32],
        )],
        imports: vec![Import {
            namespace: "a",
            name: "b",
            input_types: vec![InterfaceType::I32],
            output_types: vec![InterfaceType::I64],
        }],
        adapters: vec![Adapter::Import {
            namespace: "a",
            name: "b",
            input_types: vec![InterfaceType::I32],
            output_types: vec![InterfaceType::I32],
            instructions: vec![Instruction::ArgumentGet { index: 1 }],
        }],
        forwards: vec![Forward { name: "a" }],
    };

    let mut binary = vec![];

    original_ast
        .to_bytes(&mut binary)
        .expect("Failed to encode the AST.");

    let (remainder, ast) = parse::<()>(binary.as_slice()).expect("Failed to decode the AST.");

    assert!(remainder.is_empty());

    assert_eq!(original_ast, ast);
}
