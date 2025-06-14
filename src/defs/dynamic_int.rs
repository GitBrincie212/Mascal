use crate::defs::errors::{MascalError, MascalErrorType};
use crate::defs::InfinityControl;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IntegerNum {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    PositiveInfinity,
    NegativeInfinity
}

fn promotion_process<F>(
    num1: &IntegerNum, num2: &IntegerNum, func: F
) -> Result<IntegerNum, MascalError> where F: Fn(i128, i128) -> (i128, bool) {
    let (num, did_overflow): (i128, bool) = func(num1.to_i128(), num2.to_i128());
    if !did_overflow {
        return Ok(IntegerNum::new(num))
    }

    Err(MascalError {
        character: 0,
        line: 0,
        error_type: MascalErrorType::OverflowError,
        source: String::from("Integer overflow beyond i128 range"),
    })
}

impl IntegerNum {
    pub fn new(val: i128) -> IntegerNum {
        if (i8::MIN as i128..i8::MAX as i128).contains(&val) {IntegerNum::I8(val as i8)}
        else if (i16::MIN as i128..i16::MAX as i128).contains(&val) {IntegerNum::I16(val as i16)}
        else if (i32::MIN as i128..i32::MAX as i128).contains(&val) {IntegerNum::I32(val as i32)}
        else if (i64::MIN as i128..i64::MAX as i128).contains(&val) {IntegerNum::I64(val as i64)}
        else {IntegerNum::I128(val)}
    }

    fn to_i128(&self) -> i128 {
        match self {
            IntegerNum::I8(v) => *v as i128,
            IntegerNum::I16(v) => *v as i128,
            IntegerNum::I32(v) => *v as i128,
            IntegerNum::I64(v) => *v as i128,
            IntegerNum::I128(v) => *v,
            _ => {0}
        }
    }
    
    fn explicit_declaration_for_infinity_verfiy(
        &self, other: &IntegerNum, explicit_inf: &InfinityControl
    ) -> Result<(), MascalError> {
        if explicit_inf == &InfinityControl::DISALLOW_INFINITY && (other == &IntegerNum::PositiveInfinity
            || other == &IntegerNum::NegativeInfinity
            || self == &IntegerNum::NegativeInfinity
            || self == &IntegerNum::PositiveInfinity) {
            return Err(MascalError {
                error_type: MascalErrorType::NonExplicitInfiniteDeclarationError,
                line: 0,
                character: 0,
                source: String::from("Value has not been explicitly declared to include infinity"),
            });
        }
        Ok(())
    }
    
    fn explicit_decleration_for_infinity_verify_self(
        &self, explicit_inf: &InfinityControl
    ) -> Result<(), MascalError> {
        if explicit_inf == &InfinityControl::DISALLOW_INFINITY
            && (self == &IntegerNum::NegativeInfinity || self == &IntegerNum::PositiveInfinity) {
            return Err(MascalError {
                error_type: MascalErrorType::NonExplicitInfiniteDeclarationError,
                line: 0,
                character: 0,
                source: String::from("Value has not been explicitly declared to include infinity"),
            });
        }
        Ok(())
    }

    fn verify_infinity_case(&self, other: &IntegerNum, explicit_inf: &InfinityControl) -> Result<(), MascalError> {
        if explicit_inf != &InfinityControl::DISALLOW_INFINITY {
            return match (self, other) {
                (IntegerNum::PositiveInfinity, IntegerNum::PositiveInfinity) => return Ok(()),
                (IntegerNum::NegativeInfinity, IntegerNum::NegativeInfinity) => return Ok(()),
                (IntegerNum::NegativeInfinity, IntegerNum::PositiveInfinity) |
                (IntegerNum::PositiveInfinity, IntegerNum::NegativeInfinity) => return Err(MascalError {
                    error_type: MascalErrorType::UnallowedInfinityOperationError,
                    line: 0,
                    character: 0,
                    source: String::from("Cannot operate this operation with infinities that have different signs"),
                }),
                _ => {Ok(())}
            };
        }
        self.explicit_declaration_for_infinity_verfiy(other, explicit_inf)?;
        
        Ok(())
    }

    pub fn max(&self, other: &IntegerNum) -> IntegerNum {
        let num_other: i128 = self.to_i128();
        let self_num: i128 = other.to_i128();
        if self_num > num_other  { IntegerNum::new(self_num) } else { IntegerNum::new(num_other) }
    }

