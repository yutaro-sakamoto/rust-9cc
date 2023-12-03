use crate::assembly::*;
use crate::ast::*;
use crate::compile_error::CompilerError;
use crate::infer_type::*;
use std::collections::HashMap;

pub struct VarInfo {
    pub id: u32,
    // TODO: remove the following annotation
    #[allow(dead_code)]
    pub data_type: DataType,
}

pub struct MetaInfo {
    scopes: Vec<HashMap<String, VarInfo>>,
    label_count: u64,
    label_stack_for_break: Vec<String>,
}

impl MetaInfo {
    pub fn new() -> MetaInfo {
        MetaInfo {
            scopes: vec![HashMap::new()],
            label_count: 0,
            label_stack_for_break: Vec::new(),
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn register_variables(&mut self, variables: &[(DataType, String)]) {
        let mut id = self.scopes.last().unwrap().len() as u32;
        let current_scope = self.scopes.last_mut().unwrap();
        for (data_type, var_name) in variables.iter() {
            current_scope.insert(
                var_name.clone(),
                VarInfo {
                    id,
                    data_type: data_type.clone(),
                },
            );
            id += 1;
        }
    }

    pub fn register_variable(&mut self, variable: &str, data_type: &DataType) {
        let current_scope = self.scopes.last_mut().unwrap();
        let var_info = VarInfo {
            id: current_scope.len() as u32,
            data_type: data_type.clone(),
        };
        current_scope.insert(variable.to_string(), var_info);
    }

    pub fn get_variable(&self, lval: &String) -> Option<&VarInfo> {
        self.scopes.last().unwrap().get(lval)
    }

    pub fn get_number_of_variables(&self) -> u32 {
        self.scopes.last().unwrap().len() as u32
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

const ARGUMENT_REGISTERS: &[Operand] = &[
    Operand::Register(Register::RDI),
    Operand::Register(Register::RSI),
    Operand::Register(Register::RDX),
    Operand::Register(Register::RCX),
    Operand::Register(Register::R8),
    Operand::Register(Register::R9),
];

impl Default for MetaInfo {
    fn default() -> Self {
        Self::new()
    }
}

pub fn print_assembly(program: &Program) -> Result<(), CompilerError> {
    let mut meta_info = MetaInfo::default();
    print_assembly_internal(program, &mut meta_info)
}

fn print_assembly_internal(
    program: &Program,
    meta_info: &mut MetaInfo,
) -> Result<(), CompilerError> {
    let main_label = label("main".to_string());
    let header_code = vec![push(rbp()), mov(rbp(), rsp())];
    let footer_code = vec![mov(rsp(), rbp()), pop(rbp()), ret()];
    let mut main_code: Assembly = Vec::new();
    let mut func_def_code: Assembly = Vec::new();
    for program_unit in program.program_units.iter() {
        match program_unit {
            ProgramUnit::FuncDef(_, func_name, parameters, statement) => {
                // Enter a new scope and register parameters of functions
                meta_info.push_scope();
                meta_info.register_variables(parameters);

                // Function label
                func_def_code.push(label(func_name.clone()));

                // Prelude code
                func_def_code.append(&mut header_code.clone());
                func_def_code.push(sub(rsp(), immediate(8 * parameters.len() as i32)));

                // Copy arguments to local variables
                for (index, (register, _)) in
                    ARGUMENT_REGISTERS.iter().zip(parameters.iter()).enumerate()
                {
                    func_def_code.append(&mut vec![
                        comment("copy arguments to local variables"),
                        mov(rax(), rbp()),
                        sub(rax(), immediate((index + 1) as i32 * 8)),
                        mov(m_rax(), register.clone()),
                        comment("copy arguments to local variables end"),
                    ]);
                }

                // Function body
                func_def_code.append(&mut get_assembly_statement(statement, meta_info)?);

                // Postlude code
                func_def_code.append(&mut footer_code.clone());

                // Leave the scope
                meta_info.pop_scope();
            }
            ProgramUnit::Statement(statement) => {
                main_code.append(&mut get_assembly_statement(statement, meta_info)?);
                main_code.push(pop(rax()));
            }
        }
    }
    let number_of_variables = meta_info.get_number_of_variables();
    let sub_rsp_code = sub(rsp(), immediate(8 * number_of_variables as i32));

    println!(".intel_syntax noprefix");
    println!(".global main\n");
    print_single_instruction(&main_label);
    print_assembly_code(&header_code);
    print_single_instruction(&sub_rsp_code);
    print_assembly_code(&main_code);
    print_assembly_code(&footer_code);
    print_assembly_code(&func_def_code);
    Ok(())
}

pub fn get_assembly_statement(
    statement: &Statement,
    meta_info: &mut MetaInfo,
) -> Result<Assembly, CompilerError> {
    match statement {
        Statement::Expr(expr) => get_assembly_expr(expr, meta_info),
        Statement::Assign(left, expr) => {
            let mut assembly: Assembly = Vec::new();
            assembly.push(comment("assign"));
            assembly.append(&mut get_assembly_lval(left, meta_info)?);
            assembly.append(&mut get_assembly_expr(expr, meta_info)?);
            assembly.append(&mut vec![
                pop(rdi()),
                pop(rax()),
                mov(m_rax(), rdi()),
                push(rdi()),
            ]);
            assembly.push(comment("assign end"));
            Ok(assembly)
        }
        Statement::AssignPointer(depth, left, expr) => {
            let mut assembly: Assembly = Vec::new();
            assembly.push(comment("assign pointer"));
            assembly.append(&mut get_assembly_lval(left, meta_info)?);
            assembly.append(&mut get_assembly_expr(expr, meta_info)?);
            assembly.append(&mut vec![pop(rdi()), pop(rax())]);
            for _ in 0..*depth {
                assembly.push(mov(rax(), m_rax()));
            }
            assembly.append(&mut vec![mov(m_rax(), rdi()), push(rdi())]);
            assembly.push(comment("assign pointer end"));
            Ok(assembly)
        }
        Statement::Return(expr) => {
            let mut assembly: Assembly = get_assembly_expr(expr, meta_info)?;
            assembly.append(&mut vec![pop(rax()), mov(rsp(), rbp()), pop(rbp()), ret()]);
            Ok(assembly)
        }
        Statement::Block(statements) => {
            let mut assembly: Assembly = Vec::new();
            for (index, statement) in statements.iter().enumerate() {
                assembly.append(&mut get_assembly_statement(statement, meta_info)?);
                if index != statements.len() - 1 {
                    assembly.push(pop(rax()));
                }
            }
            Ok(assembly)
        }
        Statement::If(expr, if_statement, else_statement) => {
            let mut assembly: Assembly = Vec::new();
            assembly.push(comment("if"));
            assembly.append(&mut get_assembly_expr(expr, meta_info)?);
            assembly.append(&mut vec![pop(rax()), cmp(rax(), immediate(0))]);
            match **else_statement {
                Some(ref else_statement) => {
                    let else_label = meta_info.get_new_label();
                    let end_label = meta_info.get_new_label();
                    assembly.push(je(else_label.clone()));
                    assembly.append(&mut get_assembly_statement(if_statement, meta_info)?);
                    assembly.push(jmp(end_label.clone()));
                    assembly.push(comment("else"));
                    assembly.push(label(else_label));
                    assembly.append(&mut get_assembly_statement(else_statement, meta_info)?);
                    assembly.push(label(end_label));
                }
                None => {
                    let end_label = meta_info.get_new_label();
                    assembly.push(je(end_label.clone()));
                    assembly.append(&mut get_assembly_statement(if_statement, meta_info)?);
                    assembly.push(pop(rax()));
                    assembly.push(label(end_label));
                }
            }
            assembly.push(push(immediate(0)));
            assembly.push(comment("if end"));
            Ok(assembly)
        }
        Statement::While(expr, statement) => {
            let mut assembly: Assembly = Vec::new();
            let start_label = meta_info.get_new_label();
            let end_label = meta_info.get_new_label();

            meta_info.push_label_for_break(end_label.clone());

            assembly.push(label(start_label.clone()));
            assembly.push(comment("while"));
            assembly.append(&mut get_assembly_expr(expr, meta_info)?);
            assembly.append(&mut vec![
                pop(rax()),
                cmp(rax(), immediate(0)),
                je(end_label.clone()),
            ]);
            assembly.append(&mut get_assembly_statement(statement, meta_info)?);
            assembly.push(comment("while content pop"));
            assembly.push(pop(rax()));
            assembly.push(jmp(start_label));
            assembly.push(label(end_label));
            assembly.push(push(immediate(0)));
            assembly.push(comment("while end"));

            meta_info.pop_label_for_break();

            Ok(assembly)
        }
        Statement::For(init, cond, update, statement) => {
            let mut assembly: Assembly = Vec::new();
            if let Some(ref init) = **init {
                assembly.append(&mut get_assembly_statement(init, meta_info)?);
                assembly.push(pop(rax()));
            }
            let start_label = meta_info.get_new_label();
            let end_label = meta_info.get_new_label();

            meta_info.push_label_for_break(end_label.clone());

            assembly.push(label(start_label.clone()));
            if let Some(ref cond) = **cond {
                assembly.append(&mut get_assembly_expr(cond, meta_info)?);
                assembly.append(&mut vec![
                    pop(rax()),
                    cmp(rax(), immediate(0)),
                    je(end_label.clone()),
                ]);
            }
            assembly.append(&mut get_assembly_statement(statement, meta_info)?);
            assembly.push(pop(rax()));
            if let Some(ref update) = **update {
                assembly.append(&mut get_assembly_statement(update, meta_info)?);
                assembly.push(pop(rax()));
            }
            assembly.push(jmp(start_label));
            assembly.push(label(end_label));

            meta_info.pop_label_for_break();

            Ok(assembly)
        }
        Statement::Break => {
            let mut assembly: Assembly = Vec::new();
            let label = meta_info.get_label_for_break();
            assembly.push(comment("break"));
            assembly.push(jmp(label.clone()));
            Ok(assembly)
        }

        Statement::VarDef(data_type, var_name) => {
            meta_info.register_variable(var_name, data_type);
            Ok(vec![push(immediate(0))])
        }
    }
}

fn get_assembly_lval(lval: &String, meta_info: &mut MetaInfo) -> Result<Assembly, CompilerError> {
    let id = match meta_info.get_variable(lval) {
        Some(var_info) => var_info.id,
        None => return Err(CompilerError::UndefinedVariable(lval.clone())),
    };
    Ok(vec![
        mov(rax(), rbp()),
        sub(rax(), immediate((id + 1) as i32 * 8)),
        push(rax()),
    ])
}

fn get_assembly_expr(expr: &Expr, meta_info: &mut MetaInfo) -> Result<Assembly, CompilerError> {
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
) -> Result<Assembly, CompilerError> {
    let mut assembly: Assembly = Vec::new();
    assembly.append(&mut get_assembly_arith_expr(left, meta_info)?);
    assembly.append(&mut get_assembly_arith_expr(right, meta_info)?);
    assembly.append(&mut vec![
        pop(rdi()),
        pop(rax()),
        cmp(rax(), rdi()),
        gen_instruction(al()),
        movzb(rax(), al()),
        push(rax()),
    ]);
    Ok(assembly)
}

fn get_assembly_arith_expr(
    expr: &ArithExpr,
    meta_info: &mut MetaInfo,
) -> Result<Assembly, CompilerError> {
    match expr {
        ArithExpr::Factor(factor) => get_assembly_factor(factor, meta_info),
        ArithExpr::Add(expr, factor) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_arith_expr(expr, meta_info)?);
            assembly.append(&mut get_assembly_factor(factor, meta_info)?);
            assembly.append(&mut vec![pop(rdi()), pop(rax())]);
            let left_type = match infer_type_arith_expr(expr, meta_info) {
                Ok(data_type) => data_type,
                Err(e) => return Err(CompilerError::TypeMismatch(e)),
            };
            match left_type {
                DataType::Primitive(PrimitiveType::Int) => assembly.push(add(rax(), rdi())),
                DataType::Pointer(_, base_type) => {
                    let size = match *base_type {
                        DataType::Primitive(PrimitiveType::Int) => 8,
                        _ => 8,
                    };
                    assembly.append(&mut vec![imul(rdi(), immediate(size)), add(rax(), rdi())]);
                }
                _ => {
                    return Err(CompilerError::TypeMismatch(
                        "left side of + is not int or pointer".to_string(),
                    ))
                }
            }
            assembly.append(&mut vec![push(rax())]);
            Ok(assembly)
        }
        ArithExpr::Sub(expr, factor) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_arith_expr(expr, meta_info)?);
            assembly.append(&mut get_assembly_factor(factor, meta_info)?);
            assembly.append(&mut vec![
                pop(rdi()),
                pop(rax()),
                sub(rax(), rdi()),
                push(rax()),
            ]);
            Ok(assembly)
        }
    }
}

