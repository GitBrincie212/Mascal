use std::collections::HashMap;
use std::sync::Arc;
use once_cell::sync::Lazy;
use crate::defs::errors::MascalError;
use crate::defs::types::MascalType;
use crate::runtime::values::MascalValue;

#[derive(Clone)]
pub struct BuiltInFunction {
    pub(crate) fixed_argument_types: Vec<MascalType>,
    pub(crate) supports_dynamic_arguments: bool,
    pub(crate) execution: Arc<dyn Fn(Vec<MascalValue>) -> Result<Option<MascalValue>, MascalError> + Send + Sync>,
}

macro_rules! define_builtin_function {
    ($name: expr, $map: expr, $fixed_args: expr, $supports_dynamic_arguments: expr, $func: expr) => {
        let func: Arc<BuiltInFunction> = Arc::new(BuiltInFunction {
            fixed_argument_types: $fixed_args,
            execution: Arc::new($func),
            supports_dynamic_arguments: $supports_dynamic_arguments,
        });
        $map.insert(String::from($name).to_uppercase(), Arc::clone(&func));
        $map.insert(String::from($name), Arc::clone(&func));
        $map.insert(String::from($name).to_lowercase(), Arc::clone(&func));
    };
}

pub static BUILT_IN_FUNCTION_TABLE: Lazy<HashMap<String, Arc<BuiltInFunction>>>  = Lazy::new(|| {
    let mut map: HashMap<String, Arc<BuiltInFunction>> = HashMap::new();
    define_builtin_function!("Write", map, Vec::new(), true, |args| {
        for val in args {
            print!("{}", val.as_string());
        }
        print!("\n");
        Ok(None)
    });

    define_builtin_function!("Read", map, Vec::new(), true, |_args| {
        Ok(None)
    });
    map
});