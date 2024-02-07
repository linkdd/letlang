use crate::{
  data::LLValue,
  thread::LLInterrupt,
};

use genawaiter::sync::Co;

pub type LLCoroutine = Co<LLInterrupt, LLValue>;
