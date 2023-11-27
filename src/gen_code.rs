use crate::assembly::*;
use crate::ast::*;
use std::collections::HashMap;

pub struct MetaInfo {
    variable_map: HashMap<String, u32>,
    label_count: u64,
    label_stack_for_break: Vec<String>,
}

impl MetaInfo {
    pub fn new() -> MetaInfo {
        MetaInfo {
            variable_map: HashMap::new(),
            label_count: 0,
            label_stack_for_break: Vec::new(),
        }
    }

    pub fn get_variable_id_and_register_it(&mut self, lval: &String) -> u32 {
        match self.variable_map.get(lval) {
            Some(id) => *id,
            None => {
                let number_of_variables = self.get_number_of_variables();
                self.variable_map.insert(lval.clone(), number_of_variables);
                number_of_variables
            }
        }
    }

    pub fn get_number_of_variables(&self) -> u32 {
        self.variable_map.len() as u32
    }

    pub fn get_new_label(&mut self) -> String {
        self.label_count += 1;
        format!(".L{}", self.label_count)
    }

    pub fn push_label_for_break(&mut self, label: String) {
        self.label_stack_for_break.push(label);
    }

    pub fn pop_label_for_break(&mut self) -> String {
        self.label_stack_for_break.pop().unwrap()
    }

    pub fn get_label_for_break(&mut self) -> String {
        self.label_stack_for_break.last().unwrap().clone()
    }
}

impl Default for MetaInfo {
    fn default() -> Self {
        Self::new()
    }
}

pub fn print_assembly(program: &Program) {
    let mut meta_info = MetaInfo::default();
    print_assembly_internal(program, &mut meta_info);
}

fn print_assembly_internal(program: &Program, meta_info: &mut MetaInfo) {
    println!(".intel_syntax noprefix");
    println!(".global main\n");
    println!("main:");
    let header_code = vec![push(rbp()), mov(rbp(), rsp())];
    let mut main_code: Assembly = Vec::new();
    for statement in program.statements.iter() {
        main_code.append(&mut get_assembly_statement(statement, meta_info));
        main_code.push(pop(rax()));
    }
    let number_of_variables = meta_info.get_number_of_variables();
    let sub_rsp_code = sub(rsp(), immediate(8 * number_of_variables as i32));
    let footer_code = vec![mov(rsp(), rbp()), pop(rbp()), ret()];

    print_assembly_code(&header_code);
    print_single_instruction(&sub_rsp_code);
    print_assembly_code(&main_code);
    print_assembly_code(&footer_code);
}

