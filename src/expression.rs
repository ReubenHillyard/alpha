//! A type representing a computation.

use crate::Identifier;
use std::fmt;

/// Computes to a [`Value`](crate::value::Value).
#[derive(Clone)]
pub enum Expression {
    Variable(Identifier),
    PiType {
        tparam: Identifier,
        tparam_type: Box<Expression>,
        ret_type: Box<Expression>,
    },
    Lambda {
        param: Identifier,
        param_type: Option<Box<Expression>>,
        ret_val: Box<Expression>,
    },
    Application {
        func: Box<Expression>,
        arg: Box<Expression>,
    },
    Universe,
    Annotation {
        expr: Box<Expression>,
        type_: Box<Expression>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Expression::*;
        match self {
            Variable(id) => id.fmt(f),
            PiType {
                tparam,
                tparam_type,
                ret_type,
            } => write!(f, "\u{220F}({} : {}){}", tparam, tparam_type, ret_type),
            Lambda {
                param,
                param_type,
                ret_val,
            } => match param_type {
                Some(param_type) => write!(f, "\u{03BB}({} : {}){}", param, param_type, ret_val),
                None => write!(f, "\u{03BB}{}.{}", param, ret_val),
            },
            Application { func, arg } => write!(f, "({})({})", func, arg),
            Universe => write!(f, "U"),
            Annotation { expr, type_ } => write!(f, "({} : {})", expr, type_),
        }
    }
}

/*macro_rules! parse_expr {
    (a($name:literal)) => {
        Expr::Variable(Identifier{ name: $name })
    };
    (P (a($name:literal) : $($t:tt)*) $($r:tt)*) => {
        Expr::PiType{
            tparam: Identifier{ name: $name },
            tparam_type: Box::new(
                parse_expr!($($t)*)
            ),
            ret_type: Box::new(
                parse_expr!($($r)*)
            ),
        }
    };
    (L (a($name:literal) : $($t:tt)*) $($r:tt)*) => {
        Expr::Lambda{
            param: Identifier{ name: $name },
            param_type: Some(Box::new(
                parse_expr!($($t)*)
            )),
            ret_val: Box::new(
                parse_expr!($($r)*)
            ),
        }
    };
    (L (a($name:literal)) $($r:tt)*) => {
        Expr::Lambda{
            param: Identifier{ name: $name },
            param_type: None,
            ret_val: Box::new(
                parse_expr!($($r)*)
            ),
        }
    };
    (($($f:tt)*)($($a:tt)*)) => {
        Expr::Application{
            func: Box::new(
                parse_expr!($($f)*)
            ),
            arg: Box::new(
                parse_expr!($($a)*)
            ),
        }
    };
    (U) => {
        Expr::Universe
    };
    (($($e:tt)*) : ($($t:tt)*)) => {
        Expr::Annotation{
            expr: Box::new(
                parse_expr!($($e)*)
            ),
            type_: Box::new(
                parse_expr!($($t)*)
            ),
        }
    };
}*/
