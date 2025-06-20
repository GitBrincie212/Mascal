use std::cell::{RefCell};
use std::collections::HashMap;
use std::io;
use std::num::{ParseFloatError, ParseIntError};
use std::rc::Rc;
use std::sync::{Arc};
use once_cell::sync::Lazy;
use crate::defs::builtins::utils::flatten_impl;
use crate::defs::dynamic_int::IntegerNum;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::defs::types::{MascalType, MascalTypeKind};
use crate::runtime::ExecutionData;
use crate::runtime::values::MascalValue;
use crate::runtime::utils::{get_dimensions, get_sizes};

#[derive(Clone)]
pub enum BuiltinFunction {
    ValueBased {
        fixed_argument_types: Vec<Vec<MascalTypeKind>>,
        supports_dynamic_arguments: bool,
        execution: Arc<dyn Fn(Vec<MascalValue>, Rc<RefCell<ExecutionData>>) -> Result<Option<MascalValue>, MascalError> + Send + Sync>,
    },

    ExpressionBased {
        fixed_argument_types: Vec<Vec<MascalTypeKind>>,
        supports_dynamic_arguments: bool,
        execution: Arc<dyn Fn(Vec<&MascalExpression>, Rc<RefCell<ExecutionData>>) -> Result<Option<MascalValue>, MascalError> + Send + Sync>,
    }
}

impl BuiltinFunction {
    fn new_value_based(
        supports_dynamic_arguments: bool,
        fixed_argument_types: Vec<Vec<MascalTypeKind>>,
        execution: Arc<dyn Fn(Vec<MascalValue>, Rc<RefCell<ExecutionData>>) -> Result<Option<MascalValue>, MascalError> + Send + Sync>,
    ) -> Self {
        BuiltinFunction::ValueBased {
            supports_dynamic_arguments,
            fixed_argument_types,
            execution,
        }
    }

    fn new_expresion_based (
        supports_dynamic_arguments: bool,
        fixed_argument_types: Vec<Vec<MascalTypeKind>>,
        execution: Arc<dyn Fn(Vec<&MascalExpression>, Rc<RefCell<ExecutionData>>) -> Result<Option<MascalValue>, MascalError> + Send + Sync>,
    ) -> Self {
        BuiltinFunction::ExpressionBased {
            supports_dynamic_arguments,
            fixed_argument_types,
            execution,
        }
    }
}

macro_rules! define_builtin_function {
    (
        $builtin_construct: expr, $name: expr, $map: expr, $fixed_args: expr,
        $supports_dynamic_arguments: expr, $func: expr
    ) => {
        let func: Arc<BuiltinFunction> = Arc::new($builtin_construct(
            $supports_dynamic_arguments,
            $fixed_args,
            Arc::new($func),
        ));
        $map.insert(String::from($name).to_uppercase(), Arc::clone(&func));
        $map.insert(String::from($name), Arc::clone(&func));
        $map.insert(String::from($name).to_lowercase(), Arc::clone(&func));
    };
}

