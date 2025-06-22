#[macro_export]
macro_rules! comparison_arms {
    ( $left: expr, $right: expr, $method:ident, $fallback: expr ) => {
        match ($left, $right) {
            (MascalValue::Integer(i1), MascalValue::Integer(i2)) => {
                Ok(MascalValue::Boolean(i1.to_i128().$method(&i2.to_i128())))
            }

            (MascalValue::Float(f), MascalValue::Integer(i)) => {
                Ok(MascalValue::Boolean(f.$method(&i.as_f64())))
            }

            (MascalValue::Integer(i), MascalValue::Float(f)) => {
                Ok(MascalValue::Boolean(f.$method(&i.as_f64())))
            }

            (MascalValue::Float(f1), MascalValue::Float(f2)) => {
                Ok(MascalValue::Boolean(f1.$method(&f2)))
            }

            (MascalValue::String(s1), MascalValue::String(s2)) => {
                Ok(MascalValue::Boolean(s1.$method(s2)))
            },
            
            (v1, v2) => $fallback(v1.clone(), v2.clone())
        }
    };
}

#[macro_export]
macro_rules! define_arithmetic_fn {
    (
        $fn_name:expr, $left: expr, $right:expr, $intmeth:ident, 
        $floatop:tt, $( $extra_pat:pat => $extra_expr:expr ),* $(,)?
    ) => {
            match ($left, $right) {
                (MascalValue::Integer(l), MascalValue::Integer(r)) => {
                    Ok(MascalValue::Integer(l.$intmeth(r)?))
                }

                (l @ MascalValue::Float(_), MascalValue::Integer(r)) => {
                    $fn_name(l, MascalValue::Float(r.as_f64()))
                }

                (MascalValue::Integer(l), r @ MascalValue::Float(_)) => {
                    $fn_name(MascalValue::Float(l.as_f64()), r)
                }

                (MascalValue::Float(l), MascalValue::Float(r)) => {
                    let res = l $floatop r;
                    if res.is_infinite() && !l.is_infinite() && !r.is_infinite() {
                        return error_float_overflow!(l, r);
                    }
                    Ok(MascalValue::Float(res))
                }

                $( $extra_pat => $extra_expr, )*

                (l, r) => unsupported_operation_error!(l, r),
            }
    };

    ($fn_name:expr, $left: expr, $right:expr, $intmeth:ident, $floatop:tt) => {
        define_arithmetic_fn!($fn_name, $left, $right, $intmeth, $floatop, )
    };
}

#[macro_export]
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

#[macro_export]
macro_rules! unsupported_operation_error {
    ($left: expr, $right: expr) => {
        Err(MascalError {
            character: 0,
            line: 0,
            error_type: MascalErrorType::UndefinedOperation,
            source: format!(
                "Cannot operate between the values {:?} and {:?}",
                $left.as_string()?,
                $right.as_string()?
            ),
        })
    };
}