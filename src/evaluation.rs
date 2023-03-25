//! Functions for evaluating [`Expression`]s to [`Value`]s.

use crate::environment::{evaluate_var, Definitions, Environment};
use crate::expression::Expression;
use crate::value::{Closure, Neutral, Type, Value};

/// Evaluates an expression to a value.
pub fn evaluate(defs: &Definitions, env: &Environment, expr: &Expression) -> Value {
    use Expression::*;
    match expr {
        Variable(id) => evaluate_var(defs, env, id),
        PiType {
            tparam,
            tparam_type,
            ret_type,
        } => Value::PiType {
            param_type: Box::new(Type::create_type_from_value(evaluate(defs, env, tparam_type))),
            tclosure: Closure::new_in_env(env, tparam.clone(), *ret_type.clone()),
        },
        Lambda { param, ret_val, .. } => Value::Lambda {
            closure: Closure::new_in_env(env, param.clone(), *ret_val.clone()),
        },
        Application { func, arg } => {
            do_apply(defs, &evaluate(defs, env, func), &evaluate(defs, env, arg))
        }
        Universe => Value::Universe,
        Annotation { expr, .. } => evaluate(defs, env, expr),
    }
}

pub(crate) fn do_apply(defs: &Definitions, func: &Value, arg: &Value) -> Value {
    match func {
        Value::Lambda { closure } => closure.call(defs, arg),
        Value::Neutral { neu } => Value::Neutral {
            neu: Neutral::Application {
                func: Box::new(neu.clone()),
                arg: Box::new(arg.clone()),
            },
        },
        _ => panic!("Cannot call `{}` because it is not a function.", func),
    }
}
