use crate::defs::errors::{MascalError, MascalErrorType};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IntegerNum {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128)
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
    
    pub fn as_string(&self) -> String {
        self.to_i128().to_string()
    }

    pub fn as_f64(&self) -> f64 {
        self.to_i128() as f64
    }
    
    pub fn is_negative_or_zero(&self) -> bool {
        self.to_i128().is_negative() || self.to_i128() == 0
    }

    pub fn to_i128(&self) -> i128 {
        match self {
            IntegerNum::I8(v) => *v as i128,
            IntegerNum::I16(v) => *v as i128,
            IntegerNum::I32(v) => *v as i128,
            IntegerNum::I64(v) => *v as i128,
            IntegerNum::I128(v) => *v,
        }
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

    pub fn add(&self, other: IntegerNum) -> Result<IntegerNum, MascalError> {
        promotion_process(self, &other, i128::overflowing_add)
    }

    pub fn sub(&self, other: IntegerNum) -> Result<IntegerNum, MascalError> {
        promotion_process(self, &other, i128::overflowing_sub)
    }

    pub fn mul(&self, other: IntegerNum) -> Result<IntegerNum, MascalError> {
        promotion_process(self, &other, i128::overflowing_mul)
    }

    pub fn div(&self, other: IntegerNum) -> Result<IntegerNum, MascalError> {
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

    pub fn neg(&self) -> Result<IntegerNum, MascalError> {
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

    pub fn modulo(&self, other: IntegerNum) -> Result<IntegerNum, MascalError> {
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

    pub fn isqrt(&self) -> Result<IntegerNum, MascalError> {
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
    
    fn logarithm_operation_pipeline(&self) -> Result<i128, MascalError> {
        let num: i128 = self.to_i128();
        if num <= 0 {
            return Err(MascalError {
                error_type: MascalErrorType::UndefinedOperation,
                line: 0,
                character: 0,
                source: String::from("Cannot use the logarithm operation with a negative or zero value"),
            });
        }
        
        Ok(num)
    }

    pub fn log2(&self) -> Result<IntegerNum, MascalError> {
        let num: i128 = self.logarithm_operation_pipeline()?;
        Ok(IntegerNum::new(num.ilog2() as i128))
    }

    pub fn log10(&self) -> Result<IntegerNum, MascalError> {
        let num: i128 = self.logarithm_operation_pipeline()?;
        Ok(IntegerNum::new(num.ilog10() as i128))
    }

    pub fn ln(&self) -> Result<IntegerNum, MascalError> {
        let num: i128 = self.logarithm_operation_pipeline()?;
        Ok(IntegerNum::new((num as f32).ln().round() as i128))
    }

    pub fn exponentation(&self, other: &IntegerNum) -> Result<IntegerNum, MascalError> {
        let other_val: i128 = other.to_i128();
        let self_val: i128 = self.to_i128();
        if self_val <= 0 {
            return Err(MascalError {
                character: 0,
                line: 0,
                error_type: MascalErrorType::UndefinedOperation,
                source: String::from("Cannot perform exponentation with a negative or zero base")
            })
        }
        if other_val <= 0 {
            return Ok(IntegerNum::new(1i128 / num_traits::pow(self_val, other_val.abs() as usize)));
        }
        Ok(IntegerNum::new(num_traits::pow(self_val,  other_val as usize)))
    }
}