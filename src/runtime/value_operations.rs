use crate::defs::InfinityControl;
use crate::defs::errors::{MascalError, MascalErrorType};
use crate::runtime::values::MascalValue;

macro_rules! error_float_overflow {
    ($l: expr, $r: expr) => {
        Err(MascalError {
            character: 0,
            line: 0,
            error_type: MascalErrorType::OverflowError,
            source: format!(
                "Float has been overflowed which was caused by the addition of {:?} and {:?}",
                $l, $r
            ),
        })
    };
}

macro_rules! unsupported_operation_error {
    ($left: expr, $right: expr) => {
        Err(MascalError {
            character: 0,
            line: 0,
            error_type: MascalErrorType::UndefinedOperation,
            source: format!(
                "Cannot operate between the values {:?} and {:?}",
                $left.as_string(),
                $right.as_string()
            ),
        })
    };
}

macro_rules! define_arithmetic_fn {
    (
        $fn_name:expr, $left: expr, $right:expr, $infinity_control: expr, 
        $intmeth:ident, $floatop:tt, $( $extra_pat:pat => $extra_expr:expr ),* $(,)?
    ) => {
            match ($left, $right) {
                (MascalValue::Integer(l), MascalValue::Integer(r)) => {
                    Ok(MascalValue::Integer(l.$intmeth(r, $infinity_control)?))
                }

                (l @ MascalValue::Float(_), MascalValue::Integer(r)) => {
                    $fn_name(l, MascalValue::Float(r.as_f64()), $infinity_control)
                }

                (MascalValue::Integer(l), r @ MascalValue::Float(_)) => {
                    $fn_name(MascalValue::Float(l.as_f64()), r, $infinity_control)
                }

                (MascalValue::Float(l), MascalValue::Float(r)) => {
                    match &$infinity_control {
                        &InfinityControl::AllowInfinityExtra => {
                            Ok(MascalValue::Float(l $floatop r))
                        }

                        &InfinityControl::AllowInfinity
                        | &InfinityControl::DisallowInfinity => {
                            if $infinity_control == &InfinityControl::DisallowInfinity && (l.is_infinite() || r.is_infinite()) {
                                return error_float_overflow!(l, r);
                            }
                            let res = l $floatop r;
                            if res.is_infinite() && !l.is_infinite() && !r.is_infinite() {
                                return error_float_overflow!(l, r);
                            }
                            Ok(MascalValue::Float(res))
                        }
                    }
                }

                $( $extra_pat => $extra_expr, )*

                (l, r) => unsupported_operation_error!(l, r),
            }
    };

    // Case *without* extra arms (pure numeric)
    ($fn_name:expr, $left: expr, $right:expr, $infinity_control: expr, $intmeth:ident, $floatop:tt) => {
        define_arithmetic_fn!($fn_name, $left, $right, $infinity_control, $intmeth, $floatop, )
    };
}

impl MascalValue {
    pub fn add(
        left: MascalValue,
        right: MascalValue,
        infinity_control: &InfinityControl
    ) -> Result<MascalValue, MascalError> {
        define_arithmetic_fn!(Self::add, left, right, infinity_control, add, +,
            (MascalValue::String(l), MascalValue::String(r)) => {
                Ok(MascalValue::String(l + r.as_str()))
            },

            (MascalValue::DynamicArray(l), MascalValue::DynamicArray(r)) => {
                Ok(MascalValue::DynamicArray(l.into_iter().chain(r.into_iter()).collect()))
            }
        )
    }

    pub fn sub(
        left: MascalValue,
        right: MascalValue,
        infinity_control: &InfinityControl
    ) -> Result<MascalValue, MascalError> {
        define_arithmetic_fn!(Self::sub, left, right, infinity_control, sub, -)
    }

    pub fn mul(
        left: MascalValue,
        right: MascalValue,
        infinity_control: &InfinityControl
    ) -> Result<MascalValue, MascalError> {
        define_arithmetic_fn!(Self::mul, left, right, infinity_control, mul, *)
    }

    pub fn div(
        left: MascalValue,
        right: MascalValue,
        infinity_control: &InfinityControl,
    ) -> Result<MascalValue, MascalError> {
        match (left, right) {
            (MascalValue::Integer(l), MascalValue::Integer(r)) => {
                Ok(MascalValue::Integer(l.div(r, &infinity_control)?))
            }

            (l, MascalValue::Integer(r)) => {
                MascalValue::div(l, MascalValue::Float(r.as_f64()), &infinity_control)
            }

            (MascalValue::Integer(l), r) => {
                MascalValue::div(MascalValue::Float(l.as_f64()), r, &infinity_control)
            }

            (MascalValue::Float(l), MascalValue::Float(r)) => match infinity_control.clone() {
                InfinityControl::AllowInfinityExtra => {
                    if r == 0f64 {
                        return Err(MascalError {
                            character: 0,
                            line: 0,
                            error_type: MascalErrorType::UndefinedOperation,
                            source: String::from("Cannot divide by zero")
                        })
                    }

                    Ok(MascalValue::Float(l / r))
                },

                InfinityControl::AllowInfinity | InfinityControl::DisallowInfinity => {
                    if infinity_control == &InfinityControl::DisallowInfinity
                        && (l.is_infinite() || r.is_infinite())
                    {
                        return error_float_overflow!(l, r);
                    }
                    let res = l / r;
                    if res.is_infinite() && !l.is_infinite() && !r.is_infinite() {
                        return error_float_overflow!(l, r);
                    }
                    Ok(MascalValue::Float(res))
                }
            },

            (l, r) => unsupported_operation_error!(l, r),
        }
    }
}
