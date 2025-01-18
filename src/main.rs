// makes the csv crate accessible to your program
extern crate csv;

use chrono::NaiveDate;
use plotters::prelude::*;
use rust_decimal::Decimal;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io;
use std::process;

#[derive(Debug)]
enum StatementSource {
    Discover,
    Chase,
}

#[derive(Debug)]
struct Transaction {
    date: NaiveDate,

    description: String,

    amount: Decimal,

    category: String,

    source: StatementSource,

    // Optional fields for additional Chase data
    post_date: Option<NaiveDate>,

    transaction_type: Option<String>,

    memo: Option<String>,
}

impl std::fmt::Display for Transaction{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Transaction: {} - ${}", self.description, self.amount)
    }
}

// Trans. Date,Description,Amount,Category
#[derive(Debug, Deserialize)]
struct DiscoverRecord {
    #[serde(rename = "Trans. Date")]
    trans_date: String,

    #[serde(rename = "Description")]
    description: String,

    #[serde(rename = "Amount")]
    amount: String,

    #[serde(rename = "Category")]
    category: String,
}

// Transaction Date,Post Date,Description,Category,Type,Amount,Memo
#[derive(Debug, Deserialize)]
struct ChaseRecord {
    #[serde(rename = "Transaction Date")]
    trans_date: String,

    #[serde(rename = "Post Date")]
    post_date: String,

    #[serde(rename = "Description")]
    description: String,

    #[serde(rename = "Category")]
    category: String,

    #[serde(rename = "Type")]
    trans_type: String,

    #[serde(rename = "Amount")]
    amount: String,

    #[serde(rename = "Memo")]
    memo: String,
}

fn main() {
    draw_graph().unwrap();

    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let discover_file_path = "./test-data/disc_test_data.CSV";
    let chase_file_path = "./test-data/chase_test_data.csv";

    let mut disc_vec = Vec::new();
    let mut chase_vec = Vec::new();

    if let Ok(res) = new_read_csv_statement(discover_file_path, StatementSource::Discover) {
        disc_vec = res;
    } else {
        eprintln!(
            "Discover CSV File could not be parsed: {}",
            discover_file_path
        )
    }

    if let Ok(res2) = new_read_csv_statement(&chase_file_path, StatementSource::Chase) {
        chase_vec = res2;
    } else {
        eprintln!("Chase CSV File could not be parsed: {}", chase_file_path)
    }

    // println!("Discover Transactions: {:?}", disc_vec);
    // println!("Chase Transactions: {:?}", chase_vec);

    let mut total_amount: Decimal = Decimal::new(0, 0);

    let mut max: Decimal = Decimal::new(0, 0);
    let mut max_transaction: Transaction;



    for transaction in disc_vec {
        // println!("Discover Transaction: {:?}", transaction);
        println!("Amount: {}", transaction.amount);

        total_amount += transaction.amount;

        if transaction.amount > max {
            max = transaction.amount;
            max_transaction = transaction;
        } else {
            
        }


    }

    println!("Total Amount: ${}", total_amount);
    println!("Max Amount: ${}", max);
    println!("Max Transaction: {}", max_transaction);

    // ok() is a method that creates a Result with the Ok variant.
    Ok(())
}

fn draw_graph() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plotters-doc-data/1.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^3", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

fn new_read_csv_statement(
    filename: &str,
    source: StatementSource,
) -> Result<Vec<Transaction>, csv::Error> {
    
    let file: File = File::open(filename)?;
    let mut transactions: Vec<Transaction> = Vec::new();
    let mut reader: csv::Reader<File> = csv::Reader::from_reader(file);

    match source {
        StatementSource::Discover => {
            for result in reader.deserialize() {
                match result {
                    Ok(record) => {
                        let record: DiscoverRecord = record;

                        println!("Parsed Record: {:?}", record);

                        let transaction = Transaction {
                            date: NaiveDate::parse_from_str(&record.trans_date, "%Y-%m-%d")
                                .unwrap_or_default(),
                            description: record.description,
                            amount: record.amount.parse::<Decimal>().unwrap_or_default(),
                            category: record.category,
                            source: StatementSource::Discover,
                            post_date: None,
                            transaction_type: None,
                            memo: None,
                        };

                        transactions.push(transaction);
                    }

                    Err(err) => {
                        eprintln!("Error parsing Discover CSV: {}", err);
                    }
                }
            }
        }

        StatementSource::Chase => {
            for result in reader.deserialize() {
                match result {
                    Ok(record) => {
                        let record: ChaseRecord = record;

                        println!("Parsed Record: {:?}", record);

                        let transaction = Transaction {
                            date: NaiveDate::parse_from_str(&record.trans_date, "%m/%d/%Y")
                                .unwrap_or_default(),

                            description: record.description,

                            amount: record.amount.parse::<Decimal>().unwrap_or_default(),

                            category: record.category,

                            source: StatementSource::Chase,

                            post_date: Some(
                                NaiveDate::parse_from_str(&record.post_date, "%m/%d/%Y")
                                    .unwrap_or_default(),
                            ),

                            transaction_type: Some(record.trans_type),

                            memo: Some(record.memo),
                        };
                        transactions.push(transaction);
                    }

                    Err(err) => {
                        eprintln!("Error parsing Chase CSV: {}", err);
                    }
                }
            }
        }
    }

    println!(
        "Parsed {} transactions from {}",
        transactions.len(),
        filename
    );
    Ok(transactions)
}
