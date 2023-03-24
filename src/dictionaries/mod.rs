use crate::dictionaries::dictionary::Dictionary;
use crate::environment::Context;
use crate::evaluation::evaluate;
use crate::expression::Expression;
use crate::value::*;
use crate::Identifier;

mod dictionary;

/// A record of global definitions.
pub struct Definitions(Dictionary<TypedValue>);

impl Definitions {
    /// An iterator over defined names.
    pub fn names(&self) -> impl Iterator<Item = &Identifier> + Clone {
        self.0.names()
    }

    /// Determines the [`Type`] of a globally-defined item.
    pub fn lookup_type(&self, id: &Identifier) -> crate::Result<&Type> {
        self.0.get(id).map(|tv| &tv.type_)
    }

    /// Determines the [`Value`] of a globally-defined item.
    pub fn lookup_value(&self, id: &Identifier) -> crate::Result<&Value> {
        self.0.get(id).map(|tv| &tv.val)
    }
}

#[derive(Clone)]
struct FlatEnvironment(Dictionary<Value>);

impl FlatEnvironment {
    fn names(&self) -> impl Iterator<Item = &Identifier> + Clone {
        self.0.names()
    }
    fn lookup_value(&self, id: &Identifier) -> crate::Result<Value> {
        self.0.get(id).cloned()
    }
}

enum EnvironmentInner<'a> {
    From(&'a Context<'a>),
    Extend {
        parent: &'a FlatEnvironment,
        var: &'a Identifier,
        val: &'a Value,
    },
}

/// An assignment of [`Value`]s to variables.
pub struct Environment<'a>(EnvironmentInner<'a>);

impl Environment<'_> {
    /// The empty environment.
    pub const EMPTY: Environment<'static> = Environment(EnvironmentInner::From(&Context::EMPTY));

    /// Creates an environment from a [`Context`] by mapping each variable to itself.
    pub fn from_context<'a>(ctx: &'a Context<'a>) -> Environment<'a> {
        Environment(EnvironmentInner::From(ctx))
    }

    fn extend<'a>(
        parent: &'a FlatEnvironment,
        var: &'a Identifier,
        val: &'a Value,
    ) -> Environment<'a> {
        Environment(EnvironmentInner::Extend { parent, var, val })
    }

    fn lookup_value(&self, id: &Identifier) -> crate::Result<Value> {
        use EnvironmentInner::*;
        match self.0 {
            From(ctx) => {
                ctx.contains(id)?;
                Ok(Value::Neutral {
                    neu: Neutral::Variable(id.clone()),
                })
            }
            Extend { parent, var, val } => {
                if var == id {
                    Ok(val.clone())
                } else {
                    parent.lookup_value(id)
                }
            }
        }
    }

    fn to_flat_environment(&self) -> FlatEnvironment {
        match self.0 {
            EnvironmentInner::From(ctx) => FlatEnvironment(Dictionary {
                entries: ctx
                    .into_iter()
                    .map(|(var, _)| {
                        (
                            var.clone(),
                            Value::Neutral {
                                neu: Neutral::Variable(var.clone()),
                            },
                        )
                    })
                    .collect(),
            }),
            EnvironmentInner::Extend { parent, var, val } => {
                let mut out = FlatEnvironment(Dictionary {
                    entries: parent.0.entries.clone(),
                });
                out.0.entries.insert(var.clone(), val.clone());
                out
            }
        }
    }
}

/// Determines the [`Value`] of a variable.
pub fn evaluate_var(defs: &Definitions, env: &Environment, var: &Identifier) -> Value {
    env.lookup_value(var)
        .or_else(|_| defs.lookup_value(var).cloned())
        .unwrap()
}

/// An [`Value`] which depends on an argument,
/// and captures the [`Environment`] in which it was created.
#[derive(Clone)]
pub struct Closure {
    env: FlatEnvironment,
    pub param: Identifier,
    pub body: Expression,
}

impl Closure {
    pub(crate) fn new_in_env(env: &Environment, param: Identifier, body: Expression) -> Closure {
        Closure {
            env: env.to_flat_environment(),
            param,
            body,
        }
    }

    /// Creates a closure, capturing the context.
    pub fn new_in_ctx(ctx: &Context, param: Identifier, body: Expression) -> Closure {
        Closure::new_in_env(&Environment::from_context(ctx), param, body)
    }

    pub(crate) fn names(&self) -> impl Iterator<Item = &Identifier> + Clone {
        self.env.names()
    }

    /// Calls the closure an argument.
    pub fn call(&self, defs: &Definitions, val: &Value) -> Value {
        evaluate(
            defs,
            &Environment::extend(&self.env, &self.param, val),
            &self.body,
        )
    }
}
