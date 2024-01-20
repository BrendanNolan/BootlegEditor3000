mod csv_data_handle;
mod csv_holder;
mod csv_io;
mod csv_request;

use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut stdout = std::io::stdout().lock();
    let Ok(file) = std::fs::File::open("testdata.csv") else {
        return writeln!(stdout, "Error opening CSV file");
    };
    let file = std::io::BufReader::new(file);
    match csv_io::read_csv(file) {
        Ok(csv_holder) => run_app_loop(csv_holder, &mut stdout)?,
        Err(e) => {
            return writeln!(stdout, "Error reading CSV: {}", e);
        }
    }
    Ok(())
}

fn run_app_loop(
    mut csv_holder: csv_holder::CsvHolder,
    out: &mut impl Write,
) -> std::io::Result<()> {
    writeln!(
        out,
        "Welcome To BootlegEditor3000. Your CSV Data Has Been Loaded."
    )?;
    writeln!(
        out,
        "Below is a list of commands. To see this at any time, type 'help'."
    )?;
    if csv_request::write_help_text(out).is_err() {
        return writeln!(out, "Error writing to stdout");
    }
    loop {
        write!(out, ">>>> ")?;
        out.flush()?;
        let Some(csv_request) = read_csv_request_from_stdin() else {
            writeln!(out, "CSV Request Entered Incorrectly")?;
            continue;
        };
        if let Err(e) = csv_request::handle_csv_request(csv_request, &mut csv_holder, out) {
            writeln!(out, "CSV Request Failed: {}", e)?;
        }
    }
}

fn read_csv_request_from_stdin() -> Option<csv_request::CsvRequest> {
    let mut csv_request = String::new();
    std::io::stdin().read_line(&mut csv_request).ok()?;
    let csv_request = csv_request.trim();
    csv_request::parse_csv_request(csv_request)
}
