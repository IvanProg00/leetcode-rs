//! [Excel Sheet Column Number](https://leetcode.com/problems/excel-sheet-column-number)

const FROM_START_TO_END: i32 = 'Z' as i32 - 'A' as i32 + 1;

/// Gets a string column_title that represents the column title as appears in an Excel sheet, return its corresponding column number.
pub fn title_to_number(column_title: String) -> i32 {
    column_title.chars().fold(0, |acc, c| {
        acc * FROM_START_TO_END + (c as i32 - 'A' as i32) + 1
    })
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("A", 1)]
    #[case("AB", 28)]
    #[case("ZY", 701)]
    fn test_title_to_number(#[case] column_title: String, #[case] expected: i32) {
        let result = title_to_number(column_title);
        assert_eq!(result, expected);
    }
}
