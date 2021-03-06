use ruschm::{
    error::ToLocated,
    error::{ErrorData, SchemeError},
    interpreter::{error::LogicError, Interpreter},
    list,
    parser::pair::GenericPair,
    values::Number,
    values::Pair,
    values::Type,
    values::Value,
};

#[test]
fn list() -> Result<(), SchemeError> {
    let mut interpreter = Interpreter::<f32>::new_with_stdlib();
    assert_eq!(
        interpreter.eval("(list 1 2 3)".chars())?,
        Some(Value::Pair(Box::new(list!(
            Value::Number(Number::Integer(1)),
            Value::Number(Number::Integer(2)),
            Value::Number(Number::Integer(3))
        ))))
    );

    assert_eq!(
        interpreter.eval("'(1 2 . 3)".chars())?,
        Some(Value::Pair(Box::new(Pair::Some(
            Value::Number(Number::Integer(1)),
            Value::Pair(Box::new(Pair::Some(
                Value::Number(Number::Integer(2)),
                Value::Number(Number::Integer(3))
            )))
        ))))
    );

    assert_eq!(
        interpreter.eval("(car (list 1 2))".chars())?,
        Some(Value::Number(Number::Integer(1)))
    );

    assert_eq!(
        interpreter.eval("(cdr (list 1 2))".chars())?,
        Some(Value::Pair(Box::new(list!(Value::Number(
            Number::Integer(2)
        )))))
    );

    assert_eq!(
        interpreter.eval("(pair? (list 1 2))".chars())?,
        Some(Value::Boolean(true))
    );

    assert_eq!(
        interpreter.eval("(pair? '())".chars())?,
        Some(Value::Boolean(false))
    );
    assert_eq!(
        interpreter.eval("(pair? 1)".chars())?,
        Some(Value::Boolean(false))
    );
    Ok(())
}

#[test]
fn vector() -> Result<(), SchemeError> {
    use ruschm::values::ValueReference;
    let mut interpreter = Interpreter::<f32>::new_with_stdlib();
    assert_eq!(
        interpreter.eval("(vector 1 2 3)".chars())?,
        Some(Value::Vector(ValueReference::new_mutable(vec![
            Value::Number(Number::Integer(1)),
            Value::Number(Number::Integer(2)),
            Value::Number(Number::Integer(3)),
        ])))
    );
    assert_eq!(
        interpreter.eval("(vector)".chars())?,
        Some(Value::Vector(ValueReference::new_mutable(vec![])))
    );
    Ok(())
}

#[test]
fn apply() -> Result<(), SchemeError> {
    let mut interpreter = Interpreter::<f32>::new_with_stdlib();
    assert_eq!(
        interpreter.eval("(apply + 1 2 '(3 4))".chars())?,
        Some(Value::Number(Number::Integer(10)))
    );
    assert_eq!(
        interpreter.eval("(apply + 1)".chars()),
        Err(ErrorData::Logic(LogicError::TypeMisMatch("1".to_owned(), Type::Pair)).no_locate())
    );
    Ok(())
}

#[test]
fn booleans() -> Result<(), SchemeError> {
    {
        let mut interpreter = Interpreter::<f32>::new_with_stdlib();
        assert_eq!(
            interpreter.eval("(if #f #t #f)".chars())?,
            Some(Value::Boolean(false))
        );
        assert_eq!(
            interpreter.eval("(if #t #t #f)".chars())?,
            Some(Value::Boolean(true))
        );
        assert_eq!(
            interpreter.eval("(if '() #t #f)".chars())?,
            Some(Value::Boolean(true))
        );
        assert_eq!(
            interpreter.eval("(if 1 #t #f)".chars())?,
            Some(Value::Boolean(true))
        );
        assert_eq!(
            interpreter.eval("(if 0 #t #f)".chars())?,
            Some(Value::Boolean(true))
        );
    }

    {
        let mut interpreter = Interpreter::<f32>::new_with_stdlib();
        assert_eq!(
            interpreter.eval("(not #f)".chars())?,
            Some(Value::Boolean(true))
        );
        assert_eq!(
            interpreter.eval("(not #t)".chars())?,
            Some(Value::Boolean(false))
        );
        assert_eq!(
            interpreter.eval("(not '())".chars())?,
            Some(Value::Boolean(false))
        );
        assert_eq!(
            interpreter.eval("(not 1)".chars())?,
            Some(Value::Boolean(false))
        );
        assert_eq!(
            interpreter.eval("(not 0)".chars())?,
            Some(Value::Boolean(false))
        );
    }

    {
        let mut interpreter = Interpreter::<f32>::new_with_stdlib();
        assert_eq!(
            interpreter.eval("(boolean? #t)".chars())?,
            Some(Value::Boolean(true))
        );
        assert_eq!(
            interpreter.eval("(boolean? #f)".chars())?,
            Some(Value::Boolean(true))
        );
        assert_eq!(
            interpreter.eval("(boolean? '())".chars())?,
            Some(Value::Boolean(false))
        );
        assert_eq!(
            interpreter.eval("(boolean? 1)".chars())?,
            Some(Value::Boolean(false))
        );
        assert_eq!(
            interpreter.eval("(boolean? 0)".chars())?,
            Some(Value::Boolean(false))
        );
    }

    {
        let mut interpreter = Interpreter::<f32>::new_with_stdlib();
        assert_eq!(
            interpreter.eval("(boolean=? #t #t)".chars())?,
            Some(Value::Boolean(true))
        );
        assert_eq!(
            interpreter.eval("(boolean=? #f #f)".chars())?,
            Some(Value::Boolean(true))
        );
        assert_eq!(
            interpreter.eval("(boolean=? #f #t)".chars())?,
            Some(Value::Boolean(false))
        );
        assert_eq!(
            interpreter.eval("(boolean=? 1 1)".chars()),
            Err(
                ErrorData::Logic(LogicError::TypeMisMatch("1".to_string(), Type::Boolean))
                    .no_locate()
            )
        );
    }
    Ok(())
}
