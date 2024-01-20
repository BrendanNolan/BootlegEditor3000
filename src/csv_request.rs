use crate::{csv_data_handle::CsvDataHandle, csv_io::*};
use std::{fs::File, io::Write};

pub enum CsvRequest {
    Help,
    Display,
    DisplayHeaders,
    DisplayRowRange(usize, usize),
    ModifyRow {
        row: usize,
        new_data: Vec<String>,
    },
    DeleteRow(usize),
    ModifyColumn {
        column: usize,
        new_data: Vec<String>,
    },
    DeleteColumn(usize),
    ModifyColumnByName {
        column_name: String,
        new_data: Vec<String>,
    },
    DeleteColumnByName(String),
    Dimensions,
    WriteToFile(File),
}

pub fn parse_csv_request(s: &str) -> Option<CsvRequest> {
    match s {
        "display" => {
            return Some(CsvRequest::Display);
        }
        "help" => {
            return Some(CsvRequest::Help);
        }
        "dimensions" => {
            return Some(CsvRequest::Dimensions);
        }
        "display_headers" => {
            return Some(CsvRequest::DisplayHeaders);
        }
        _ => {}
    };
    let (command, args) = s.split_once(' ')?;
    match command {
        "display_row_range" => {
            let mut args = args.splitn(2, ' ');
            let first_row = args.next()?.parse::<usize>().ok()?;
            let last_row = args.next()?.parse::<usize>().ok()?;
            Some(CsvRequest::DisplayRowRange(first_row, last_row))
        }
        "modify_row" => {
            let mut args = args.splitn(2, ' ');
            let row = args.next()?.parse::<usize>().ok()?;
            let new_data = args.next()?.split(',').map(|s| s.to_string()).collect();
            Some(CsvRequest::ModifyRow { row, new_data })
        }
        "delete_row" => {
            let row = args.parse::<usize>().ok()?;
            Some(CsvRequest::DeleteRow(row))
        }
        "modify_column" => {
            let mut args = args.splitn(2, ' ');
            let column = args.next()?.parse::<usize>().ok()?;
            let new_data = args.next()?.split(',').map(|s| s.to_string()).collect();
            Some(CsvRequest::ModifyColumn { column, new_data })
        }
        "delete_column" => {
            let column = args.parse::<usize>().ok()?;
            Some(CsvRequest::DeleteColumn(column))
        }
        "modify_column_by_name" => {
            let mut args = args.splitn(2, ' ');
            let column_name = args.next()?.to_string();
            let new_data = args.next()?.split(',').map(|s| s.to_string()).collect();
            Some(CsvRequest::ModifyColumnByName {
                column_name,
                new_data,
            })
        }
        "delete_column_by_name" => {
            let column_name = args.to_string();
            Some(CsvRequest::DeleteColumnByName(column_name))
        }
        "write_to_file" => {
            let file = File::create(args).ok()?;
            Some(CsvRequest::WriteToFile(file))
        }
        _ => None,
    }
}

pub fn handle_csv_request(
    csv_request: CsvRequest,
    csv_holder: &mut impl CsvDataHandle,
    writer: &mut impl Write,
) -> Result<(), CsvIoError> {
    match csv_request {
        CsvRequest::Help => write_help_text(writer),
        CsvRequest::Display => write_csv(writer, csv_holder),
        CsvRequest::DisplayRowRange(start, end) => {
            write_csv_row_range(writer, csv_holder, start, end)
        }
        CsvRequest::ModifyRow { row, new_data } => Ok(csv_holder.replace_row(row, new_data)?),
        CsvRequest::DeleteRow(row) => Ok(csv_holder.delete_row(row)?),
        CsvRequest::ModifyColumn { column, new_data } => {
            Ok(csv_holder.replace_column(column, new_data)?)
        }
        CsvRequest::DeleteColumn(column) => Ok(csv_holder.delete_column(column)?),
        CsvRequest::ModifyColumnByName {
            column_name,
            new_data,
        } => Ok(csv_holder.replace_column_by_field(&column_name, new_data)?),
        CsvRequest::DeleteColumnByName(column_name) => {
            Ok(csv_holder.delete_column_by_field(&column_name)?)
        }
        CsvRequest::Dimensions => {
            let rows = csv_holder.row_count();
            let columns = csv_holder.column_count();
            writeln!(writer, "Rows: {}, Columns: {}", rows, columns)?;
            Ok(())
        }
        CsvRequest::WriteToFile(mut file) => write_csv(&mut file, csv_holder),
        CsvRequest::DisplayHeaders => write_headers(writer, csv_holder),
    }
}

pub fn write_help_text(writer: &mut impl Write) -> Result<(), CsvIoError> {
    writeln!(writer, "display")?;
    writeln!(writer, "display_row_range <first_row> <last_row>")?;
    writeln!(writer, "modify_row <row> <new_data>")?;
    writeln!(writer, "delete_row <row>")?;
    writeln!(writer, "modify_column <column> <new_data>")?;
    writeln!(writer, "delete_column <column>")?;
    writeln!(writer, "modify_column_by_name <column_name> <new_data>")?;
    writeln!(writer, "delete_column_by_name <column_name>")?;
    writeln!(writer, "dimensions")?;
    writeln!(writer, "write_to_file <file_name>")?;
    writeln!(writer, "display_headers")?;
    writeln!(writer, "CTRL+C to quit")?;
    Ok(())
}
