use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy)]
pub struct Index {
    pub row: usize,
    pub column: usize,
}

#[derive(Debug)]
pub enum CsvError {
    NoSuchRow(usize),
    FailedToReplaceRow(usize),
    NoSuchColumn(usize),
    FailedToReplaceColumn(usize),
    NoSuchField(String),
    NoSuchIndex(Index),
}

impl Display for CsvError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CsvError::NoSuchRow(row) => write!(f, "No such row: {}", row),
            CsvError::FailedToReplaceRow(row) => write!(f, "Failed to replace row: {}", row),
            CsvError::NoSuchColumn(column) => write!(f, "No such column: {}", column),
            CsvError::FailedToReplaceColumn(column) => {
                write!(f, "Failed to replace column: {}", column)
            }
            CsvError::NoSuchField(field) => write!(f, "No such field: {}", field),
            CsvError::NoSuchIndex(index) => {
                write!(f, "Index ({},{}) does not exist", index.row, index.column)
            }
        }
    }
}

impl Error for CsvError {}

pub type CsvResult<T> = Result<T, CsvError>;

pub trait CsvDataHandle {
    fn data_at(&self, index: Index) -> CsvResult<&str>;
    fn row(&self, row: usize) -> CsvResult<Vec<&str>>;
    fn column(&self, column: usize) -> CsvResult<Vec<&str>>;
    fn headers(&self) -> Vec<&str>;
    fn property_count(&self) -> usize;
    fn row_count(&self) -> usize;
    fn column_count(&self) -> usize;
    fn column_of_field(&self, field: &str) -> CsvResult<usize>;

    fn replace_data_at(&mut self, index: Index, new_data: String) -> CsvResult<()>;
    fn replace_column(&mut self, column: usize, new_data: Vec<String>) -> CsvResult<()>;
    fn replace_column_by_field(&mut self, field: &str, new_data: Vec<String>) -> CsvResult<()> {
        let column = self.column_of_field(field)?;
        self.replace_column(column, new_data)?;
        Ok(())
    }
    fn replace_row(&mut self, row: usize, new_data: Vec<String>) -> CsvResult<()>;

    fn delete_row(&mut self, row: usize) -> CsvResult<()>;
    fn delete_column(&mut self, column: usize) -> CsvResult<()>;
    fn delete_column_by_field(&mut self, field: &str) -> CsvResult<()> {
        let column = self.column_of_field(field)?;
        self.delete_column(column)?;
        Ok(())
    }
}
