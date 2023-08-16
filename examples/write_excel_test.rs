// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

//! An example of writing a Polar Rust dataframe to an Excel file. This example
//! demonstrates autofitting column widths in the output worksheet.

use polars::prelude::*;

fn main() {
    // Create a sample dataframe for the example.
    let mut df: DataFrame = df!(
        "String" => &["North", "South", "East", "West"],
        "Int" => &[1, 2, 3, 4],
        "Float" => &[1.0, 2.22, 3.333, 4.4444],
    )
    .unwrap();

    example(&mut df).unwrap();
}

use polars_excel_writer::PolarsXlsxWriter;

fn example(df: &mut DataFrame) -> PolarsResult<()> {
    let mut xl = PolarsXlsxWriter::new();

    xl.write_dataframe_to_cell(df, 1, 1)?;
    xl.write_excel("dataframe.xlsx")?;

    Ok(())
}
