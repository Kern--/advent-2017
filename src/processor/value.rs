#[derive(Debug, PartialEq, Eq)]
/// Represents a literal or register value
pub enum Value {
    Literal(i64),
    Register(String),
}

impl Value {
    pub fn parse(input: &str) -> Value {
        let parsed = input.parse::<i64>().ok();
        match parsed {
            Some(value) => Value::Literal(value),
            None => Value::Register(input.into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use processor::Environment;

    #[test]
    fn test_parse() {
        let mut input = "a";
        assert_eq!(Value::parse(input), Value::Register("a".into()));

        input = "10";
        assert_eq!(Value::parse(input), Value::Literal(10));
    }

    #[test]
    fn test_get_value() {
        let mut environment = Environment::new();

        assert_eq!(environment.get_value(&Value::Literal(10)), 10);

        environment.set(&"a", 5);
        assert_eq!(environment.get_value(&Value::Register("a".into())), 5);
        assert_eq!(environment.get_value(&Value::Register("b".into())), 0);
    }

}