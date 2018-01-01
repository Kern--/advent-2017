use std::convert::TryFrom;
use std::fmt::{self, Debug, Formatter, Display};
use std;

#[derive(PartialEq, Eq, Clone, Hash)]
/// Models a square matrix where each cell can either be on or off
pub struct Matrix {
    data: Vec<Vec<bool>>
}

impl Matrix {
    /// Creates and initializes a square matrix of size len x len 
    pub fn new(len: usize) -> Matrix {
        let mut data = Vec::with_capacity(len);
        for _ in 0..len {
            data.push((0..len).map(|_| false).collect::<Vec<bool>>());
        }
        Matrix { data }
    }

    /// Joins a vector of of square matrices into a single matrix
    ///  The smaller matrices should all be the same size and in row major order
    pub fn join(parts: Vec<Matrix>) -> Matrix {
        if parts.len() == 0 {
            return Matrix::new(0);
        }
        let num_sub_matrices = parts.len();
        let sub_matrix_len = parts[0].rows();
        let joined_len = (num_sub_matrices as f64).sqrt() as usize * sub_matrix_len;
        let mut result = Matrix::new(joined_len);
        let parts_per_row = joined_len/sub_matrix_len;
        for i in 0..parts_per_row {
            for j in 0..parts_per_row {
                result.splice(i*sub_matrix_len, j*sub_matrix_len, &parts[i*parts_per_row + j]);
            }
        }
        result
    }

    /// Rotates a matrix 90 degrees clockwise 
    pub fn rotate(&self) -> Matrix {
        let mut new_matrix = self.clone();
        for i in 0..self.rows() {
            for j in 0..self.columns() {
                let new_i = j;
                let new_j = new_matrix.rows() - 1 - i;
                new_matrix.data[new_i][new_j] = self.data[i][j];
            }
        }
        new_matrix
    }

    /// Flips a matrix horizontally
    pub fn flip(&self) -> Matrix {
        let mut new_matrix = self.clone();
        for row in &mut new_matrix.data {
            row.reverse();
        }
        new_matrix
    }

    /// The number of rows in the matrix
    pub fn rows(&self) -> usize {
        self.data.len()
    }

    /// The number of columns in the matrix
    pub fn columns(&self) -> usize {
        if self.data.len() > 0 {
            return self.data[0].len();
        }
        0
    }

    /// The number of cells which are set to be on
    pub fn num_on_pixels(&self) -> usize {
        self.data.iter()
            .flat_map(|row| row.iter().map(|x| *x as usize))
            .fold(0, |sum, val| sum + val)
    }

    /// Inserts `other` into the matrix with `other`'s upper-left corner at (`row`, `size`).
    ///  If the matrix is not large enough to insert `other` at (`row`, `size`), nothing will happen
    /// 
    /// # Example
    /// inserting:
    /// ```
    /// ##
    /// ##
    /// ```
    /// into
    /// ```
    /// ...
    /// ...
    /// ...
    /// ```
    /// at (1,1) produces
    /// ```
    /// ...
    /// .##
    /// .##
    /// ```
    pub fn splice(&mut self, row: usize, column: usize, other: &Matrix) {
        // Ensure there is enough space in self to splice the other matrix
        if row + other.rows() > self.rows() 
            || column + other.columns() > self.columns() {
                return;
            }
        
        for i in 0..other.rows() {
            for j in 0..other.columns() {
                self.data[row + i][column + j] = other.data[i][j];
            }
        }
    }

    /// Splits a matrix into a vector of submatrices in row-major order.
    ///  If the matrix's side length is even, then the submatrices will be 2x2
    ///  otherwise the submatrices will be 3x3
    pub fn split(self) -> Vec<Matrix> {
        let split_size = if self.data.len() % 2 == 0 { 2 } else { 3 };
        let mut result = Vec::new();
        for self_i in 0..(self.rows() / split_size) {
            for self_j in 0..(self.columns() / split_size) {
                let mut new_matrix = Matrix::new(split_size);
                for i in 0..split_size {
                    for j in 0..split_size {
                        new_matrix.data[i][j] = self.data[self_i * split_size + i][self_j * split_size + j];
                    }
                }
                result.push(new_matrix);
            }
        }
        result
    }
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        let mut display = String::new();
        for row in &self.data {
            display.push('\n');
            for column in row {
                match *column {
                    true => display.push('#'),
                    false => display.push('.')
                }
            }
        }
        return write!(f, "{}", display);
    }
}

