pub mod error;

use std::{
    collections::HashMap,
    fs::{read, remove_file},
};

use uuid::Uuid;
use xlsxwriter::Workbook;

use self::error::ExcelError;

/// Excel export util
///
/// # Performance test
/// ```
/// 100000 rows, 10 columns, 4.332097613s
/// 100000 rows, 30 columns, 12.265614828s
/// 100000 rows, 100 columns, 54.567792084s
/// 1000000 rows, 10 columns, 45.134951892s
/// 1000000 rows, 30 columns, 128.813227389s
/// ```
pub struct Excel;

impl Excel {
    /// export one sheet with datas
    /// - `rows`: export data rows and columns
    /// - `file_path`: excel file path and name
    ///
    /// # Example
    /// ```rust
    /// let cols = vec!["cell1", "cell2"];
    /// let rows = vec![cols];
    /// Excel::write(rows, "path/to/demo.xlsx");
    /// ```
    pub fn write(rows: Vec<Vec<&str>>, file_path: &str) -> Result<(), ExcelError> {
        let mut sheets_and_rows = HashMap::new();
        sheets_and_rows.insert("sheet1", rows);
        Self::write_with_sheet(sheets_and_rows, file_path)
    }

    /// export multi sheets with data
    /// - `sheets_and_rows`: export sheet with data rows and columns
    /// - `file_path`: excel file path and name
    ///
    /// # Example
    /// ```rust
    /// let cols = vec!["cell1", "cell2"];
    /// let rows = vec![cols];
    /// let sheets_and_rows = HashMap::new();
    /// sheets_and_rows.insert("sheet1", rows);
    /// Excel::write_with_sheet(sheets_and_rows, "path/to/demo.xlsx");
    /// ```
    pub fn write_with_sheet(
        sheets_and_rows: HashMap<&str, Vec<Vec<&str>>>,
        file_path: &str,
    ) -> Result<(), ExcelError> {
        let workbook = Workbook::new(file_path);

        for (sheet, rows) in sheets_and_rows {
            let mut sheet1 = workbook
                .add_worksheet(Some(sheet))
                .map_err(ExcelError::Export)?;
            for (row_index, row) in rows.into_iter().enumerate() {
                for (col_index, col) in row.into_iter().enumerate() {
                    sheet1
                        .write_string(
                            row_index.try_into().unwrap(),
                            col_index.try_into().unwrap(),
                            col,
                            None,
                        )
                        .map_err(ExcelError::Export)?;
                }
            }
        }
        workbook.close().map_err(ExcelError::Export)?;

        Ok(())
    }

    /// export a sheet with data, return bytes
    /// - `rows`: export rows and columns
    ///
    /// # Example
    /// ```rust
    /// let cols = vec!["cell1", "cell2"];
    /// let rows = vec![cols];
    /// Excel::write_and_get_bytes(rows);
    /// ```
    pub fn write_and_get_bytes(rows: Vec<Vec<&str>>) -> Result<Vec<u8>, ExcelError> {
        let mut sheets_and_rows = HashMap::new();
        sheets_and_rows.insert("sheet1", rows);
        Self::write_with_sheet_and_get_bytes(sheets_and_rows)
    }

    /// export multi sheets with data, return bytes
    /// - `sheets_and_rows`: export sheet with data rows and columns
    ///
    /// # Example
    /// ```rust
    /// let cols = vec!["cell1", "cell2"];
    /// let rows = vec![cols];
    /// let sheets_and_rows = HashMap::new();
    /// sheets_and_rows.insert("sheet1", rows);
    /// Excel::write_with_sheet_and_get_bytes(sheets_and_rows, "path/to/demo.xlsx");
    /// ```
    pub fn write_with_sheet_and_get_bytes(
        sheets_and_rows: HashMap<&str, Vec<Vec<&str>>>,
    ) -> Result<Vec<u8>, ExcelError> {
        let uuid = Uuid::new_v4().to_string();
        Self::write_with_sheet(sheets_and_rows, &uuid)?;
        let result = read(&uuid).map_err(ExcelError::IO)?;
        remove_file(&uuid).map_err(ExcelError::IO)?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, vec};

    use crate::Excel;

    #[test]
    fn test_write() {
        let cols = vec!["cell1", "cell2"];
        let rows = vec![cols];
        Excel::write(rows, "/Volumes/code/temp/simple1.xlsx").unwrap();
    }

    #[test]
    fn test_write_with_sheets() {
        let cols = vec!["cell1", "cell2"];
        let rows = vec![cols];
        let mut sheets_and_rows = HashMap::new();
        sheets_and_rows.insert("sheet1", rows.clone());
        sheets_and_rows.insert("sheet2", rows);
        Excel::write_with_sheet(sheets_and_rows, "/Volumes/code/temp/simple1.xlsx").unwrap();
    }

    #[test]
    fn test_write_and_get_bytes() {
        let cols = vec!["cell1", "cell2"];
        let rows = vec![cols];
        Excel::write_and_get_bytes(rows).unwrap();
    }

    #[test]
    fn test_write_with_sheet_and_get_bytes() {
        let cols = vec!["cell1", "cell2"];
        let rows = vec![cols];
        let mut sheets_and_rows = HashMap::new();
        sheets_and_rows.insert("sheet1", rows.clone());
        sheets_and_rows.insert("sheet2", rows);
        Excel::write_with_sheet_and_get_bytes(sheets_and_rows).unwrap();
    }
}
