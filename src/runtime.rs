mod execute_binary_expression;
mod execute_builtin_function;
mod execute_declaration_statement;
pub mod execute_expression;
mod execute_function_expression;
mod execute_statement;
pub mod execute_typecast;
mod execute_unary_expression;
pub mod utils;
pub mod values;
mod variable_table;

use crate::ast::AbstractSyntaxTree;
use crate::defs::blocks::{ExecutionBlock, ScopedBlocks};
use crate::defs::errors::MascalError;
use crate::runtime::execute_statement::{SemanticContext, execute_statement};
use crate::runtime::variable_table::{VariableTable, create_variable_table};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::{Arc, LazyLock, Mutex};

pub struct ExecutionData {
    pub variable_table: Option<Rc<RefCell<VariableTable>>>,
    pub scoped_blocks: Rc<RefCell<Vec<ScopedBlocks>>>,
}

pub static FUNCTION_HASHSET: LazyLock<Mutex<HashSet<Arc<str>>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

pub fn interpert(abstract_syntax_tree: AbstractSyntaxTree) -> Result<(), MascalError> {
    let mut scoped_blocks: Vec<ScopedBlocks> = abstract_syntax_tree.blocks;
    let program_block: ScopedBlocks = scoped_blocks.remove(abstract_syntax_tree.program_index);
    let mut exec_block: ExecutionBlock = match program_block {
        ScopedBlocks::Program(exec_block) => exec_block,
        ScopedBlocks::Function { .. } => {
            unreachable!()
        }
    };
    let scoped_variable_table: Rc<RefCell<VariableTable>>;
    (scoped_variable_table, exec_block) = create_variable_table(exec_block)?;
    let containerized_scoped_blocks: Rc<RefCell<Vec<ScopedBlocks>>> =
        Rc::new(RefCell::new(scoped_blocks));
    let semantic_context: Rc<SemanticContext> = Rc::new(SemanticContext {
        variable_table: scoped_variable_table.clone(),
        scoped_blocks: containerized_scoped_blocks.clone(),
        in_loop: false,
        function_name: None,
    });
    for statement in exec_block.body.into_iter() {
        execute_statement(statement, semantic_context.clone())?;
    }
    Ok(())
}
