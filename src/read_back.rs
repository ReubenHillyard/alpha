//! Functions for reading back [`Value`]s as [`Expression`]s.

use crate::environment::{Context, Definitions};
use crate::evaluation::do_apply;
use crate::expression::Expression;
use crate::identifier::fresh_identifier;
use crate::typing::synth_type;
use crate::value::{Neutral, Type, Value};
use std::ops::Deref;

/// Reads back a [`Type`]d [`Value`] to an [`Expression`] in beta-normal, eta-long form.
pub fn read_back_typed(defs: &Definitions, ctx: &Context, val: &Value, type_: &Type) -> Expression {
    match type_.deref() {
        Value::PiType {
            param_type,
            tclosure,
        } => {
            let fresh_id = fresh_identifier(defs, ctx, tclosure);
            let fresh_var = Value::Neutral {
                neu: Neutral::Variable(fresh_id.clone()),
            };
            let ret_val = read_back_typed(
                defs,
                &ctx.extend(&fresh_id, param_type),
                &do_apply(defs, val, &fresh_var),
                &Type::create_type_from_value(tclosure.call(defs, &fresh_var)),
            );
            Expression::Lambda {
                param: fresh_id,
                param_type: None,
                ret_val: Box::new(ret_val),
            }
        }
        Value::Universe => match val {
            Value::PiType {
                param_type,
                tclosure,
            } => {
                let fresh_id = fresh_identifier(defs, ctx, tclosure);
                let fresh_var = Value::Neutral {
                    neu: Neutral::Variable(fresh_id.clone()),
                };
                let ret_type = read_back_typed(
                    defs,
                    &ctx.extend(&fresh_id, param_type),
                    &tclosure.call(defs, &fresh_var),
                    &Type::UNIVERSE,
                );
                let param_type = read_back_typed(defs, ctx, param_type, &Type::UNIVERSE);
                Expression::PiType {
                    tparam: fresh_id,
                    tparam_type: Box::new(param_type),
                    ret_type: Box::new(ret_type),
                }
            }
            Value::Universe => Expression::Universe,
            Value::Neutral { neu } => read_back_neutral(defs, ctx, neu),
            _ => panic!(
                "Cannot read back `{}` as a type because it is not a type.",
                val
            ),
        },
        Value::Neutral { .. } => match val {
            Value::Neutral { neu } => read_back_neutral(defs, ctx, neu),
            _ => panic!(
                "Cannot read back `{}` as a `{}` because it is not of that type.",
                val, type_
            ),
        },
        _ => panic!("Cannot read back as `{}` because it is not a type.", type_),
    }
}

/// Reads back a [`Neutral`] value to an [`Expression`] in beta-normal, eta-long form.
pub fn read_back_neutral(defs: &Definitions, ctx: &Context, neu: &Neutral) -> Expression {
    match neu {
        Neutral::Variable(id) => Expression::Variable(id.clone()),
        Neutral::Application { func, arg } => {
            let func = read_back_neutral(defs, ctx, func);
            let arg_type = match synth_type(defs, ctx, &func).map(Into::into) {
                Ok(Value::PiType { param_type, .. }) => param_type,
                _ => panic!("Cannot read back `{}` because it is ill-typed.", neu),
            };
            Expression::Application {
                func: Box::new(func),
                arg: Box::new(read_back_typed(defs, ctx, arg, &arg_type)),
            }
        }
    }
}
