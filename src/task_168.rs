//! [Excel Sheet Column Title](https://leetcode.com/problems/excel-sheet-column-title)

const LAST_LETTER_POS: i32 = (b'Z' - b'A' + 1) as i32;

/// Gets an integer column_number, returns its corresponding column title as it
/// appears in an Excel sheet.
pub fn convert_to_title(mut column_number: i32) -> String {
    let mut result = String::new();

    while column_number > 0 {
        column_number -= 1;

        let c = (column_number % LAST_LETTER_POS) as u8 + b'A';
        result.insert(0, c as char);

        column_number /= LAST_LETTER_POS;
    }

    result
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(1, "A")]
    #[case(28, "AB")]
    #[case(701, "ZY")]
    fn test_convert_to_title(#[case] column_number: i32, #[case] expected: String) {
        let result = convert_to_title(column_number);
        assert_eq!(result, expected);
    }
}
