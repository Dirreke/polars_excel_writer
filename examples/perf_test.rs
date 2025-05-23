// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

//! Simple performance test to compare with the Python Polars example in
//! `perf_test.py`.

use std::time::Instant;

use chrono::prelude::*;
use polars::prelude::*;
use polars_excel_writer::PolarsExcelWriter;

const DATA_SIZE: usize = 250_000;

fn main() {
    // Create a sample dataframe for testing.
    let df: DataFrame = df!(
        "Int" => &[1; DATA_SIZE],
        "Float" => &[123.456789; DATA_SIZE],
        "Date" => &[Utc::now().date_naive(); DATA_SIZE],
        "String" => &["Test"; DATA_SIZE],
    )
    .unwrap();

    let timer = Instant::now();
    example(&df).unwrap();
    println!("Elapsed time: {:.2?}", timer.elapsed());
}

fn example(df: &DataFrame) -> PolarsResult<()> {
    let mut excel_writer = PolarsExcelWriter::new();
    excel_writer.write_dataframe(df)?;
    excel_writer.save("dataframe_rs.xlsx")?;

    Ok(())
}