    pub fn min(&self, other: &IntegerNum) -> IntegerNum {
        let num_other: i128 = self.to_i128();
        let self_num: i128 = other.to_i128();
        if self_num < num_other  { IntegerNum::new(self_num) } else { IntegerNum::new(num_other) }
    }

    pub fn add(&self, other: IntegerNum, explicit_inf: InfinityControl) -> Result<IntegerNum, MascalError> {
        self.verify_infinity_case(&other, &explicit_inf)?;
        promotion_process(self, &other, i128::overflowing_add)
    }

    pub fn sub(&self, other: IntegerNum, explicit_inf: InfinityControl) -> Result<IntegerNum, MascalError> {
        self.verify_infinity_case(&other, &explicit_inf)?;
        promotion_process(self, &other, i128::overflowing_sub)
    }

    pub fn mul(&self, other: IntegerNum, explicit_inf: InfinityControl) -> Result<IntegerNum, MascalError> {
        self.explicit_declaration_for_infinity_verfiy(&other, &explicit_inf)?;
        promotion_process(self, &other, i128::overflowing_mul)
    }

    pub fn div(&self, other: IntegerNum, explicit_inf: InfinityControl) -> Result<IntegerNum, MascalError> {
        self.explicit_declaration_for_infinity_verfiy(&other, &explicit_inf)?;
        if other.to_i128() == 0 {
            return Err(MascalError {
                error_type: MascalErrorType::UndefinedOperation,
                line: 0,
                character: 0,
                source: String::from("Division by zero"),
            })
        }
        
        promotion_process(self, &other, i128::overflowing_div)
    }

    pub fn neg(&self, explicit_inf: InfinityControl) -> Result<IntegerNum, MascalError> {
        self.explicit_decleration_for_infinity_verify_self(&explicit_inf)?;
        let (num, did_overflow): (i128, bool) = self.to_i128().overflowing_neg();
        if !did_overflow {
            return Ok(IntegerNum::new(num))
        }

        Err(MascalError {
            character: 0,
            line: 0,
            error_type: MascalErrorType::OverflowError,
            source: String::from("Integer overflow beyond i128 range"),
        })
    }

    pub fn modulo(&self, other: IntegerNum, explicit_inf: InfinityControl) -> Result<IntegerNum, MascalError> {
        self.explicit_declaration_for_infinity_verfiy(&other, &explicit_inf)?;
        if other.to_i128() == 0 {
            return Err(MascalError {
                character: 0,
                line: 0,
                error_type: MascalErrorType::UndefinedOperation,
                source: String::from("Modulo by zero"),
            });
        }

        promotion_process(self, &other, |num1, num2| {
            let overflow: bool = num1 == i128::MIN && num2 == -1;
            let result: i128 = num1 % num2;
            (result, overflow)
        })
    }

    pub fn isqrt(&self, explicit_inf: InfinityControl) -> Result<IntegerNum, MascalError> {
        self.explicit_decleration_for_infinity_verify_self(&explicit_inf)?;
        let num = self.to_i128();
        if num < 0 {
            return Err(MascalError {
                character: 0,
                line: 0,
                error_type: MascalErrorType::UndefinedOperation,
                source: String::from("Cannot get the square root of a negative number"),
            });
        }

        Ok(IntegerNum::new(num.isqrt()))
    }
    
    fn logarithm_operation_pipeline(&self, explicit_inf: &InfinityControl) -> Result<i128, MascalError> {
        self.explicit_decleration_for_infinity_verify_self(explicit_inf)?;
        let num: i128 = self.to_i128();
        if num < 0 {
            return Err(MascalError {
                error_type: MascalErrorType::UndefinedOperation,
                line: 0,
                character: 0,
                source: String::from(" by zero"),
            });
        } else if num == 0 {
            return Err(MascalError {
                error_type: MascalErrorType::UndefinedOperation,
                line: 0,
                character: 0,
                source: String::from(" by zero"),
            })
        }
        
        Ok(num)
    }

    pub fn log2(&self, explicit_inf: InfinityControl) -> Result<IntegerNum, MascalError> {
        let num: i128 = self.logarithm_operation_pipeline(&explicit_inf)?;
        Ok(IntegerNum::new(num.ilog2() as i128))
    }

    pub fn log10(&self, explicit_inf: InfinityControl) -> Result<IntegerNum, MascalError> {
        let num: i128 = self.logarithm_operation_pipeline(&explicit_inf)?;
        Ok(IntegerNum::new(num.ilog10() as i128))
    }
}