pub fn get_assembly_statement(statement: &Statement, meta_info: &mut MetaInfo) -> Assembly {
    match statement {
        Statement::Expr(expr) => get_assembly_expr(expr, meta_info),
        Statement::Assign(left, expr) => {
            let mut assembly: Assembly = Vec::new();
            assembly.push(comment("assign"));
            assembly.append(&mut get_assembly_lval(left, meta_info));
            assembly.append(&mut get_assembly_expr(expr, meta_info));
            assembly.append(&mut vec![
                pop(rdi()),
                pop(rax()),
                mov(m_rax(), rdi()),
                push(rdi()),
            ]);
            assembly.push(comment("assign end"));
            assembly
        }
        Statement::Return(expr) => {
            let mut assembly: Assembly = get_assembly_expr(expr, meta_info);
            assembly.append(&mut vec![pop(rax()), mov(rsp(), rbp()), pop(rbp()), ret()]);
            assembly
        }
        Statement::Block(statements) => {
            let mut assembly: Assembly = Vec::new();
            for (index, statement) in statements.iter().enumerate() {
                assembly.append(&mut get_assembly_statement(statement, meta_info));
                if index != statements.len() - 1 {
                    assembly.push(pop(rax()));
                }
            }
            assembly
        }
        Statement::If(expr, if_statement, else_statement) => {
            let mut assembly: Assembly = Vec::new();
            assembly.push(comment("if"));
            assembly.append(&mut get_assembly_expr(expr, meta_info));
            assembly.append(&mut vec![pop(rax()), cmp(rax(), immediate(0))]);
            match **else_statement {
                Some(ref else_statement) => {
                    let else_label = meta_info.get_new_label();
                    let end_label = meta_info.get_new_label();
                    assembly.push(je(else_label.clone()));
                    assembly.append(&mut get_assembly_statement(if_statement, meta_info));
                    assembly.push(jmp(end_label.clone()));
                    assembly.push(comment("else"));
                    assembly.push(label(else_label));
                    assembly.append(&mut get_assembly_statement(else_statement, meta_info));
                    assembly.push(label(end_label));
                }
                None => {
                    let end_label = meta_info.get_new_label();
                    assembly.push(je(end_label.clone()));
                    assembly.append(&mut get_assembly_statement(if_statement, meta_info));
                    assembly.push(pop(rax()));
                    assembly.push(label(end_label));
                }
            }
            assembly.push(push(immediate(0)));
            assembly.push(comment("if end"));
            assembly
        }
        Statement::While(expr, statement) => {
            let mut assembly: Assembly = Vec::new();
            let start_label = meta_info.get_new_label();
            let end_label = meta_info.get_new_label();

            meta_info.push_label_for_break(end_label.clone());

            assembly.push(label(start_label.clone()));
            assembly.push(comment("while"));
            assembly.append(&mut get_assembly_expr(expr, meta_info));
            assembly.append(&mut vec![
                pop(rax()),
                cmp(rax(), immediate(0)),
                je(end_label.clone()),
            ]);
            assembly.append(&mut get_assembly_statement(statement, meta_info));
            assembly.push(comment("while content pop"));
            assembly.push(pop(rax()));
            assembly.push(jmp(start_label));
            assembly.push(label(end_label));
            assembly.push(push(immediate(0)));
            assembly.push(comment("while end"));

            meta_info.pop_label_for_break();

            assembly
        }
        Statement::For(init, cond, update, statement) => {
            let mut assembly: Assembly = Vec::new();
            if let Some(ref init) = **init {
                assembly.append(&mut get_assembly_statement(init, meta_info));
                assembly.push(pop(rax()));
            }
            let start_label = meta_info.get_new_label();
            let end_label = meta_info.get_new_label();

            meta_info.push_label_for_break(end_label.clone());

            assembly.push(label(start_label.clone()));
            if let Some(ref cond) = **cond {
                assembly.append(&mut get_assembly_expr(cond, meta_info));
                assembly.append(&mut vec![
                    pop(rax()),
                    cmp(rax(), immediate(0)),
                    je(end_label.clone()),
                ]);
            }
            assembly.append(&mut get_assembly_statement(statement, meta_info));
            assembly.push(pop(rax()));
            if let Some(ref update) = **update {
                assembly.append(&mut get_assembly_statement(update, meta_info));
                assembly.push(pop(rax()));
            }
            assembly.push(jmp(start_label));
            assembly.push(label(end_label));

            meta_info.pop_label_for_break();

            assembly
        }
        Statement::Break => {
            let mut assembly: Assembly = Vec::new();
            let label = meta_info.get_label_for_break();
            assembly.push(comment("break"));
            assembly.push(jmp(label.clone()));
            assembly
        }
    }
}

