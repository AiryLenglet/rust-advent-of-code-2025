use std::fmt::{Debug, Display, Formatter};

struct Worksheet {
    cells: Vec<Vec<char>>,
    column_count: usize,
    row_count: usize,
}

#[derive(Debug, PartialEq)]
struct WorksheetError {
    cause: String,
}

impl Display for WorksheetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Worksheet error")
    }
}

impl std::error::Error for WorksheetError {}

impl From<std::num::ParseIntError> for WorksheetError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self {
            cause: format!("{}", error),
        }
    }
}

type Result<T> = std::result::Result<T, WorksheetError>;

impl Worksheet {
    fn parse(input: &str) -> Self {
        let cells = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let column_count = cells.iter().map(|row| row.len()).max().unwrap();
        let row_count = cells.len();
        Self {
            cells,
            column_count,
            row_count,
        }
    }

    fn compute_grand_total(&self) -> Result<u64> {
        let mut total: u64 = 0;
        let mut numbers: Vec<u64> = Vec::new();

        for column_index in (0..self.column_count).rev() {
            let mut column_number_str = String::new();

            for row_index in 0..self.row_count - 1 {
                let cell_value = self.cells[row_index]
                    .get(column_index)
                    .unwrap_or_else(|| &' ');
                column_number_str.push(*cell_value);
            }
            if !column_number_str.trim().is_empty() {
                let digit = column_number_str.trim().parse::<u64>()?;
                numbers.push(digit);
            }

            let char_op = self.cells[self.row_count - 1][column_index];
            if char_op == '*' {
                let product = numbers.iter().product::<u64>();
                total += product;
                numbers.clear();
            } else if char_op == '+' {
                let sum = numbers.iter().sum::<u64>();
                total += sum;
                numbers.clear();
            }
        }
        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use crate::Worksheet;
    use std::fs;

    #[test]
    fn test_sample() {
        let worksheet = Worksheet::parse(
            "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ",
        );
        assert_eq!(worksheet.compute_grand_total(), Ok(3263827));
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("./resource/input.txt").expect("Failed to read input file.");
        let worksheet = Worksheet::parse(&input);
        assert_eq!(worksheet.compute_grand_total(), Ok(8843673199391));
    }
}
