use super::*;
use crate::codegen::expressions::expression;
use crate::utils::symbol_table::create_symbol_table;
use crate::utils::Grouping;
use crate::utils::SymbolTable;

pub fn codewrite(class_grouping: &Grouping) -> Result<String, String> {
    let class_symbol_table = create_symbol_table(class_grouping)?;
    compile_expressions(class_grouping, &class_symbol_table);
    return Ok("".to_string());
}

fn compile_expressions(grouping: &Grouping, symbol_table: &SymbolTable) {
    for sg in grouping.subgroupings() {
        if sg.name == "expression" {
            println!("{}", sg.as_xml());
            println!("{}", expression(sg, symbol_table).unwrap());
        } else {
            compile_expressions(sg, symbol_table);
        }
    }
}
