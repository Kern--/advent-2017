use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{self, Formatter, Display};
use std;

use super::matrix::{self, Matrix};

#[derive(Debug, PartialEq, Eq)]
/// Holds a list of rules for enhancing matrices
pub struct RuleBook {
    rules: HashMap<Matrix, Matrix>
}

impl RuleBook {
    /// Parses a rule of the form `##/.. => #.#/#.#/#.#`
    /// into left and right matrices
    fn parse_rule(rule: &str) -> Result<(Matrix, Matrix), ParseError> {
        let parts = rule.split(" => ").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(ParseError::InvalidRule(rule))
        }
        let key = matrix::Matrix::try_from(parts[0])?;
        let value = matrix::Matrix::try_from(parts[1])?;
        Ok((key, value))
    }

    fn enhanced_matrix<'a>(&self, matrix: Matrix) -> Result<Matrix, EnhanceError> {
        let enhanced = self.rules.get(&matrix).ok_or(EnhanceError::NoRule(matrix))?;
        Ok(enhanced.clone())
    }

    /// Enhances a matrix according to the rulse of the rule book
    pub fn enhance(&self, matrix: Matrix) -> Result<Matrix, EnhanceError> {
        // the process becomes:
        // 1) Split the matrix
        // 2) Enhance each sub matrix
        // 3) Join the sub matrices back into the final, enhanced matrix
        matrix.split().into_iter()
            .map(|sub_matrix| self.enhanced_matrix(sub_matrix))
            .collect::<Result<Vec<Matrix>, EnhanceError>>()
            .and_then(|enhanced_matrices| Ok(Matrix::join(enhanced_matrices)))
    }
}

impl <'a> TryFrom<&'a str> for RuleBook {
    type Error = ParseError<'a>;

    fn try_from(value: &'a str) -> Result<RuleBook, Self::Error> {
        let mut rulebook = RuleBook { rules: HashMap::new() };
        let rules = value.split("\n").collect::<Vec<&str>>();
        // There probably is some way to do this more efficiently.
        //  For now, add a separate rule for every variation of the matrix
        for rule in rules {
            let (mut key, value) = RuleBook::parse_rule(rule)?;
        
            // Add a rule for all 4 rotations of the matrix
            for _ in 0..4 {
                rulebook.rules.insert(key.clone(), value.clone());
                key = key.rotate();
            }
            // Flip the matrix and then add a rule for all 4 rotations of the flipped matrix
            key = key.rotate().flip();
            for _ in 0..4 {
                rulebook.rules.insert(key.clone(), value.clone());
                key = key.rotate();
            }
        }
        Ok(rulebook)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    InvalidRule(&'a str),
    InvalidMatrix(matrix::ParseError<'a>)
}

impl <'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ParseError::InvalidMatrix(ref matrix_parse_error) => write!(f, "Invalid matrix {}", matrix_parse_error),
            ParseError::InvalidRule(rule) => write!(f, "Invalid rule: {}", rule)
        }
    }
}

impl <'a> std::error::Error for ParseError<'a> {
    fn description(&self) -> &str {
        match *self {
            ParseError::InvalidMatrix(_) => "Invalid Matrix",
            ParseError::InvalidRule(_) => "Invalid Rule"
        }
    }
}

impl <'a> From <matrix::ParseError<'a>> for ParseError<'a> {
    fn from(error: matrix::ParseError<'a>) -> ParseError<'a> {
        ParseError::InvalidMatrix(error)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum EnhanceError{
    NoRule( Matrix)
}

impl  Display for EnhanceError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &EnhanceError::NoRule(ref matrix) => write!(f, "No rule for enhancing matrix: {:?}", matrix)
        }
    }
}

impl std::error::Error for EnhanceError {
    fn description(&self) -> &str {
        match *self {
            EnhanceError::NoRule(_) => "No rule to enhance matrix"
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_rule() {
        let input = "###/.../... => ###/###/###";
        let left = Matrix::try_from("###/.../...").unwrap();
        let right = Matrix::try_from("###/###/###").unwrap();
        assert_eq!(RuleBook::parse_rule(input), Ok((left, right)));
    }

    #[test]
    fn test_from_str() {
        let input = "##./.../... => ###/###/###";
        let rulebook = RuleBook::try_from(input).unwrap();
        assert_eq!(rulebook.rules.len(), 8);
    }

    #[test]
    fn test_enhanced_matrix() {
        let input = "../.. => ##/##\n../.# => #./#.\n##/#. => #./..";
        let rulebook = RuleBook::try_from(input).unwrap();
        let matrix = Matrix::try_from("../..").unwrap();
        let enhanced_matrix = rulebook.enhanced_matrix(matrix).unwrap();
        let expected = Matrix::try_from("##/##").unwrap();
        assert_eq!(enhanced_matrix, expected);
    }

    #[test]
    fn test_application() {
        let rules = "../.# => ##./#../...\n.#./..#/### => #..#/..../..../#..#";
        let rulebook = RuleBook::try_from(rules).unwrap();
        let start = Matrix::try_from(".#./..#/###").unwrap();
        let first_enhancement = rulebook.enhance(start);
        assert_eq!(first_enhancement, Ok(Matrix::try_from("#..#/..../..../#..#").unwrap()));
        let second_enahncement = rulebook.enhance(first_enhancement.unwrap());
        assert_eq!(second_enahncement, Ok(Matrix::try_from("##.##./#..#../....../##.##./#..#../......").unwrap()));
        assert_eq!(second_enahncement.unwrap().num_on_pixels(), 12);
    }
}