fn get_assembly_factor(
    factor: &Factor,
    meta_info: &mut MetaInfo,
) -> Result<Assembly, CompilerError> {
    match factor {
        Factor::Unary(unary) => get_assembly_unary(unary, meta_info),
        Factor::Mul(factor, unary) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_factor(factor, meta_info)?);
            assembly.append(&mut get_assembly_unary(unary, meta_info)?);
            assembly.append(&mut vec![pop(rdi()), pop(rax()), mul(rdi()), push(rax())]);
            Ok(assembly)
        }
        Factor::Div(factor, unary) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_factor(factor, meta_info)?);
            assembly.append(&mut get_assembly_unary(unary, meta_info)?);
            assembly.append(&mut vec![
                pop(rdi()),
                pop(rax()),
                cqo(),
                idiv(rdi()),
                push(rax()),
            ]);
            Ok(assembly)
        }
    }
}

fn get_assembly_unary(unary: &Unary, meta_info: &mut MetaInfo) -> Result<Assembly, CompilerError> {
    match unary {
        Unary::Atom(atom) => get_assembly_atom(atom, meta_info),
        Unary::Neg(atom) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_atom(atom, meta_info)?);
            assembly.append(&mut vec![pop(rax()), neg(rax()), push(rax())]);
            Ok(assembly)
        }
        Unary::PointerDeref(atom) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_atom(atom, meta_info)?);
            assembly.append(&mut vec![pop(rax()), mov(rax(), m_rax()), push(rax())]);
            Ok(assembly)
        }
    }
}