impl <'a> TryFrom<&'a str> for Matrix {
    type Error = ParseError<'a>;

    fn try_from(value: &'a str) -> Result<Matrix, Self::Error> {
        // cells are allowed to be one of:
        //  '#' = on/true
        //  '.' = off/false
        let parse_cell = |i, j, c| match c {
            '#' => Ok(true),
            '.' => Ok(false),
            _ => Err(ParseError::InvalidCharacter(value, i, j, c))
        };
        // Parses a row of cells like ### or #.#
        //  into a list of bools
        let parse_row = |(row_num, row) : (usize, &str)| row.chars().enumerate()
            .map(|(j, c)| parse_cell(row_num, j, c))
            .collect::<Result<Vec<bool>, Self::Error>>();

        // Parse the input matrix
        let data = value.split("/").enumerate().map(parse_row).collect::<Result<Vec<Vec<bool>>, Self::Error>>()?;

        // Ensure the data is well formed
        let expected_dimension = data.len();
        for (row_num, row) in data.iter().enumerate() {
            if row.len() != expected_dimension {
                return Err(ParseError::NonSquareMatrix(value, expected_dimension, row_num, row.len()));
            }
        }
        Ok(Matrix { data })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError<'a> {
    InvalidCharacter(&'a str, usize, usize, char),
    NonSquareMatrix(&'a str, usize, usize, usize)
}

impl <'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ParseError::InvalidCharacter(input, i, j, c) => write!(f, "Invalid character: {} in {}. At input matrix: ({}, {})", c, input, i, j),
            ParseError::NonSquareMatrix(input, expected, bad_row, bad_row_len) => write!(f, "Non-square Matrix detected in {3}. Expected {0}x{0} matrix, but row {1} was {2}", expected, bad_row, bad_row_len, input)
        }
    }
}

