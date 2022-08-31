use crate::*;

pub fn entrypoint(func: &dyn Function) {
  let context = Arc::new(Mutex::new(Context::new()));

  let main_args: Vec<Value> = vec![];
  let mut block = func.call(context.clone(), main_args);

  let ignored = Value::Primitive(PrimitiveValue::Boolean(bool::default()));
  let mut state = block.resume_with(ignored);

  loop {
    match &state {
      GeneratorState::Yielded(FunctionInterruption::Effect { name, args }) => {
        let args_tuple = Value::Tuple(args.clone());

        match name.as_str() {
          "debug" => {
            println!("{}", context.lock().unwrap().format_value(&args_tuple));

            let ok = Value::Primitive(PrimitiveValue::Atom(
              context.lock().unwrap().get_atom("@ok"))
            );
            state = block.resume_with(ok);
          },
          _ => {
            eprintln!(
              "Effect {}{} not implemented.",
              name,
              context.lock().unwrap().format_value(&args_tuple),
            );
            std::process::exit(3);
          }
        }
      },
      GeneratorState::Yielded(FunctionInterruption::Exception(exc)) => {
        eprintln!(
          "Uncaught exception: {}",
          context.lock().unwrap().format_value(&exc),
        );
        std::process::exit(2);
      },
      GeneratorState::Complete(val) => {
        let ok_atom = context.lock().unwrap().get_atom("@ok");
        match val {
          Value::Primitive(PrimitiveValue::Atom(atom)) if *atom == ok_atom => {
            break;
          },
          _ => {
            eprintln!(
              "Main function returned non-ok result: {}",
              context.lock().unwrap().format_value(val),
            );
            std::process::exit(1);
          }
        }
      }
    }
  }
}