fn get_assembly_atom(atom: &Atom, meta_info: &mut MetaInfo) -> Result<Assembly, CompilerError> {
    match atom {
        Atom::Number(n) => Ok(vec![push(Operand::Immediate(*n))]),
        Atom::Expr(expr) => get_assembly_expr(expr, meta_info),
        Atom::Variable(lval) => {
            //let id = meta_info.get_variable_id_and_register_it(lval);
            let id = match meta_info.get_variable(lval) {
                Some(var_info) => var_info.id,
                None => return Err(CompilerError::UndefinedVariable(lval.clone())),
            };
            Ok(vec![
                mov(rax(), rbp()),
                sub(rax(), immediate((id + 1) as i32 * 8)),
                push(m_rax()),
            ])
        }
        // 7 or more arguments are not supported
        Atom::FunctionCall(func_name, arguments) => {
            let mut assembly: Assembly = Vec::new();
            for argument in arguments.iter() {
                assembly.append(&mut get_assembly_expr(argument, meta_info)?);
            }
            for (_, register) in arguments.iter().zip(ARGUMENT_REGISTERS.iter()).rev() {
                assembly.push(pop(register.clone()));
            }

            assembly.push(call(func_name.clone()));
            assembly.push(push(rax()));
            Ok(assembly)
        }
        Atom::AddressOf(lval) => {
            let id = match meta_info.get_variable(lval) {
                Some(var_info) => var_info.id,
                None => return Err(CompilerError::UndefinedVariable(lval.clone())),
            };
            Ok(vec![
                comment("address of"),
                mov(rax(), rbp()),
                sub(rax(), immediate((id + 1) as i32 * 8)),
                push(rax()),
                comment("address of end"),
            ])
        }
    }
}
