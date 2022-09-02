use crate::core::function::FunctionInterruption;
use crate::repr::Value;

use genawaiter::sync::Co;

pub type FunctionCoroutine = Co<FunctionInterruption, Value>;
