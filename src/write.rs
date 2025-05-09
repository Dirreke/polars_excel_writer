// excel_writer - A Polars extension to serialize dataframes to Excel xlsx files.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

#![warn(missing_docs)]

use std::io::{Seek, Write};

use polars::prelude::*;
use rust_xlsxwriter::Format;

use crate::PolarsXlsxWriter;

/// `ExcelWriter` implements the Polars [`SerWriter`] trait to serialize a
/// dataframe to an Excel Xlsx file.
///
/// `ExcelWriter` provides a simple interface for writing to an Excel file
/// similar to Polars [`CsvWriter`]. However, unless you specifically need to
/// use the `SerWriter` trait you should instead use the more configurable
/// [`PolarsXlsxWriter`] serializer interface, which is also part of this crate.
///
/// `ExcelWriter` uses `PolarsXlsxWriter` to do the Excel serialization which in
/// turn uses the [`rust_xlsxwriter`] crate.
///
///  # Examples
///
/// An example of writing a Polar Rust dataframe to an Excel file.
///
/// ```
/// # // This code is available in examples/excelwriter_intro.rs
/// #
/// use chrono::prelude::*;
/// use polars::prelude::*;
///
/// use polars_excel_writer::ExcelWriter;
///
/// fn main() -> PolarsResult<()> {
///     // Create a sample dataframe for the example.
///     let mut df: DataFrame = df!(
///         "String" => &["North", "South", "East", "West"],
///         "Integer" => &[1, 2, 3, 4],
///         "Float" => &[4.0, 5.0, 6.0, 7.0],
///         "Time" => &[
///             NaiveTime::from_hms_milli_opt(2, 59, 3, 456).unwrap(),
///             NaiveTime::from_hms_milli_opt(2, 59, 3, 456).unwrap(),
///             NaiveTime::from_hms_milli_opt(2, 59, 3, 456).unwrap(),
///             NaiveTime::from_hms_milli_opt(2, 59, 3, 456).unwrap(),
///             ],
///         "Date" => &[
///             NaiveDate::from_ymd_opt(2022, 1, 1).unwrap(),
///             NaiveDate::from_ymd_opt(2022, 1, 2).unwrap(),
///             NaiveDate::from_ymd_opt(2022, 1, 3).unwrap(),
///             NaiveDate::from_ymd_opt(2022, 1, 4).unwrap(),
///             ],
///         "Datetime" => &[
///             NaiveDate::from_ymd_opt(2022, 1, 1).unwrap().and_hms_opt(1, 0, 0).unwrap(),
///             NaiveDate::from_ymd_opt(2022, 1, 2).unwrap().and_hms_opt(2, 0, 0).unwrap(),
///             NaiveDate::from_ymd_opt(2022, 1, 3).unwrap().and_hms_opt(3, 0, 0).unwrap(),
///             NaiveDate::from_ymd_opt(2022, 1, 4).unwrap().and_hms_opt(4, 0, 0).unwrap(),
///         ],
///     )?;
///
///     // Create a new file object.
///     let mut file = std::fs::File::create("dataframe.xlsx").unwrap();
///
///     // Write the dataframe to an Excel file using the Polars SerWriter interface.
///     ExcelWriter::new(&mut file).finish(&mut df)?;
///
///     Ok(())
/// }
/// ```
///
/// Output file:
///
/// <img src="https://rustxlsxwriter.github.io/images/excelwriter_intro.png">
///
pub struct ExcelWriter<W>
where
    W: Write,
{
    writer: W,
    xlsx_writer: PolarsXlsxWriter,
}

impl<W> SerWriter<W> for ExcelWriter<W>
where
    W: Write + Seek + Send,
{
    fn new(buffer: W) -> Self {
        ExcelWriter {
            writer: buffer,
            xlsx_writer: PolarsXlsxWriter::default(),
        }
    }

    fn finish(&mut self, df: &mut DataFrame) -> PolarsResult<()> {
        self.xlsx_writer.save_to_writer(df, &mut self.writer)?;

        Ok(())
    }
}