pub static BUILT_IN_FUNCTION_TABLE: Lazy<HashMap<String, Arc<BuiltinFunction>>>  = Lazy::new(|| {
    let mut map: HashMap<String, Arc<BuiltinFunction>> = HashMap::new();
    define_builtin_function!(
        BuiltinFunction::new_value_based, "Write", map, vec![], true,
        |args, _| {
            for val in args {
                print!("{}", val.as_string()?);
            }
            print!("\n");
            Ok(None)
        }
    );

    define_builtin_function!(
        BuiltinFunction::new_value_based, "Ln", map, vec![vec![MascalTypeKind::Float]], false,
        |args, _| {
            match args.first().unwrap() {
                MascalValue::Integer(i) => Ok(Some(MascalValue::Integer(i.ln()?))),
                MascalValue::Float(f) => Ok(Some(MascalValue::Float(f.ln()))),
                _ => unreachable!()
            }
        }
    );

    define_builtin_function!(
        BuiltinFunction::new_value_based, "Sqrt", map, vec![vec![MascalTypeKind::Float, MascalTypeKind::Integer]], false,
        |args, _| {
            match args.first().unwrap() {
                MascalValue::Integer(i) => Ok(Some(MascalValue::Integer(i.isqrt()?))),
                MascalValue::Float(f) => Ok(Some(MascalValue::Float(f.sqrt()))),
                _ => unreachable!()
            }
        }
    );

    define_builtin_function!(
        BuiltinFunction::new_value_based, "Log", map, vec![vec![MascalTypeKind::Float, MascalTypeKind::Integer]], false,
        |args, _| {
            match args.first().unwrap() {
                MascalValue::Integer(i) => Ok(Some(MascalValue::Integer(i.log10()?))),
                MascalValue::Float(f) => Ok(Some(MascalValue::Float(f.log10()))),
                _ => unreachable!()
            }
        }
    );

    define_builtin_function!(
        BuiltinFunction::new_value_based, "Log2", map, vec![vec![MascalTypeKind::Float, MascalTypeKind::Integer]], false,
        |args, _| {
            match args.first().unwrap() {
                MascalValue::Integer(i) => Ok(Some(MascalValue::Integer(i.log2()?))),
                MascalValue::Float(f) => Ok(Some(MascalValue::Float(f.log2()))),
                _ => unreachable!()
            }
        }
    );

    define_builtin_function!(
        BuiltinFunction::new_value_based, "Exp", map, vec![vec![MascalTypeKind::Float, MascalTypeKind::Integer]], false,
        |args, _| {
            match args.first().unwrap() {
                MascalValue::Integer(i) => Ok(Some(
                    MascalValue::Integer(IntegerNum::new(i.as_f64().exp().round() as i128))
                )),
                MascalValue::Float(f) => Ok(Some(MascalValue::Float(f.exp()))),
                _ => unreachable!()
            }
        }
    );

    define_builtin_function!(
        BuiltinFunction::new_value_based, "Cbrt", map, vec![vec![MascalTypeKind::Float, MascalTypeKind::Integer]], false,
        |args, _| {
            match args.first().unwrap() {
                MascalValue::Integer(i) => Ok(Some(
                    MascalValue::Integer(IntegerNum::new(i.as_f64().cbrt().round() as i128))
                )),
                MascalValue::Float(f) => Ok(Some(MascalValue::Float(f.cbrt()))),
                _ => unreachable!()
            }
        }
    );

    define_builtin_function!(
        BuiltinFunction::new_value_based, "Abs", map, vec![vec![MascalTypeKind::Float, MascalTypeKind::Integer]], false,
        |args, _| {
            match args.first().unwrap() {
                MascalValue::Integer(i) => Ok(Some(
                    MascalValue::Integer(IntegerNum::new(i.to_i128().abs()))
                )),
                MascalValue::Float(f) => Ok(Some(MascalValue::Float(f.abs()))),
                _ => unreachable!()
            }
        }
    );

    define_builtin_function!(
        BuiltinFunction::new_value_based, "Sin", map, vec![vec![MascalTypeKind::Float, MascalTypeKind::Integer]], false,
        |args, _| {
            match args.first().unwrap() {
                MascalValue::Integer(i) => Ok(Some(
                    MascalValue::Integer(IntegerNum::new(i.as_f64().sin() as i128))
                )),
                MascalValue::Float(f) => Ok(Some(MascalValue::Float(f.sin()))),
                _ => unreachable!()
            }
        }
    );

    define_builtin_function!(
        BuiltinFunction::new_value_based, "Tan", map, vec![vec![MascalTypeKind::Float, MascalTypeKind::Integer]], false,
        |args, _| {
            match args.first().unwrap() {
                MascalValue::Integer(i) => Ok(Some(
                    MascalValue::Integer(IntegerNum::new(i.as_f64().tan() as i128))
                )),
                MascalValue::Float(f) => Ok(Some(MascalValue::Float(f.tan()))),
                _ => unreachable!()
            }
        }
    );

    define_builtin_function!(
        BuiltinFunction::new_value_based, "Cos", map, vec![vec![MascalTypeKind::Float, MascalTypeKind::Integer]], false,
        |args, _| {
            match args.first().unwrap() {
                MascalValue::Integer(i) => Ok(Some(
                    MascalValue::Integer(IntegerNum::new(i.as_f64().cos() as i128))
                )),
                MascalValue::Float(f) => Ok(Some(MascalValue::Float(f.cos()))),
                _ => unreachable!()
            }
        }
    );

    define_builtin_function!(
        BuiltinFunction::new_value_based, "Floor", map, vec![vec![MascalTypeKind::Float]], false,
        |args, _| {
            match args.first().unwrap() {
                MascalValue::Float(f) => Ok(Some(MascalValue::Float(f.floor()))),
                _ => unreachable!()
            }
        }
    );

    define_builtin_function!(
        BuiltinFunction::new_value_based, "Ceil", map, vec![vec![MascalTypeKind::Float]], false,
        |args, _| {
            match args.first().unwrap() {
                MascalValue::Float(f) => Ok(Some(MascalValue::Float(f.ceil()))),
                _ => unreachable!()
            }
        }
    );

    define_builtin_function!(
        BuiltinFunction::new_value_based, "Round", map, vec![vec![MascalTypeKind::Float]], false,
        |args, _| {
            match args.first().unwrap() {
                MascalValue::Float(f) => Ok(Some(MascalValue::Float(f.round()))),
                _ => unreachable!()
            }
        }
    );

    define_builtin_function!(
        BuiltinFunction::new_value_based, "Len", map,
        vec![vec![MascalTypeKind::StaticArray, MascalTypeKind::DynamicArray]],
        false,
        |args, _exec_data| {
            let length: usize = match args.first().unwrap() {
                MascalValue::StaticArray(v) => v.len(),
                MascalValue::DynamicArray(v) => v.len(),
                _ => {unreachable!()}
            };
            Ok(Some(MascalValue::Integer(IntegerNum::new(length as i128))))
        }
    );

    define_builtin_function!(
        BuiltinFunction::new_value_based, "Dimensions", map,
        vec![vec![MascalTypeKind::StaticArray, MascalTypeKind::DynamicArray]],
        false,
        |args, _exec_data| {
            get_dimensions(&Some(args.first().unwrap().clone()), 0)
        }
    );

    define_builtin_function!(
        BuiltinFunction::new_value_based, "Sizes", map,
        vec![vec![MascalTypeKind::StaticArray, MascalTypeKind::DynamicArray]],
        false,
        |args, _exec_data| {
            get_sizes(&Some(args.first().unwrap().clone()), Vec::new())
        }
    );

    define_builtin_function!(
        BuiltinFunction::new_value_based, "Push", map,
        vec![vec![MascalTypeKind::DynamicArray]],
        true,
        |args, _exec_data| {
            let array: MascalValue = args[0].clone();
            if args.len() == 1 {
                return Err(MascalError {
                    error_type: MascalErrorType::ArgumentError,
                    line: 0,
                    character: 0,
                    source: String::from("Expected a value (element) to be pushed to the array")
                });
            }
            if let MascalValue::DynamicArray(mut v) = array {
                v.reserve(args.len());
                for val in args.into_iter().skip(1) {
                    v.push(Rc::new(RefCell::new(Some(val))));
                }
                return Ok(Some(MascalValue::DynamicArray(v)));
            }
            unreachable!()
        }
    );

    define_builtin_function!(
        BuiltinFunction::new_value_based, "Append", map,
        vec![vec![MascalTypeKind::DynamicArray]],
        true,
        |args, _exec_data| {
            let array: MascalValue = args[0].clone();
            if args.len() == 1 {
                return Err(MascalError {
                    error_type: MascalErrorType::ArgumentError,
                    line: 0,
                    character: 0,
                    source: String::from("Expected an array of elements to be appended to the array")
                });
            }
            if let MascalValue::DynamicArray(mut v) = array {
                let mut counter: usize = 0;
                for arg in args.iter().skip(1) {
                    counter += match arg {
                        MascalValue::StaticArray(v) => v.len(),
                        MascalValue::DynamicArray(v) => v.len(),
                        _ => {
                            return Err(MascalError {
                                error_type: MascalErrorType::ArgumentError,
                                line: 0,
                                character: 0,
                                source: String::from("Expected an array to push but got an atomic type")
                            });
                        }
                    };
                }
                v.reserve(counter);
                for arr_val in args.into_iter().skip(1) {
                    match arr_val {
                        MascalValue::DynamicArray(v_inner) => {
                            for val in v_inner {
                                v.push(Rc::new(RefCell::new(val.borrow().clone())));
                            }
                        }
                        _ => unreachable!()
                    }
                }
                return Ok(Some(MascalValue::DynamicArray(v)));
            }
            unreachable!()
        }
    );

    define_builtin_function!(
        BuiltinFunction::new_value_based, "Flatten", map,
        vec![vec![MascalTypeKind::DynamicArray]],
        false,
        |args, _exec_data| {
            let mut flat_cells = Vec::new();

            for arg in args.into_iter() {
                let cell = Rc::new(RefCell::new(Some(arg)));
                flat_cells.extend(flatten_impl(cell)?);
            }

            Ok(Some(MascalValue::DynamicArray(flat_cells)))
        }
    );

    define_builtin_function!(
        BuiltinFunction::new_value_based, "Pop", map,
        vec![vec![MascalTypeKind::DynamicArray]],
        false,
        |args, _exec_data| {
            return match &args[0] {
                MascalValue::DynamicArray(v) => {
                    let mut values_clone = v.clone();
                    values_clone.pop();
                    Ok(Some(MascalValue::DynamicArray(values_clone)))
                }
                _ => unreachable!()
            };
        }
    );

    define_builtin_function!(
        BuiltinFunction::new_value_based, "Shift", map,
        vec![vec![MascalTypeKind::DynamicArray]],
        false,
        |args, _exec_data| {
            return match &args[0] {
                MascalValue::DynamicArray(v) => {
                    let mut values_clone = v.clone();
                    values_clone.remove(0);
                    Ok(Some(MascalValue::DynamicArray(values_clone)))
                }
                _ => unreachable!()
            };
        }
    );

    define_builtin_function!(
        BuiltinFunction::new_expresion_based, "Read", map, vec![], true,
        |args, exec_data| {
            for arg in args {
                let varname: &str = match arg {
                    MascalExpression::SymbolicExpression(s) => s.as_str(),

                    _ => return Err(MascalError {
                        error_type: MascalErrorType::RuntimeError,
                        line: 0,
                        character: 0,
                        source: String::from("Expected an identifier for a variable name but found other expression")
                    }),
                };
                let execdata_ref = exec_data.borrow_mut();
                let extracted_vartable = execdata_ref.variable_table.as_ref()
                    .unwrap();
                let mut mutable_borrow_vartable = extracted_vartable.borrow_mut();
                if let Some(unwrapped_vardata) = mutable_borrow_vartable.get_mut(varname) {
                    if !unwrapped_vardata.array_dimensions.is_empty() {
                        return Err(MascalError {
                            error_type: MascalErrorType::RuntimeError,
                            line: 0,
                            character: 0,
                            source: format!("The variable called {:?} is an array type which is unsupported", varname)
                        })
                    }
                    let atomic_type: &MascalType = unwrapped_vardata.atomic_variable_type.as_ref();
                    let mut input: String = String::new();
                    let result: Result<usize, _> = io::stdin().read_line(&mut input);
                    let input_str: &str = input.trim();
                    if result.is_err() {
                        return Err(MascalError {
                            error_type: MascalErrorType::InputError,
                            line: 0,
                            character: 0,
                            source: String::from("Could not read user input")
                        })
                    }
                    let read_value: MascalValue = match atomic_type {
                        MascalType::Integer => {
                            let int: Result<i128, ParseIntError> = input_str.parse::<i128>();
                            if int.is_err() {
                                return Err(MascalError {
                                    error_type: MascalErrorType::InputError,
                                    line: 0,
                                    character: 0,
                                    source: String::from("The user input cannot be parsed as an integer")
                                })
                            }
                            MascalValue::Integer(IntegerNum::new(int.unwrap()))
                        }
                        MascalType::Float => {
                            let int: Result<f64, ParseFloatError> = input_str.parse::<f64>();
                            if int.is_err() {
                                return Err(MascalError {
                                    error_type: MascalErrorType::InputError,
                                    line: 0,
                                    character: 0,
                                    source: String::from("The user input cannot be parsed as a float")
                                })
                            }
                            MascalValue::Float(int.unwrap())
                        }
                        MascalType::Boolean => {
                            match input_str {
                                "true" | "TRUE" => MascalValue::Boolean(true),
                                "false" | "FALSE" => MascalValue::Boolean(false),
                                _ => {
                                    return Err(MascalError {
                                        error_type: MascalErrorType::InputError,
                                        line: 0,
                                        character: 0,
                                        source: String::from("The user input cannot be parsed as a boolean")
                                    })
                                }
                            }
                        }
                        MascalType::String => {MascalValue::String(Arc::from(input_str))}
                        _ => {return Err(MascalError {
                            error_type: MascalErrorType::TypeError,
                            line: 0,
                            character: 0,
                            source: String::from("This type of variable is unsupported when reading a user input")
                        })}
                    };
                    let value_ref = &mut unwrapped_vardata.value;
                    *value_ref = Some(Rc::new(RefCell::new(read_value)));
                } else {
                    return Err(MascalError {
                        error_type: MascalErrorType::RuntimeError,
                        line: 0,
                        character: 0,
                        source: format!("The variable name {:?} does not exist", varname)
                    })
                }
            }
            Ok(None)
        }
    );
    map
});