// Test case that compares a file generated by polars_excel_writer with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;

use polars::prelude::*;
use polars_excel_writer::PolarsExcelWriter;
use rust_xlsxwriter::{Format, XlsxError};

// Compare output against target Excel file using PolarsExcelWriter. This file
// has bold integers in the first column.

// Use dtype formatting.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let df: DataFrame = df!(
        "Foo" => &[1, 1, 1],
        "Bar" => &[2.4, 2.4, 2.4],
    )?;

    let mut excel_writer = PolarsExcelWriter::new();
    let format = Format::new().set_bold();

    excel_writer.set_dtype_format(DataType::Int32, format);

    excel_writer.write_dataframe(&df)?;
    excel_writer.save(filename)?;

    Ok(())
}

// Ensure column formatting overrides dtype formatting.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let df: DataFrame = df!(
        "Foo" => &[1, 1, 1],
        "Bar" => &[2.4, 2.4, 2.4],
    )?;

    let mut excel_writer = PolarsExcelWriter::new();
    let format = Format::new().set_bold();

    excel_writer.set_dtype_format(DataType::Int32, &format);
    excel_writer.set_column_format("Foo", &format);

    excel_writer.write_dataframe(&df)?;
    excel_writer.save(filename)?;

    Ok(())
}

// Use column formatting.
fn create_new_xlsx_file_3(filename: &str) -> Result<(), XlsxError> {
    let df: DataFrame = df!(
        "Foo" => &[1, 1, 1],
        "Bar" => &[2.4, 2.4, 2.4],
    )?;

    let mut excel_writer = PolarsExcelWriter::new();
    let format = Format::new().set_bold();

    excel_writer.set_column_format("Foo", format);

    excel_writer.write_dataframe(&df)?;
    excel_writer.save(filename)?;

    Ok(())
}

// Use all int dtype formatting.
fn create_new_xlsx_file_4(filename: &str) -> Result<(), XlsxError> {
    let df: DataFrame = df!(
        "Foo" => &[1, 1, 1],
        "Bar" => &[2.4, 2.4, 2.4],
    )?;

    let mut excel_writer = PolarsExcelWriter::new();
    let format = Format::new().set_bold();

    excel_writer.set_dtype_int_format(format);

    excel_writer.write_dataframe(&df)?;
    excel_writer.save(filename)?;

    Ok(())
}

// Use all number dtype formatting.
fn create_new_xlsx_file_5(filename: &str) -> Result<(), XlsxError> {
    let df: DataFrame = df!(
        "Foo" => &[1, 1, 1],
        "Bar" => &[2.4, 2.4, 2.4],
    )?;

    let mut excel_writer = PolarsExcelWriter::new();
    let num_format = Format::new().set_bold();
    let default_format = Format::new();

    excel_writer.set_dtype_number_format(num_format);

    // Override the number format for the "Bar" column.
    excel_writer.set_column_format("Bar", default_format);

    excel_writer.write_dataframe(&df)?;
    excel_writer.save(filename)?;

    Ok(())
}

#[test]
fn dataframe_excelwriter13_1() {
    let test_runner = common::TestRunner::new()
        .set_name("dataframe13")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn dataframe_excelwriter13_2() {
    let test_runner = common::TestRunner::new()
        .set_name("dataframe13")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn dataframe_excelwriter13_3() {
    let test_runner = common::TestRunner::new()
        .set_name("dataframe13")
        .set_function(create_new_xlsx_file_3)
        .unique("3")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn dataframe_excelwriter13_4() {
    let test_runner = common::TestRunner::new()
        .set_name("dataframe13")
        .set_function(create_new_xlsx_file_4)
        .unique("4")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn dataframe_excelwriter13_5() {
    let test_runner = common::TestRunner::new()
        .set_name("dataframe13")
        .set_function(create_new_xlsx_file_5)
        .unique("5")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
