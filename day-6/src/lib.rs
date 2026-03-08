use std::fmt::{Debug, Display, Formatter};

struct Worksheet<'a> {
    cells: Vec<Vec<&'a str>>,
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

type Result<T> = std::result::Result<T, WorksheetError>;

impl<'a> Worksheet<'a> {
    fn parse(input: &'a str) -> Self {
        let cells = input
            .lines()
            .map(|line| line.split_whitespace().collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();
        let column_count = cells[0].len();
        let row_count = cells.len();
        Self {
            cells,
            column_count,
            row_count,
        }
    }

    fn reduce_column(&self, column_index: usize) -> Result<u64> {
        let reduce_fn = match self.cells[self.row_count - 1][column_index] {
            "*" => |left, right| left * right,
            "+" => |left, right| left + right,
            other => {
                return Err(WorksheetError {
                    cause: format!("Unable to parse operator {}", other),
                });
            }
        };
        (0..self.row_count - 1)
            .into_iter()
            .map(|row_index| self.cells[row_index][column_index])
            .map(|value| value.parse::<u64>().expect("Unable to parse value"))
            .reduce(reduce_fn)
            .ok_or(WorksheetError {
                cause: "Worksheet not containing any columns".to_string(),
            })
    }

    fn compute_grand_total(&self) -> u64 {
        (0..self.column_count)
            .into_iter()
            .map(|column_index| self.reduce_column(column_index))
            .flatten()
            .sum()
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
        assert_eq!(worksheet.reduce_column(0), Ok(33210));
        assert_eq!(worksheet.reduce_column(1), Ok(490));
        assert_eq!(worksheet.reduce_column(2), Ok(4243455));
        assert_eq!(worksheet.reduce_column(3), Ok(401));
        assert_eq!(worksheet.compute_grand_total(), 4277556);
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("./resource/input.txt").expect("Failed to read input file.");
        let worksheet = Worksheet::parse(&input);
        assert_eq!(worksheet.compute_grand_total(), 5524274308182);
    }
}
