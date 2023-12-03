use crate::ast::*;
use crate::gen_code::*;

pub fn infer_type(statement: &Statement, meta_info: &MetaInfo) -> Result<DataType, String> {
    match statement {
        Statement::Expr(expr) => infer_type_expr(expr, meta_info),
        Statement::Assign(_, right) => infer_type_expr(right, meta_info),
        Statement::AssignPointer(_, _, expr) => infer_type_expr(expr, meta_info),
        Statement::Return(expr) => infer_type_expr(expr, meta_info),
        Statement::If(_, if_branch, else_branch) => {
            let if_branch_type = infer_type(if_branch, meta_info)?;
            match **else_branch {
                Some(ref else_branch) => {
                    let else_branch_type = infer_type(else_branch, meta_info)?;
                    if if_branch_type == else_branch_type {
                        Ok(if_branch_type)
                    } else {
                        Err(format!(
                            "Type mismatch: if branch is {}, else branch is {}",
                            if_branch_type, else_branch_type
                        ))
                    }
                }
                None => Ok(void()),
            }
        }
        Statement::Block(statements) => match statements.last() {
            Some(last_statement) => infer_type(last_statement, meta_info),
            None => Ok(void()),
        },
        Statement::While(_, _) => Ok(void()),
        Statement::For(_, _, _, _) => Ok(void()),
        Statement::Break => Ok(void()),
        Statement::VarDef(_, _) => Ok(void()),
    }
}

pub fn infer_type_expr(expr: &Expr, meta_info: &MetaInfo) -> Result<DataType, String> {
    match expr {
        Expr::ArithExpr(arith_expr) => infer_type_arith_expr(arith_expr, meta_info),
        _ => Ok(int()),
    }
}

fn infer_2types(left: DataType, right: DataType) -> Result<DataType, String> {
    if left == right {
        Ok(left)
    } else {
        Err(format!(
            "Type mismatch: left is {}, right is {}",
            left, right
        ))
    }
}

pub fn infer_type_arith_expr(
    arith_expr: &ArithExpr,
    meta_info: &MetaInfo,
) -> Result<DataType, String> {
    match arith_expr {
        ArithExpr::Factor(factor) => infer_type_factor(factor, meta_info),
        ArithExpr::Add(left, right) => infer_2types(
            infer_type_arith_expr(left, meta_info)?,
            infer_type_factor(right, meta_info)?,
        ),
        ArithExpr::Sub(left, right) => infer_2types(
            infer_type_arith_expr(left, meta_info)?,
            infer_type_factor(right, meta_info)?,
        ),
    }
}

pub fn infer_type_factor(factor: &Factor, meta_info: &MetaInfo) -> Result<DataType, String> {
    match factor {
        Factor::Unary(unary) => infer_type_unary(unary, meta_info),
        Factor::Mul(left, right) => infer_2types(
            infer_type_factor(left, meta_info)?,
            infer_type_unary(right, meta_info)?,
        ),
        Factor::Div(left, right) => infer_2types(
            infer_type_factor(left, meta_info)?,
            infer_type_unary(right, meta_info)?,
        ),
    }
}

pub fn infer_type_unary(unary: &Unary, meta_info: &MetaInfo) -> Result<DataType, String> {
    match unary {
        Unary::Atom(atom) => infer_type_atom(atom, meta_info),
        Unary::Neg(atom) => match infer_type_atom(atom, meta_info)? {
            DataType::Primitive(PrimitiveType::Int) => Ok(int()),
            _ => Err(format!("Type mismatch: {:?} (Negeation)", atom)),
        },
        Unary::PointerDeref(atom) => match infer_type_atom(atom, meta_info) {
            Ok(DataType::Pointer(_, t)) => Ok(*t),
            _ => Err(format!("Type mismatch: {:?} (PointerDeref)", atom)),
        },
    }
}

pub fn infer_type_atom(atom: &Atom, meta_info: &MetaInfo) -> Result<DataType, String> {
    match atom {
        Atom::Number(_) => Ok(int()),
        Atom::Expr(expr) => infer_type_expr(expr, meta_info),
        Atom::Variable(name) => match meta_info.get_variable(name) {
            Some(var_info) => Ok(var_info.data_type.clone()),
            None => Err(format!("Undefined variable: {}", name)),
        },
        Atom::AddressOf(name) => match meta_info.get_variable(name) {
            Some(var_info) => Ok(pointer(1, var_info.data_type.clone())),
            None => Err(format!("Undefined variable: {}", name)),
        },
        Atom::FunctionCall(_, _) => Ok(int()), //TODO
    }
}
