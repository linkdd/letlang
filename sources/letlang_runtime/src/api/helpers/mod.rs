mod type_check;
mod function;

pub use self::{
  type_check::assert_type,
  function::call_function,
};
