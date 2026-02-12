use regex::Regex;

///
/// A type that can be validated
///
pub trait ValidateOwned {
    ///
    /// The output type
    ///
    type Output;

    ///
    /// Validates the instance
    ///
    fn validate_owned(&self) -> Result<Self::Output, Error>;
}

///
/// Validates a field with a specified function
///
pub fn validate_field_with<T, F: Fn() -> Result<T, Error>>(name: &str, f: F) -> Result<T, Error> {
    f().map_err(|e| e.with_field(name))
}

///
/// Validates a field from a specified result
/// 
pub fn validate_field<T>(name: &str, result: Result<T, Error>) -> Result<T, Error> {
    result.map_err(|e| e.with_field(name))
}

///
/// Validates a vec of values
///
pub fn validate_vec<T, U, F: Fn(&T) -> Result<U, Error>>(values: &Vec<T>, f: F) -> Result<Vec<U>, Error> {
    let mut result = Vec::with_capacity(values.len());
    for index in 0..values.len() {
        result.push(f(&values[index]).map_err(|e| e.with_element(index))?);
    }
    Ok(result)
}

///
/// Validates a field vec of values
///
pub fn validate_vec_field<T, U, F: Fn(&T) -> Result<U, Error>>(name: &str, values: &Vec<T>, f: F) -> Result<Vec<U>, Error> {
    validate_field_with(name, || validate_vec(values, &f))
}

///
/// Triggers a validation failure
///
pub fn validation_failed<T>(msg: &str) -> Result<T, Error> {
    Err(Error::from_string(msg.to_owned()))
}

///
/// Checks whether the input is a positive integer
///
pub fn positive_integer(number: &i32) -> Result<i32, Error> {
    if number >= &0 {
        Ok(*number)
    } else {
        Err(Error::from_string(format!("should be a positive integer but was {}", number)))
    }
}

///
/// Checks whether the input is a strictly positive floating point number
///
pub fn strictly_positive_f32(number: &f32) -> Result<f32, Error> {
    if number > &0.0 {
        Ok(*number)
    } else {
        Err(Error::from_string(format!("should be a strictly positive floating point number but was {}", number)))
    }
}

///
/// Checks whether the input is a positive floating point number
///
pub fn positive_f32(number: &f32) -> Result<f32, Error> {
    if number >= &0.0 {
        Ok(*number)
    } else {
        Err(Error::from_string(format!("should be a positive floating point number but was {}", number)))
    }
}

///
/// Checks whether the input is a nonempty string
///
pub fn non_empty_string(input: &String) -> Result<String, Error> {
    if input.is_empty() {
        Err(Error::from_string(String::from("should be a non empty string")))
    } else {
        Ok(input.clone())
    }
}

///
/// Matches the pattern and returns any captures as string references
///
pub fn matches_pattern(input: &String, regex: Regex) -> Result<String, Error> {
    if regex.is_match(&input) {
        Ok(input.clone())
    } else {
        Err(Error::from_string(format!("should fit the pattern '{}', but was {}", regex, input)))
    }
}

///
/// Matches the pattern and returns any captures as string references
///
pub fn matches_pattern_and_capture<'a>(input: &'a String, regex: &'a Regex) -> Result<Vec<&'a str>, Error> {
    match regex.captures(&input) {
        Some(caps) => {
            Ok(caps.iter().filter(Option::is_some).map(Option::unwrap).map(|c| c.as_str()).collect())
        },
        None => Err(Error::from_string(format!("should fit the pattern '{}', but was {}", regex, input))),
    }
}

///
/// Validation errors
///
#[derive(Debug, PartialEq)]
pub enum Error {
    ///
    /// Validation failed
    ///
    Failure(String, String),

    ///
    /// An unexpected error occurred
    ///
    Unexpected(String),
}

impl Error {
    ///
    /// Creates an error with the specified message
    ///
    pub(crate) fn from_str(msg: &str) -> Error {
        Error::Failure(msg.to_owned(), String::new())
    }

    ///
    /// Creates an error with the specified message
    ///
    fn from_string(msg: String) -> Error {
        Error::Failure(msg, String::new())
    }

    ///
    /// Adds a field to the front of the path
    ///
    fn with_field(&self, name: &str) -> Error{
        match self {
            Error::Failure(msg, path) => {
                if path.is_empty() {
                    Error::Failure(msg.clone(), format!(".{}", name))
                } else {
                    Error::Failure(msg.clone(), format!(".{}{}", name, path))
                }
            },
            Error::Unexpected(msg) => Error::Unexpected(msg.clone()),
        }
    }

    ///
    /// Adds an array index to the front of the path
    ///
    fn with_element(&self, index: usize) -> Error{
        match self {
            Error::Failure(msg, path) => Error::Failure(msg.clone(), format!("[{}]{}", index, path)),
            Error::Unexpected(msg) => Error::Unexpected(msg.clone()),
        }
    }
}

