use std::collections::HashMap;
use std::sync::Arc;
use once_cell::sync::Lazy;
use crate::defs::errors::MascalError;
use crate::defs::expressions::MascalExpression;
use crate::defs::types::MascalType;
use crate::runtime::values::MascalValue;

#[derive(Clone)]
pub enum BuiltinFunction {
    ValueBased {
        fixed_argument_types: Vec<MascalType>,
        supports_dynamic_arguments: bool,
        execution: Arc<dyn Fn(Vec<MascalValue>) -> Result<Option<MascalValue>, MascalError> + Send + Sync>,
    },
    
    ExpressionBased {
        fixed_argument_types: Vec<MascalType>,
        supports_dynamic_arguments: bool,
        execution: Arc<dyn Fn(Vec<&MascalExpression>) -> Result<Option<MascalValue>, MascalError> + Send + Sync>,
    }
}

impl BuiltinFunction {
    fn new_value_based(
        supports_dynamic_arguments: bool,
        fixed_argument_types: Vec<MascalType>,
        execution: Arc<dyn Fn(Vec<MascalValue>) -> Result<Option<MascalValue>, MascalError> + Send + Sync>,
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
        execution: Arc<dyn Fn(Vec<&MascalExpression>) -> Result<Option<MascalValue>, MascalError> + Send + Sync>,
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
    define_builtin_function!(BuiltinFunction::new_value_based, "Write", map, vec![], true, |args| {
        for val in args {
            print!("{}", val.as_string());
        }
        print!("\n");
        Ok(None)
    });

    define_builtin_function!(BuiltinFunction::new_value_based, "Read", map, vec![], true, |_args| {
        Ok(None)
    });
    map
});