//! Functions for type-checking and type-inference.

use crate::environment::{type_var, Context, Definitions, Environment};
use crate::equivalence::judgmentally_equal;
use crate::evaluation::evaluate;
use crate::expression::Expression;
use crate::read_back::read_back_typed;
use crate::value::{Closure, Neutral, Type, Value};
use crate::TypeError;
use std::ops::Deref;

/// Checks the [`Type`] of an [`Expression`].
pub fn check_type(
    defs: &Definitions,
    ctx: &Context,
    expr: &Expression,
    type_: &Type,
) -> crate::Result<()> {
    use Expression::*;
    match expr {
        Lambda {
            param,
            param_type: None::<_>,
            ret_val,
        } => {
            let Value::PiType{ param_type, tclosure } = type_.deref() else {
                return Err(TypeError{
                    msg: format!(
                        "{} is not of type {}, because all lambda terms are of pi types.",
                        expr,
                        type_
                    )
                });
            };
            let ret_type = tclosure.call(
                defs,
                &Value::Neutral {
                    neu: Neutral::Variable(param.clone()),
                },
            );
            check_type(
                defs,
                &ctx.extend(param, param_type),
                ret_val,
                &Type::create_type_from_value(ret_type),
            )
        }
        _ => {
            let syn_type = synth_type(defs, ctx, expr)?;
            judgmentally_equal(defs, ctx, &syn_type, type_, &Type::UNIVERSE)
        }
    }
}

/// Synthesizes a [`Type`] for an [`Expression`].
pub fn synth_type(defs: &Definitions, ctx: &Context, expr: &Expression) -> crate::Result<Type> {
    use Expression::*;
    match expr {
        Variable(id) => type_var(defs, ctx, id).cloned(),
        PiType {
            tparam,
            tparam_type,
            ret_type,
        } => {
            check_type(defs, ctx, tparam_type, &Type::UNIVERSE)?;
            let tparam_type = evaluate(defs, &Environment::from_context(ctx), tparam_type);
            check_type(
                defs,
                &ctx.extend(tparam, &Type::create_type_from_value(tparam_type)),
                ret_type,
                &Type::UNIVERSE,
            )?;
            Ok(Type::UNIVERSE)
        }
        Lambda {
            param,
            param_type,
            ret_val,
        } => {
            let Some(param_type) = param_type else {
                return Err(TypeError{
                    msg: format!("Cannot infer a type for lambda expression `{}` without parameter type given.", expr)
                })
            };
            check_type(defs, ctx, param_type, &Type::UNIVERSE)?;
            let param_type = Type::create_type_from_value(evaluate(
                defs,
                &Environment::from_context(ctx),
                param_type,
            ));
            let ret_type = synth_type(defs, &ctx.extend(param, &param_type), ret_val)?;
            let ret_type = read_back_typed(defs, ctx, &ret_type, &Type::UNIVERSE);
            Ok(Type::create_type_from_value(Value::PiType {
                param_type: Box::new(param_type),
                tclosure: Closure::new_in_ctx(ctx, param.clone(), ret_type),
            }))
        }
        Application { func, arg } => {
            let func_type = synth_type(defs, ctx, func)?;
            let Value::PiType{ param_type, tclosure } = func_type.deref() else {
                return Err(TypeError{
                    msg: format!(
                        "Cannot call `{}` as a function, because it is of non-function type `{}`.",
                        func, func_type
                    )
                })
            };
            check_type(defs, ctx, arg, param_type)?;
            let arg = evaluate(defs, &Environment::from_context(ctx), arg);
            Ok(Type::create_type_from_value(tclosure.call(defs, &arg)))
        }
        Universe => Ok(Type::UNIVERSE),
        Annotation { expr, type_ } => {
            check_type(defs, ctx, type_, &Type::UNIVERSE)?;
            let type_ = Type::create_type_from_value(evaluate(
                defs,
                &Environment::from_context(ctx),
                type_,
            ));
            check_type(defs, ctx, expr, &type_)?;
            Ok(type_)
        }
    }
}
