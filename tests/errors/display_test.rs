use letlang::errors::LetlangError;

#[test]
fn it_should_format_notimplementederror() {
  let res = format!("{}", LetlangError::NotImplementedError {
    message: "test-case".to_string()
  });

  assert_eq!("NotImplementedError: test-case", res);
}