impl <'a> std::error::Error for ParseError<'a> {
    fn description(&self) -> &str {
        match *self {
            ParseError::InvalidCharacter(_, _, _, _) => "Invalid Character",
            ParseError::NonSquareMatrix(_, _, _, _) => "Non-square Matrix"
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_on_pixels() {
        let mut matrix = Matrix::new(3);
        matrix.data[0][0] = true;
        matrix.data[1][1] = true;
        matrix.data[2][2] = true;
        matrix.data[2][1] = true;
        matrix.data[1][2] = true;

        assert_eq!(matrix.num_on_pixels(), 5);
    }

    #[test]
    fn test_successful_parse() {
        let input = "#../..#/###";
        let data = vec![vec![true, false, false], vec![false, false, true], vec![true, true, true]];
        assert_eq!(Matrix::try_from(&input), Ok(Matrix{ data }));
    }

    #[test]
    fn test_invalid_character_parse() {
        let input = "#.!/..#/###";
        assert_eq!(Matrix::try_from(&input), Err(ParseError::InvalidCharacter(input, 0, 2, '!')));
    }

    #[test]
    fn test_non_square_matrix_parse() {
        let input = "#../../###";
        assert_eq!(Matrix::try_from(&input), Err(ParseError::NonSquareMatrix(input, 3, 1, 2)));
    }

    #[test]
    fn test_rotate() {
        // ##  => .#
        // ..     .#
        let mut data = vec![vec![true, true], vec![false, false]];
        let mut rotated_data = vec![vec![false, true], vec![false, true]];
        assert_eq!(Matrix{data}.rotate(), Matrix{data: rotated_data});

        // #.. => ###
        // ##.    ##.
        // ###    #..
        data = vec![vec![true, false, false], vec![true, true, false], vec![true, true, true]];
        rotated_data = vec![vec![true, true, true], vec![true, true, false], vec![true, false, false]];
        assert_eq!(Matrix{data}.rotate(), Matrix{data: rotated_data});

        // ##.. => #..#
        // .##.    ..##
        // ..##    .##.
        // #..#    ##..
        data = vec![vec![true, true, false, false], vec![false, true, true, false], vec![false, false, true, true], vec![true, false, false, true]];
        rotated_data = vec![vec![true, false, false, true], vec![false, false, true, true], vec![false, true, true, false], vec![true, true, false, false]];
        assert_eq!(Matrix{data}.rotate(), Matrix{data: rotated_data});
    }

    #[test]
    fn test_flip() {
        // #. => .#
        // #.    .#
        let mut data = vec![vec![true, false], vec![true, false]];
        let mut flipped_data = vec![vec![false, true], vec![false, true]];
        assert_eq!(Matrix{data}.flip(), Matrix{data: flipped_data});

        // #.. => ..#
        // ##.    .##
        // ###    ###
        data = vec![vec![true, false, false], vec![true, true, false], vec![true, true, true]];
        flipped_data = vec![vec![false, false, true], vec![false, true, true], vec![true, true, true]];
        assert_eq!(Matrix{data}.flip(), Matrix{data: flipped_data});

        // ##.. => ..##
        // .##.    .##.
        // ..##    ##..
        // #..#    #..#
        data = vec![vec![true, true, false, false], vec![false, true, true, false], vec![false, false, true, true], vec![true, false, false, true]];
        flipped_data = vec![vec![false, false, true, true], vec![false, true, true, false], vec![true, true, false, false], vec![true, false, false, true]];
        assert_eq!(Matrix{data}.flip(), Matrix{data: flipped_data});
    }

    #[test]
    fn test_split() {
        let mut data = vec![vec![true, false], vec![true, false]];
        assert_eq!(Matrix{ data: data.clone()}.split(), vec![Matrix{data}]);

        data = vec![vec![true, false, false], vec![true, true, false], vec![true, true, true]];
        assert_eq!(Matrix{ data: data.clone()}.split(), vec![Matrix{data}]);


        // #.#..# => [#.  #.  .#  #.  #.  .#  #.  #.  .# 
        // #.#..#     #., #., .#, #., #., .#, #., #., .#]
        // #.#..#
        // #.#..#
        // #.#..#
        // #.#..#
        data = vec![vec![true, false, true, false, false, true], 
                    vec![true, false, true, false, false, true],
                    vec![true, false, true, false, false, true], 
                    vec![true, false, true, false, false, true],
                    vec![true, false, true, false, false, true], 
                    vec![true, false, true, false, false, true],];
        
        let split_type1 = vec![vec![true, false], vec![true, false]];
        let split_type2 = vec![vec![false, true], vec![false, true]];
        let split_matrices: Vec<Matrix> = vec![split_type1.clone(), split_type1.clone(), split_type2.clone(),
                                split_type1.clone(), split_type1.clone(), split_type2.clone(),
                                split_type1.clone(), split_type1.clone(), split_type2.clone()]
                        .into_iter().map(|split_data| Matrix {data: split_data}).collect();
        assert_eq!(Matrix{data}.split(), split_matrices);
    }

    #[test]
    fn test_splice() {
        // ... with ## spliced at (1,1) => ...
        // ...      ##                     .##
        // ...                             .##

        let mut matrix = Matrix::new(3);
        let data = vec![vec![true, true], vec![true, true]];
        let splice_matrix = Matrix {data};
        let spliced_data = vec![vec![false, false, false], vec![false, true, true], vec![false, true, true]];
        matrix.splice(1, 1, &splice_matrix);
        assert_eq!(matrix, Matrix { data: spliced_data});
    }

    #[test]
    fn test_join() {
        // [#.  #.  .#  #.  #.  .#  #.  #.  .#  =>  #.#..#
        //  #., #., .#, #., #., .#, #., #., .#]     #.#..#     
        //                                          #.#..#
        //                                          #.#..#
        //                                          #.#..#
        //                                          #.#..#
        let data = vec![vec![true, false, true, false, false, true], 
                        vec![true, false, true, false, false, true],
                        vec![true, false, true, false, false, true], 
                        vec![true, false, true, false, false, true],
                        vec![true, false, true, false, false, true], 
                        vec![true, false, true, false, false, true],];
        
        let split_type1 = vec![vec![true, false], vec![true, false]];
        let split_type2 = vec![vec![false, true], vec![false, true]];
        let split_matrices: Vec<Matrix> = vec![split_type1.clone(), split_type1.clone(), split_type2.clone(),
                                split_type1.clone(), split_type1.clone(), split_type2.clone(),
                                split_type1.clone(), split_type1.clone(), split_type2.clone()]
                        .into_iter().map(|split_data| Matrix {data: split_data}).collect();
        assert_eq!(Matrix::join(split_matrices), Matrix{data});
    }
}
