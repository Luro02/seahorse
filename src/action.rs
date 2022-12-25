use std::sync::Arc;

use crate::Context;

/// Command and application action type
///
/// Example
///
/// ```
/// use seahorse::{Action, Context};
///
/// let action: Action = Action::from(|c: &Context| {
///     println!("{:?}", c.args);
/// });
/// ```
#[derive(Clone)]
pub struct Action {
    inner: Arc<dyn Fn(&Context)>,
}

impl Action {
    pub fn run(&self, context: &Context) {
        (self.inner)(context);
    }
}

// fn(_: &Context) = for<'a> fn(&'a Context)

/*
impl From<fn(_: &Context)> for Action {
    fn from(value: fn(_: &Context)) -> Self {
        unimplemented!()
    }
}*/

impl<F> From<F> for Action
where
    for<'a> F: Fn(&'a Context) + 'static,
{
    fn from(value: F) -> Self {
        Self {
            inner: Arc::new(value),
        }
    }
}
