//! Functions for checking equivalence of terms.

use crate::environment::{Context, Definitions};
use crate::expression::Expression;
use crate::lists::Names;
use crate::read_back::read_back_typed;
use crate::value::{Type, Value};
use crate::TypeError;

fn alpha_equiv_helper(
    lhs_names: &Names,
    lhs: &Expression,
    rhs_names: &Names,
    rhs: &Expression,
) -> bool {
    use Expression::*;
    match (lhs, rhs) {
        (Variable(lhs_id), Variable(rhs_id)) => {
            lhs_names.index_of(lhs_id) == rhs_names.index_of(rhs_id)
        }
        (
            PiType {
                tparam: lhs_tparam,
                tparam_type: lhs_tparam_type,
                ret_type: lhs_ret_type,
            },
            PiType {
                tparam: rhs_tparam,
                tparam_type: rhs_tparam_type,
                ret_type: rhs_ret_type,
            },
        ) => {
            alpha_equiv_helper(lhs_names, lhs_tparam_type, rhs_names, rhs_tparam_type)
                && alpha_equiv_helper(
                    &lhs_names.extend_names(lhs_tparam),
                    lhs_ret_type,
                    &rhs_names.extend_names(rhs_tparam),
                    rhs_ret_type,
                )
        }
        (
            Lambda {
                param: lhs_param,
                param_type: lhs_param_type,
                ret_val: lhs_ret_val,
            },
            Lambda {
                param: rhs_param,
                param_type: rhs_param_type,
                ret_val: rhs_ret_val,
            },
        ) => {
            if let (Some(lhs_param_type), Some(rhs_param_type)) = (lhs_param_type, rhs_param_type) {
                if !alpha_equiv_helper(lhs_names, lhs_param_type, rhs_names, rhs_param_type) {
                    return false;
                }
            } else if lhs_param_type.is_some() || rhs_param_type.is_some() {
                return false;
            }
            alpha_equiv_helper(
                &lhs_names.extend_names(lhs_param),
                lhs_ret_val,
                &rhs_names.extend_names(rhs_param),
                rhs_ret_val,
            )
        }
        (
            Application {
                func: lhs_func,
                arg: lhs_arg,
            },
            Application {
                func: rhs_func,
                arg: rhs_arg,
            },
        ) => {
            alpha_equiv_helper(lhs_names, lhs_func, rhs_names, rhs_func)
                && alpha_equiv_helper(lhs_names, lhs_arg, rhs_names, rhs_arg)
        }
        (Universe, Universe) => true,
        (
            Annotation {
                expr: lhs_expr,
                type_: lhs_type_,
            },
            Annotation {
                expr: rhs_expr,
                type_: rhs_type_,
            },
        ) => {
            alpha_equiv_helper(lhs_names, lhs_expr, rhs_names, rhs_expr)
                && alpha_equiv_helper(lhs_names, lhs_type_, rhs_names, rhs_type_)
        }
        _ => false,
    }
}

/// Checks alpha-equivalence of [`Expression`]s.
pub fn alpha_equivalent(lhs: &Expression, rhs: &Expression) -> crate::Result<()> {
    if alpha_equiv_helper(&Names::Empty, lhs, &Names::Empty, rhs) {
        Ok(())
    } else {
        Err(TypeError {
            msg: format!(
                "Expressions `{}` and `{}` are not alpha-equivalent.",
                lhs, rhs
            ),
        })
    }
}

/// Checks judgmental equality of [`Value`]s of a shared [`Type`].
pub fn judgmentally_equal(
    defs: &Definitions,
    ctx: &Context,
    lhs: &Value,
    rhs: &Value,
    type_: &Type,
) -> crate::Result<()> {
    alpha_equivalent(
        &read_back_typed(defs, ctx, lhs, type_),
        &read_back_typed(defs, ctx, rhs, type_),
    )
}
