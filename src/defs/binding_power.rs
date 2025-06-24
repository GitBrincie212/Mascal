use crate::defs::expressions::MascalExpression;
use crate::defs::literal::MascalLiteral;
use crate::defs::operators::{MascalBinaryOperators, MascalUnaryOperators};

#[derive(Hash, Clone)]
pub struct BindingPower {
    pub left_binding_power: usize,
    pub right_binding_power: usize,
}

impl BindingPower {
    pub fn new(binding_power_num: usize) -> Self {
        BindingPower {
            left_binding_power: binding_power_num,
            right_binding_power: binding_power_num + 1,
        }
    }
}

pub fn get_binding_power(expression: MascalExpression) -> BindingPower {
    match expression {
        MascalExpression::Call { .. } => BindingPower::new(100),
        MascalExpression::Unary { .. } => BindingPower {
            left_binding_power: 0,
            right_binding_power: 90,
        },
        MascalExpression::Binary { operator, .. } => match operator {
            MascalBinaryOperators::Exponentiation => BindingPower::new(80),

            MascalBinaryOperators::Divide
            | MascalBinaryOperators::Modulo
            | MascalBinaryOperators::Multiply => BindingPower::new(70),

            MascalBinaryOperators::GreaterThan
            | MascalBinaryOperators::LessThan
            | MascalBinaryOperators::GreaterThanOrEqual
            | MascalBinaryOperators::LessThanOrEqual => BindingPower::new(40),

            MascalBinaryOperators::Equals | MascalBinaryOperators::NotEqual => {
                BindingPower::new(35)
            }

            MascalBinaryOperators::And => BindingPower::new(30),
            MascalBinaryOperators::Or => BindingPower::new(29),

            _ => BindingPower::new(60),
        },
        _ => BindingPower::new(0),
    }
}

pub fn get_binding_power_from_bsign(sign: MascalBinaryOperators) -> BindingPower {
    get_binding_power(MascalExpression::Binary {
        left: Box::new(MascalExpression::Literal(MascalLiteral::Null)),
        operator: sign,
        right: Box::new(MascalExpression::Literal(MascalLiteral::Null)),
    })
}

pub fn get_binding_power_from_psign(sign: MascalUnaryOperators) -> BindingPower {
    get_binding_power(MascalExpression::Unary {
        value: Box::new(MascalExpression::Literal(MascalLiteral::Null)),
        operator: sign,
    })
}
