pub mod errors;
pub mod token;
pub mod types;
pub mod operators;
pub mod literal;
pub mod blocks;
pub mod statements;
pub mod expressions;
pub mod declerations;
pub mod binding_power;
pub mod dynamic_int;


#[derive(PartialEq, Eq, Clone, Debug)]
pub enum InfinityControl {
    DISALLOW_INFINITY,
    ALLOW_INFINITY,
    ALLOW_INFINITY_EXTRA
}