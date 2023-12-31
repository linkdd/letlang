use crate::{LLValue, LLInterrupt};

use genawaiter::sync::Co;

pub type LLCoroutine = Co<LLInterrupt, LLValue>;