impl<W> ExcelWriter<W>
where
    W: Write,
{
    /// Turn on/off the dataframe header in the exported Excel file.
    ///
    /// Turn on/off the dataframe header row in the Excel table. It is on by
    /// default.
    ///
    /// # Parameters
    ///
    /// - `has_header` - Export dataframe with/without header.
    ///
    /// # Examples
    ///
    /// An example of writing a Polar Rust dataframe to an Excel file. This
    /// demonstrates saving the dataframe with a header (which is the default).
    ///
    /// ```
    /// # // This code is available in examples/excelwriter_has_header_on.rs
    /// #
    /// use polars::prelude::*;
    ///
    /// use polars_excel_writer::ExcelWriter;
    ///
    /// fn main() -> PolarsResult<()> {
    ///     // Create a sample dataframe for the example.
    ///     let mut df: DataFrame = df!(
    ///         "String" => &["North", "South", "East", "West"],
    ///         "Int" => &[1, 2, 3, 4],
    ///         "Float" => &[1.0, 2.22, 3.333, 4.4444],
    ///     )?;
    ///
    ///     // Create a new file object.
    ///     let mut file = std::fs::File::create("dataframe.xlsx").unwrap();
    ///
    ///     // Write the dataframe to an Excel file using the Polars SerWriter
    ///     // interface. This example also turns off the default header.
    ///     ExcelWriter::new(&mut file)
    ///         .has_header(false)
    ///         .finish(&mut df)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Output file:
    ///
    /// <img
    /// src="https://rustxlsxwriter.github.io/images/excelwriter_has_header_on.png">
    ///
    /// If we set `has_header()` to `false` we can output the dataframe from the
    /// previous example without the header row:
    ///
    /// ```
    /// # // This code is available in examples/excelwriter_has_header_off.rs
    /// #
    /// use polars::prelude::*;
    ///
    /// use polars_excel_writer::ExcelWriter;
    ///
    /// fn main() -> PolarsResult<()> {
    ///     // Create a sample dataframe for the example.
    ///     let mut df: DataFrame = df!(
    ///         "String" => &["North", "South", "East", "West"],
    ///         "Int" => &[1, 2, 3, 4],
    ///         "Float" => &[1.0, 2.22, 3.333, 4.4444],
    ///     )?;
    ///
    ///     // Create a new file object.
    ///     let mut file = std::fs::File::create("dataframe.xlsx").unwrap();
    ///
    ///     // Write the dataframe to an Excel file using the Polars SerWriter
    ///     // interface. This example also turns on the header (this is the default).
    ///     ExcelWriter::new(&mut file)
    ///         .has_header(false)
    ///         .finish(&mut df)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Output file:
    ///
    /// <img
    /// src="https://rustxlsxwriter.github.io/images/excelwriter_has_header_off.png">
    ///
    pub fn has_header(mut self, has_header: bool) -> Self {
        self.xlsx_writer.set_header(has_header);
        self
    }

    /// Set the Excel number format for time values.
    ///
    /// [Datetimes in Excel] are stored as f64 floats with a format used to
    /// display them. The default time format used by this library is
    /// `hh:mm:ss;@`. This method can be used to specify an alternative user
    /// defined format.
    ///
    /// [Datetimes in Excel]:
    ///     ../../rust_xlsxwriter/struct.ExcelDateTime.html#datetimes-in-excel
    ///
    /// # Parameters
    ///
    /// - `format` - A `rust_xlsxwriter` [`Format`] or an Excel number format
    ///   string that can be converted to a `Format`.
    ///
    /// # Examples
    ///
    /// An example of writing a Polar Rust dataframe to an Excel file. This
    /// example demonstrates how to change the default format for Polars time
    /// types.
    ///
    /// ```
    /// # // This code is available in examples/excelwriter_time_format.rs
    /// #
    /// use chrono::prelude::*;
    /// use polars::prelude::*;
    ///
    /// use polars_excel_writer::ExcelWriter;
    ///
    /// fn main() -> PolarsResult<()> {
    ///     // Create a sample dataframe for the example.
    ///     let mut df: DataFrame = df!(
    ///         "Time" => &[
    ///             NaiveTime::from_hms_milli_opt(2, 00, 3, 456).unwrap(),
    ///             NaiveTime::from_hms_milli_opt(2, 18, 3, 456).unwrap(),
    ///             NaiveTime::from_hms_milli_opt(2, 37, 3, 456).unwrap(),
    ///             NaiveTime::from_hms_milli_opt(2, 59, 3, 456).unwrap(),
    ///         ],
    ///     )?;
    ///
    ///     // Create a new file object.
    ///     let mut file = std::fs::File::create("dataframe.xlsx").unwrap();
    ///
    ///     // Write the dataframe to an Excel file using the Polars SerWriter
    ///     // interface. This example also adds a time format.
    ///     ExcelWriter::new(&mut file)
    ///         .with_time_format("hh:mm")
    ///         .finish(&mut df)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Output file:
    ///
    /// <img
    /// src="https://rustxlsxwriter.github.io/images/excelwriter_time_format.png">
    ///
    pub fn with_time_format(mut self, format: impl Into<Format>) -> Self {
        self.xlsx_writer.set_dtype_format(DataType::Time, format);
        self
    }

    /// Set the Excel number format for date values.
    ///
    /// [Datetimes in Excel] are stored as f64 floats with a format used to
    /// display them. The default date format used by this library is
    /// `yyyy-mm-dd;@`. This method can be used to specify an alternative user
    /// defined format.
    ///
    /// [Datetimes in Excel]:
    ///     ../../rust_xlsxwriter/struct.ExcelDateTime.html#datetimes-in-excel
    ///
    /// # Parameters
    ///
    /// - `format` - A `rust_xlsxwriter` [`Format`] or an Excel number format
    ///   string that can be converted to a `Format`.
    ///
    /// # Examples
    ///
    /// An example of writing a Polar Rust dataframe to an Excel file. This example
    /// demonstrates how to change the default format for Polars date types.
    ///
    /// ```
    /// # // This code is available in examples/excelwriter_date_format.rs
    /// #
    /// use chrono::prelude::*;
    /// use polars::prelude::*;
    ///
    /// use polars_excel_writer::ExcelWriter;
    ///
    /// fn main() -> PolarsResult<()> {
    ///     // Create a sample dataframe for the example.
    ///     let mut df: DataFrame = df!(
    ///         "Date" => &[
    ///             NaiveDate::from_ymd_opt(2023, 1, 11),
    ///             NaiveDate::from_ymd_opt(2023, 1, 12),
    ///             NaiveDate::from_ymd_opt(2023, 1, 13),
    ///             NaiveDate::from_ymd_opt(2023, 1, 14),
    ///         ],
    ///     )?;
    ///
    ///     // Create a new file object.
    ///     let mut file = std::fs::File::create("dataframe.xlsx").unwrap();
    ///
    ///     // Write the dataframe to an Excel file using the Polars SerWriter
    ///     // interface. This example also adds a date format.
    ///     ExcelWriter::new(&mut file)
    ///         .with_date_format("mmm d yyyy")
    ///         .finish(&mut df)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Output file:
    ///
    /// <img src="https://rustxlsxwriter.github.io/images/excelwriter_date_format.png">
    ///
    pub fn with_date_format(mut self, format: impl Into<Format>) -> Self {
        self.xlsx_writer.set_dtype_format(DataType::Date, format);
        self
    }

    /// Set the Excel number format for datetime values.
    ///
    /// [Datetimes in Excel] are stored as f64 floats with a format used to
    /// display them. The default datetime format used by this library is
    /// `yyyy-mm-dd hh:mm:ss`. This method can be used to specify an alternative
    /// user defined format.
    ///
    /// [Datetimes in Excel]:
    ///     ../../rust_xlsxwriter/struct.ExcelDateTime.html#datetimes-in-excel
    ///
    /// # Parameters
    ///
    /// - `format` - A `rust_xlsxwriter` [`Format`] or an Excel number format
    ///   string that can be converted to a `Format`.
    ///
    /// # Examples
    ///
    /// An example of writing a Polar Rust dataframe to an Excel file. This
    /// example demonstrates how to change the default format for Polars
    /// datetime types.
    ///
    /// ```
    /// # // This code is available in examples/excelwriter_datetime_format.rs
    /// #
    /// use chrono::prelude::*;
    /// use polars::prelude::*;
    ///
    /// use polars_excel_writer::ExcelWriter;
    ///
    /// fn main() -> PolarsResult<()> {
    ///     // Create a sample dataframe for the example.
    ///     let mut df: DataFrame = df!(
    ///         "Datetime" => &[
    ///             NaiveDate::from_ymd_opt(2023, 1, 11).unwrap().and_hms_opt(1, 0, 0).unwrap(),
    ///             NaiveDate::from_ymd_opt(2023, 1, 12).unwrap().and_hms_opt(2, 0, 0).unwrap(),
    ///             NaiveDate::from_ymd_opt(2023, 1, 13).unwrap().and_hms_opt(3, 0, 0).unwrap(),
    ///             NaiveDate::from_ymd_opt(2023, 1, 14).unwrap().and_hms_opt(4, 0, 0).unwrap(),
    ///         ],
    ///     )?;
    ///
    ///     // Create a new file object.
    ///     let mut file = std::fs::File::create("dataframe.xlsx").unwrap();
    ///
    ///     // Write the dataframe to an Excel file using the Polars SerWriter
    ///     // interface. This example also adds a datetime format.
    ///     ExcelWriter::new(&mut file)
    ///         .with_datetime_format("hh::mm - mmm d yyyy")
    ///         .finish(&mut df)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Output file:
    ///
    /// <img
    /// src="https://rustxlsxwriter.github.io/images/excelwriter_datetime_format.png">
    ///
    pub fn with_datetime_format(mut self, format: impl Into<Format>) -> Self {
        self.xlsx_writer.set_dtype_datetime_format(format);
        self
    }

    /// Set the Excel number format for floats.
    ///
    /// Set the Excel number format for f32/f64 float types using an Excel
    /// number format string. These format strings can be obtained from the
    /// `Format Cells -> Number` dialog in Excel.
    ///
    /// See the [Number Format Categories] section and subsequent Number Format
    /// sections in the `rust_xlsxwriter` documentation.
    ///
    /// [Number Format Categories]:
    ///     ../../rust_xlsxwriter/struct.Format.html#number-format-categories
    ///
    /// Note, the numeric values aren't truncated in Excel, this option just
    /// controls the display of the number.
    ///
    /// # Parameters
    ///
    /// - `format` - A `rust_xlsxwriter` [`Format`] or an Excel number format
    ///   string that can be converted to a `Format`.
    ///
    /// # Examples
    ///
    /// An example of writing a Polar Rust dataframe to an Excel file. This
    /// demonstrates setting an Excel number format for floats.
    ///
    /// ```
    /// # // This code is available in examples/excelwriter_float_format.rs
    /// #
    /// use polars::prelude::*;
    ///
    /// use polars_excel_writer::ExcelWriter;
    ///
    /// fn main() -> PolarsResult<()> {
    ///     // Create a sample dataframe for the example.
    ///     let mut df: DataFrame = df!(
    ///         "Float" => &[1000.0, 2000.22, 3000.333, 4000.4444],
    ///     )?;
    ///
    ///     // Create a new file object.
    ///     let mut file = std::fs::File::create("dataframe.xlsx").unwrap();
    ///
    ///     // Write the dataframe to an Excel file using the Polars SerWriter
    ///     // interface. This example also adds a float format.
    ///     ExcelWriter::new(&mut file)
    ///         .with_float_format("#,##0.00")
    ///         .finish(&mut df)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Output file:
    ///
    /// <img
    /// src="https://rustxlsxwriter.github.io/images/excelwriter_float_format.png">
    ///
    pub fn with_float_format(mut self, format: impl Into<Format>) -> Self {
        self.xlsx_writer.set_dtype_float_format(format);
        self
    }

    /// Set the Excel number precision for floats.
    ///
    /// Set the number precision of all floats exported from the dataframe to
    /// Excel. The precision is converted to an Excel number format (see
    /// [`with_float_format()`](ExcelWriter::with_float_format) above), so for
    /// example 3 is converted to the Excel format `0.000`.
    ///
    /// The precision should be in the Excel range 1-30.
    ///
    /// Note, the numeric values aren't truncated in Excel, this option just
    /// controls the display of the number.
    ///
    /// # Parameters
    ///
    /// - `precision` - The floating point precision in the Excel range 1-30.
    ///
    /// # Examples
    ///
    /// An example of writing a Polar Rust dataframe to an Excel file. This
    /// example demonstrates how to set the precision of the float output.
    /// Setting the precision to 3 is equivalent to an Excel number format of
    /// `0.000`.
    ///
    /// ```
    /// # // This code is available in examples/excelwriter_float_precision.rs
    /// #
    /// use polars::prelude::*;
    ///
    /// use polars_excel_writer::ExcelWriter;
    ///
    /// fn main() -> PolarsResult<()> {
    ///     // Create a sample dataframe for the example.
    ///     let mut df: DataFrame = df!(
    ///         "Float" => &[1.0, 2.22, 3.333, 4.4444],
    ///     )?;
    ///
    ///     // Create a new file object.
    ///     let mut file = std::fs::File::create("dataframe.xlsx").unwrap();
    ///
    ///     // Write the dataframe to an Excel file using the Polars SerWriter
    ///     // interface. This example also adds a float precision.
    ///     ExcelWriter::new(&mut file)
    ///         .with_float_precision(3)
    ///         .finish(&mut df)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Output file:
    ///
    /// <img
    /// src="https://rustxlsxwriter.github.io/images/excelwriter_float_precision.png">
    ///
    pub fn with_float_precision(mut self, precision: usize) -> Self {
        self.xlsx_writer.set_float_precision(precision);
        self
    }

    /// Replace Null values in the exported dataframe with string values.
    ///
    /// By default Null values in a dataframe aren't exported to Excel and will
    /// appear as empty cells. If you wish you can specify a string such as
    /// "Null", "NULL" or "N/A" as an alternative.
    ///
    /// # Parameters
    ///
    /// - `value` - A replacement string for Null values.
    ///
    /// # Examples
    ///
    /// An example of writing a Polar Rust dataframe to an Excel file. This
    /// demonstrates setting a value for Null values in the dataframe. The
    /// default is to write them as blank cells.
    ///
    /// ```
    /// # // This code is available in examples/excelwriter_null_values.rs
    /// #
    /// # use polars::prelude::*;
    /// #
    /// # use polars_excel_writer::ExcelWriter;
    /// #
    /// # fn main() -> PolarsResult<()> {
    /// #     // Create a dataframe with Null values.
    /// #     let csv_string = "Foo,Bar\nNULL,B\nA,B\nA,NULL\nA,B\n";
    /// #     let buffer = std::io::Cursor::new(csv_string);
    /// #     let mut df = CsvReadOptions::default()
    /// #         .map_parse_options(|parse_options| {
    /// #             parse_options.with_null_values(Some(NullValues::AllColumnsSingle("NULL".into())))
    /// #         })
    /// #         .into_reader_with_file_handle(buffer)
    /// #         .finish()
    /// #         .unwrap();
    /// #
    ///     // Create a new file object.
    ///     let mut file = std::fs::File::create("dataframe.xlsx").unwrap();
    ///
    ///     // Write the dataframe to an Excel file using the Polars SerWriter
    ///     // interface. This example also sets a string value for Null values.
    ///     ExcelWriter::new(&mut file)
    ///         .with_null_value("Null")
    ///         .finish(&mut df)?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    /// Output file:
    ///
    /// <img
    /// src="https://rustxlsxwriter.github.io/images/excelwriter_null_values.png">
    ///
    pub fn with_null_value(mut self, null_value: impl Into<String>) -> Self {
        self.xlsx_writer.set_null_value(null_value);
        self
    }

    /// Simulate autofit for columns in the dataframe output.
    ///
    /// Use a simulated autofit to adjust dataframe columns to the maximum
    /// string or number widths.
    ///
    /// There are several limitations to this autofit method, see the
    /// `rust_xlsxwriter` docs on [`worksheet.autofit()`] for details.
    ///
    /// [`worksheet.autofit()`]:
    ///     ../../rust_xlsxwriter/worksheet/struct.Worksheet.html#method.autofit
    ///
    /// # Examples
    ///
    /// An example of writing a Polar Rust dataframe to an Excel file. This
    /// example demonstrates autofitting column widths in the output worksheet.
    ///
    /// ```
    /// # // This code is available in examples/excelwriter_autofit.rs
    /// #
    /// use polars::prelude::*;
    ///
    /// use polars_excel_writer::ExcelWriter;
    ///
    /// fn main() -> PolarsResult<()> {
    ///     // Create a sample dataframe for the example.
    ///     let mut df: DataFrame = df!(
    ///         "Col 1" => &["A", "B", "C", "D"],
    ///         "Column 2" => &["A", "B", "C", "D"],
    ///         "Column 3" => &["Hello", "World", "Hello, world", "Ciao"],
    ///         "Column 4" => &[1234567, 12345678, 123456789, 1234567],
    ///     )?;
    ///
    ///     // Create a new file object.
    ///     let mut file = std::fs::File::create("dataframe.xlsx").unwrap();
    ///
    ///     // Write the dataframe to an Excel file using the Polars SerWriter
    ///     // interface. This example also autofits the output.
    ///     ExcelWriter::new(&mut file).with_autofit().finish(&mut df)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// Output file:
    ///
    /// <img
    /// src="https://rustxlsxwriter.github.io/images/excelwriter_autofit.png">
    ///
    pub fn with_autofit(mut self) -> Self {
        self.xlsx_writer.set_autofit(true);
        self
    }
}