fn get_assembly_lval(lval: &String, meta_info: &mut MetaInfo) -> Assembly {
    let id = meta_info.get_variable_id_and_register_it(lval);
    vec![
        mov(rax(), rbp()),
        sub(rax(), immediate((id + 1) as i32 * 8)),
        push(rax()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_variable_id_and_register_it() {
        let mut meta_info = MetaInfo::new();
        let a_id = meta_info.get_variable_id_and_register_it(&"a".to_string());
        let b_id = meta_info.get_variable_id_and_register_it(&"b".to_string());
        assert_eq!(
            meta_info.get_variable_id_and_register_it(&"a".to_string()),
            a_id
        );
        assert_eq!(
            meta_info.get_variable_id_and_register_it(&"b".to_string()),
            b_id
        );
    }
}

fn get_assembly_expr(expr: &Expr, meta_info: &mut MetaInfo) -> Assembly {
    match expr {
        Expr::ArithExpr(arith_expr) => get_assembly_arith_expr(arith_expr, meta_info),
        Expr::Equal(left, right) => {
            get_compare_instruction(&|o: Operand| sete(o), left, right, meta_info)
        }
        Expr::NotEqual(left, right) => {
            get_compare_instruction(&|o: Operand| setne(o), left, right, meta_info)
        }
        Expr::Less(left, right) => {
            get_compare_instruction(&|o: Operand| setl(o), left, right, meta_info)
        }
        Expr::LessOrEqual(left, right) => {
            get_compare_instruction(&|o: Operand| setle(o), left, right, meta_info)
        }
    }
}

fn get_compare_instruction(
    gen_instruction: &dyn Fn(Operand) -> Instruction,
    left: &ArithExpr,
    right: &ArithExpr,
    meta_info: &mut MetaInfo,
) -> Assembly {
    let mut assembly: Assembly = Vec::new();
    assembly.append(&mut get_assembly_arith_expr(left, meta_info));
    assembly.append(&mut get_assembly_arith_expr(right, meta_info));
    assembly.append(&mut vec![
        pop(rdi()),
        pop(rax()),
        cmp(rax(), rdi()),
        gen_instruction(al()),
        movzb(rax(), al()),
        push(rax()),
    ]);
    assembly
}

fn get_assembly_arith_expr(expr: &ArithExpr, meta_info: &mut MetaInfo) -> Assembly {
    match expr {
        ArithExpr::Factor(factor) => get_assembly_factor(factor, meta_info),
        ArithExpr::Add(expr, factor) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_arith_expr(expr, meta_info));
            assembly.append(&mut get_assembly_factor(factor, meta_info));
            assembly.append(&mut vec![
                pop(rdi()),
                pop(rax()),
                add(rax(), rdi()),
                push(rax()),
            ]);
            assembly
        }
        ArithExpr::Sub(expr, factor) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_arith_expr(expr, meta_info));
            assembly.append(&mut get_assembly_factor(factor, meta_info));
            assembly.append(&mut vec![
                pop(rdi()),
                pop(rax()),
                sub(rax(), rdi()),
                push(rax()),
            ]);
            assembly
        }
    }
}

fn get_assembly_factor(factor: &Factor, meta_info: &mut MetaInfo) -> Assembly {
    match factor {
        Factor::Unary(unary) => get_assembly_unary(unary, meta_info),
        Factor::Mul(factor, unary) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_factor(factor, meta_info));
            assembly.append(&mut get_assembly_unary(unary, meta_info));
            assembly.append(&mut vec![pop(rdi()), pop(rax()), mul(rdi()), push(rax())]);
            assembly
        }
        Factor::Div(factor, unary) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_factor(factor, meta_info));
            assembly.append(&mut get_assembly_unary(unary, meta_info));
            assembly.append(&mut vec![
                pop(rdi()),
                pop(rax()),
                cqo(),
                idiv(rdi()),
                push(rax()),
            ]);
            assembly
        }
    }
}

fn get_assembly_unary(unary: &Unary, meta_info: &mut MetaInfo) -> Assembly {
    match unary {
        Unary::Atom(atom) => get_assembly_atom(atom, meta_info),
        Unary::Neg(atom) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_atom(atom, meta_info));
            assembly.append(&mut vec![pop(rax()), neg(rax()), push(rax())]);
            assembly
        }
    }
}

fn get_assembly_atom(atom: &Atom, meta_info: &mut MetaInfo) -> Assembly {
    match atom {
        Atom::Number(n) => vec![push(Operand::Immediate(*n))],
        Atom::Expr(expr) => get_assembly_expr(expr, meta_info),
        Atom::Variable(lval) => {
            let id = meta_info.get_variable_id_and_register_it(lval);
            vec![
                mov(rax(), rbp()),
                sub(rax(), immediate((id + 1) as i32 * 8)),
                push(m_rax()),
            ]
        }
        // 7 or more arguments are not supported
        Atom::FunctionCall(func_name, arguments) => {
            let mut assembly: Assembly = Vec::new();
            let argument_registers = vec![rdi(), rsi(), rdx(), rcx(), r8(), r9()];
            for (argument, register) in arguments.iter().zip(argument_registers.iter()) {
                assembly.append(&mut get_assembly_expr(argument, meta_info));
                assembly.push(pop(register.clone()));
            }
            assembly.push(call(func_name.clone()));
            assembly.push(push(rax()));
            assembly
        }
    }
}
