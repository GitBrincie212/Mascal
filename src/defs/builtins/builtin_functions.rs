use std::cell::{RefCell};
use std::collections::HashMap;
use std::io;
use std::num::{ParseFloatError, ParseIntError};
use std::rc::Rc;
use std::sync::{Arc};
use once_cell::sync::Lazy;
use crate::defs::dynamic_int::IntegerNum;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::expressions::MascalExpression;
use crate::defs::types::MascalType;
use crate::runtime::ExecutionData;
use crate::runtime::values::MascalValue;

#[derive(Clone)]
pub enum BuiltinFunction {
    ValueBased {
        fixed_argument_types: Vec<MascalType>,
        supports_dynamic_arguments: bool,
        execution: Arc<dyn Fn(Vec<MascalValue>, Rc<RefCell<ExecutionData>>) -> Result<Option<MascalValue>, MascalError> + Send + Sync>,
    },
    
    ExpressionBased {
        fixed_argument_types: Vec<MascalType>,
        supports_dynamic_arguments: bool,
        execution: Arc<dyn Fn(Vec<&MascalExpression>, Rc<RefCell<ExecutionData>>) -> Result<Option<MascalValue>, MascalError> + Send + Sync>,
    }
}

impl BuiltinFunction {
    fn new_value_based(
        supports_dynamic_arguments: bool,
        fixed_argument_types: Vec<MascalType>,
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
        fixed_argument_types: Vec<MascalType>,
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
    ($builtin_construct: expr, $name: expr, $map: expr, $fixed_args: expr, $supports_dynamic_arguments: expr, $func: expr) => {
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
                print!("{}", val.as_string());
            }
            print!("\n");
            Ok(None)
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
                if let Some(unwrapped_vardata) = extracted_vartable.borrow_mut().get_mut(varname) {
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
                            let int: Result<i128, ParseIntError> = input.parse::<i128>();
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
                            let int: Result<f64, ParseFloatError> = input.parse::<f64>();
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
                            match input.as_str() {
                                "true" | "TRUE" => MascalValue::Boolean(true),
                                "false" | "FALSE" => MascalValue::Boolean(false),
                                _ => {
                                    return Err(MascalError {
                                        error_type: MascalErrorType::InputError,
                                        line: 0,
                                        character: 0,
                                        source: String::from("The user input cannot be parsed as an integer")
                                    })
                                }
                            }
                        }
                        MascalType::String => {MascalValue::String(Arc::new(input))}
                        _ => {return Err(MascalError {
                            error_type: MascalErrorType::TypeError,
                            line: 0,
                            character: 0,
                            source: String::from("This type of variable is unsupported when reading a user input")
                        })}
                    };
                    let value_ref = &mut unwrapped_vardata.value;
                    *value_ref = Some(Rc::new(RefCell::new(read_value)));
                    let execdata_mut_ref = exec_data.borrow_mut();
                    execdata_mut_ref.variable_table
                        .as_ref()
                        .unwrap()
                        .borrow_mut()
                        .insert(varname.to_string(), unwrapped_vardata.clone());
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