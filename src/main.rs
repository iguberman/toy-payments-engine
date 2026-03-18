mod account;
mod input_parser;
mod transaction;
mod tx_processor;

use crate::input_parser::{process_record, RawTxRecord};
use crate::tx_processor::TxProcessor;
use csv::ReaderBuilder;
use std::io::{Read, Write};

const STREAMING: bool = false;

/// Ingest transactions from any `Read` (file, stdin, in-memory buffer, ...).
/// * `flexible = true` allows dispute/resolve/chargeback rows with fewer columns.
/// * `trim = All` .
/// * Invalid rows will result in errors printed to stderror
pub fn ingest<R: Read, W: Write>(reader: R, mut writer: W, tx_processor: &mut TxProcessor) {
    let mut csv_reader = ReaderBuilder::new()
        .trim(csv::Trim::All) //strips whitespace around every field
        .flexible(true) // allows dispute/resolve/chargeback rows with fewer columns
        .from_reader(reader);

    for (row_idx, result) in csv_reader.deserialize::<RawTxRecord>().enumerate() {
        let row_num = row_idx + 1;
        match result {
            Err(e) => {
                eprintln!("Row {row_num}: CSV parsing error: {e:?}")
            }
            Ok(raw_tx_record) => match process_record(&raw_tx_record, tx_processor) {
                Ok(account) => {
                    if STREAMING {
                        // in streaming mode, keep outputting the resulting account state
                        writeln!(writer, "{},{account}", raw_tx_record.client).unwrap();
                    }
                }
                Err(e) => {
                    eprintln!("Row {row_num}: {raw_tx_record} : Processing error: {e:?}");
                }
            },
        }
    }

    tx_processor.output_accounts(writer);
}

pub fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut tx_processor = TxProcessor::new();
    if args.len() > 1 {
        let file = std::fs::File::open(&args[1]).expect("Unable to open file");
        ingest(file, std::io::stdout(), &mut tx_processor)
    } else {
        ingest(std::io::stdin(), std::io::stdout(), &mut tx_processor)
    };
}
