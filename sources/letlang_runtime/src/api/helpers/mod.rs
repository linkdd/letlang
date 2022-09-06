mod type_check;
mod function;
mod params;

pub use self::{
  type_check::assert_type,
  params::assert_param_count,
  function::call_function,
};
