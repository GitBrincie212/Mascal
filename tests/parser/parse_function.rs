use rstest::rstest;
use mascal::ast::AbstractSyntaxTree;
use mascal::defs::blocks::ScopedBlocks;
use mascal::defs::types::MascalUnprocessedType;
use crate::{define_program_boilerplate, run_parsing};

#[rstest(
    input, expected_function_name, expected_params, expected_return_type,
    case("DEFINE_FUNCTION abc() {IMPLEMENTATION {}}", "abc", vec![], None),
    case("DEFINE_FUNCTION mut_print(mut abc) -> STRING {IMPLEMENTATION {}}", "mut_print", vec![
        ("abc", true)
    ], Some(MascalUnprocessedType::String)),
    case(
        "DEFINE_FUNCTION hello_world(a, mut b, c) -> INTEGER {IMPLEMENTATION {}}",
        "hello_world",
        vec![
            ("a", false),
            ("b", true),
            ("c", false)
        ], Some(MascalUnprocessedType::Integer)
    ),
    case(
            "DEFINE_FUNCTION yes123() -> INTEGER {IMPLEMENTATION {}}",
            "yes123", vec![], Some(MascalUnprocessedType::Integer)
    ),
    case(
            "DEFINE_FUNCTION hmmm_3() {VARIABLES {} IMPLEMENTATION {}}",
            "hmmm_3", vec![], None
    ),
)]
fn test_correct_parsing(
    input: &str,
    expected_function_name: &str,
    expected_params: Vec<(&str, bool)>,
    expected_return_type: Option<MascalUnprocessedType>
) {
    let mut input_program_boilerplate: String = define_program_boilerplate!(
        Vec::<String>::new(),
        vec![ "" ]
    ) + "\n";
    input_program_boilerplate.push_str(input);
    let ast: AbstractSyntaxTree = run_parsing!(input_program_boilerplate.as_str()).unwrap();
    assert_eq!(ast.blocks.len(), 2);
    let ScopedBlocks::Function {
        parameters,
        name,
        return_type,
        execution_block,
    } = &ast.blocks[1] else {unreachable!()};
    assert_eq!(execution_block.body.len(), 0);
    assert_eq!(parameters.len(), expected_params.len());
    assert_eq!(name, expected_function_name);
    assert_eq!(return_type, &expected_return_type);
    parameters.iter().enumerate().for_each(|(index, parameter)| {
        assert_eq!(&*parameter.name, expected_params[index].0);
        assert_eq!(parameter.is_mutable, expected_params[index].1);
    })
}