impl From<regex::Error> for Error {
    fn from(e: regex::Error) -> Error {
        Error::Unexpected(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct TestResult {
        number: i32,
        numbers: Vec<i32>,
        model: SubResult,
        models: Vec<SubResult>,
    }

    #[derive(Debug, PartialEq)]
    struct SubResult {
        number: i32,
        numbers: Vec<i32>,
    }

    struct TestModel {
        number: i32,
        numbers: Vec<i32>,
        model: SubModel,
        models: Vec<SubModel>,
    }

    impl ValidateOwned for TestModel {
        type Output = TestResult;

        fn validate_owned(&self) -> Result<Self::Output, Error> {
            Ok(TestResult {
                number: validate_field_with("number", || positive_integer(&self.number))?,
                numbers: validate_field_with("numbers", || validate_vec(&self.numbers, positive_integer))?,
                model: validate_field_with("model", || self.model.validate_owned())?,
                models: validate_vec_field("models", &self.models, SubModel::validate_owned)?,
            })
        }
    }

    struct SubModel {
        number: i32,
        numbers: Vec<i32>,
    }

    impl ValidateOwned for SubModel {
        type Output = SubResult;
        fn validate_owned(&self) -> Result<Self::Output, Error> {
            Ok(SubResult{
                number: validate_field_with("number", || positive_integer(&self.number))?,
                numbers: validate_vec_field("numbers", &self.numbers, positive_integer)?,
            })
        }
    }

    #[test]
    pub fn validate_positive_integer() {
        let result = positive_integer(&-1);
        assert_eq!(Err(Error::from_str("should be a positive integer but was -1")), result);
        let result = positive_integer(&0);
        assert_eq!(Ok(0), result);
    }

    #[test]
    pub fn validate_positive_f32() {
        let result = positive_f32(&-1.0);
        assert_eq!(Err(Error::from_str("should be a positive floating point number but was -1")), result);
        assert_eq!(Ok(0.0), positive_f32(&0.0));
        assert_eq!(Ok(2.0), positive_f32(&2.0));
    }

    #[test]
    pub fn validate_strictly_positive_f32() {
        assert_eq!(Err(Error::from_str("should be a strictly positive floating point number but was -1")), strictly_positive_f32(&-1.0));
        assert_eq!(Err(Error::from_str("should be a strictly positive floating point number but was 0")), strictly_positive_f32(&0.0));
        assert_eq!(Ok(2.0), strictly_positive_f32(&2.0));
    }

    #[test]
    pub fn validate_regex() {
        let input = String::from("aab");
        let regex = Regex::new(r"^(a+)b*$").unwrap();
        let result = matches_pattern_and_capture(&input, &regex).expect("expected successful match");
        assert_eq!(2, result.len());
        assert_eq!("aab", result[0]);
        assert_eq!("aa", result[1]);
        let input = String::from("bb");
        assert_eq!(Err(Error::from_string(String::from("should fit the pattern '^(a+)b*$', but was bb"))), matches_pattern_and_capture(&input, &regex));
    }

    #[test]
    pub fn validate_non_empty_string() {
        assert_eq!(Err(Error::from_str("should be a non empty string")), non_empty_string(&String::new()));
        let input = String::from("abc");
        assert_eq!(Ok(input.clone()), non_empty_string(&input));

    }

    #[test]
    pub fn validate_model() {
        let input = TestModel {
            number: 1,
            numbers: vec![2, 3, 0],
            model: SubModel {
                number: 3,
                numbers: vec![3, 4],
            },
            models: vec![
                SubModel {
                    number: 2,
                    numbers: vec![3, 4],
                }
            ],
        };
        let expected = TestResult {
            number: 1,
            numbers: vec![2, 3, 0],
            model: SubResult {
                number: 3,
                numbers: vec![3, 4],
            },
            models: vec![
                SubResult {
                    number: 2,
                    numbers: vec![3, 4],
                }
            ],
        };
        assert_eq!(Ok(expected), input.validate_owned());
    }

    #[test]
    pub fn invalid_field() {
        let input = TestModel {
            number: -1,
            numbers: vec![2, 3, 0],
            model: SubModel {
                number: 3,
                numbers: vec![3, 4],
            },
            models: vec![
                SubModel {
                    number: 2,
                    numbers: vec![3, 4],
                }
            ],
        };
        let expected = Error::Failure(String::from("should be a positive integer but was -1"), String::from(".number"));
        assert_eq!(Err(expected), input.validate_owned());
    }

    #[test]
    pub fn invalid_vec_field() {
        let input = TestModel {
            number: 1,
            numbers: vec![2, -3, 0],
            model: SubModel {
                number: 3,
                numbers: vec![3, 4],
            },
            models: vec![
                SubModel {
                    number: 2,
                    numbers: vec![3, 4],
                }
            ],
        };
        let expected = Error::Failure(String::from("should be a positive integer but was -3"), String::from(".numbers[1]"));
        assert_eq!(Err(expected), input.validate_owned());
    }

    #[test]
    pub fn invalid_nested_field() {
        let input = TestModel {
            number: 1,
            numbers: vec![2, 3, 0],
            model: SubModel {
                number: -1,
                numbers: vec![3, 4],
            },
            models: vec![
                SubModel {
                    number: 2,
                    numbers: vec![3, 4],
                }
            ],
        };
        let expected = Error::Failure(String::from("should be a positive integer but was -1"), String::from(".model.number"));
        assert_eq!(Err(expected), input.validate_owned());
    }

    #[test]
    pub fn invalid_nested_array() {
        let input = TestModel {
            number: 1,
            numbers: vec![2, 3, 0],
            model: SubModel {
                number: 1,
                numbers: vec![3, -4],
            },
            models: vec![
                SubModel {
                    number: 2,
                    numbers: vec![3, 4],
                }
            ],
        };
        let expected = Error::Failure(String::from("should be a positive integer but was -4"), String::from(".model.numbers[1]"));
        assert_eq!(Err(expected), input.validate_owned());
    }

    #[test]
    pub fn invalid_nested_model_array() {
        let input = TestModel {
            number: 1,
            numbers: vec![2, 3, 0],
            model: SubModel {
                number: 1,
                numbers: vec![3, 4],
            },
            models: vec![
                SubModel {
                    number: 0,
                    numbers: vec![3, -4],
                }
            ],
        };
        let expected = Error::Failure(String::from("should be a positive integer but was -4"), String::from(".models[0].numbers[1]"));
        assert_eq!(Err(expected), input.validate_owned());
    }
}