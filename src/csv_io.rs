use crate::csv_data_handle::*;
use crate::csv_holder::CsvHolder;
use std::{
    error::Error,
    fmt::Display,
    io::{BufRead, Write},
};

#[derive(Debug)]
pub enum CsvIoError {
    InvalidCsv,
    IoError(std::io::Error),
}

impl Display for CsvIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CsvIoError::InvalidCsv => write!(f, "invalid csv"),
            CsvIoError::IoError(e) => write!(f, "io error: {}", e),
        }
    }
}

impl Error for CsvIoError {}

impl From<std::io::Error> for CsvIoError {
    fn from(e: std::io::Error) -> Self {
        CsvIoError::IoError(e)
    }
}

impl From<CsvError> for CsvIoError {
    fn from(_e: CsvError) -> Self {
        CsvIoError::InvalidCsv
    }
}

pub fn read_csv(reader: impl BufRead) -> Result<CsvHolder, CsvIoError> {
    let mut lines = reader.lines();
    let header_line = lines.next().ok_or(CsvIoError::InvalidCsv)??;
    let rows = lines
        .map(|line| -> std::io::Result<Vec<String>> {
            let strs = line?
                .split(',')
                .map(|s| s.to_string())
                .filter(|s| !s.is_empty())
                .collect();
            Ok(strs)
        })
        .collect::<std::io::Result<Vec<_>>>()?;
    let rows = rows
        .into_iter()
        .filter(|r| !r.is_empty())
        .collect::<Vec<_>>();
    CsvHolder::new(
        header_line.split(',').map(|s| s.to_string()).collect(),
        rows,
    )
    .ok_or(CsvIoError::InvalidCsv)
}

pub fn write_csv(
    writer: &mut impl Write,
    data_provider: &impl CsvDataHandle,
) -> Result<(), CsvIoError> {
    write_line(writer, &data_provider.headers())?;
    let row_count = data_provider.row_count();
    for row_index in 1..row_count + 1 {
        let row = data_provider.row(row_index)?;
        write_line(writer, &row)?;
    }
    Ok(())
}

pub fn write_headers(
    writer: &mut impl Write,
    data_provider: &impl CsvDataHandle,
) -> Result<(), CsvIoError> {
    write_line(writer, &data_provider.headers())?;
    Ok(())
}

pub fn write_csv_row_range(
    writer: &mut impl Write,
    data_provider: &impl CsvDataHandle,
    first_row: usize,
    last_row: usize,
) -> Result<(), CsvIoError> {
    for row_index in first_row..last_row + 1 {
        let row = data_provider.row(row_index)?;
        write_line(writer, &row)?;
    }
    Ok(())
}

fn write_line(writer: &mut impl Write, line: &[&str]) -> std::io::Result<()> {
    let line_string = line
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join(",");
    writeln!(writer, "{}", line_string)
}
