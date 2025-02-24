// Utilities for working with polars dataframes.
use polars::prelude::*;

// Read in a CSV file
pub fn read_csv(path: &str) -> DataFrame {
    CsvReader::from_path(path)
        .unwrap()
        .finish()
        .unwrap()
}

// Print 'n' rows of a Dataframe
pub fn print_df(df: &DataFrame, n: usize) {
    println!("{}", df.head(Some(n)));
}

// Print the shape of a Dataframe
pub fn print_shape(df: &DataFrame) {
    println!("{:?}", df.shape());
}

// Print the scheme of a Dataframe
pub fn print_scheme(df: &DataFrame) {
    println!("{:?}", df.schema());
}

