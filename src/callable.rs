use crate::{errors::RuntimeError, interpreter::Interpreter, types::Object};

pub trait Callable {
    fn from_obj(obj: &Object) -> Result<dyn Self, RuntimeError>;

    fn call(
        &mut self,
        interpreter: &Interpreter,
        arguments: &[Object],
    ) -> Result<Object, RuntimeError>;

    fn arity(&self) -> usize;
}
