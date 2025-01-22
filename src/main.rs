// makes the csv crate accessible to your program
extern crate csv;

use chrono::{Datelike, NaiveDate};
use plotters::prelude::*;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::process;
use std::thread;
use std::time::{Duration, Instant, SystemTime};

#[derive(Debug, PartialEq)]
enum StatementSource {
    Discover,
    Chase,
}

#[derive(Debug)]
struct Transaction {
    amount: Decimal,

    category: String,

    date: NaiveDate,

    description: String,

    memo: Option<String>,

    // Optional fields for additional Chase data
    post_date: Option<NaiveDate>,

    source: StatementSource,

    transaction_type: Option<String>,
}

impl std::fmt::Display for Transaction {
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
#[derive(Debug, Deserialize, PartialEq)]
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

    if let Err(err) = process_csv_sheets() {
        println!("{}", err);
        process::exit(1);
    }
}

fn process_csv_sheets() -> Result<(), Box<dyn Error>> {
    let discover_file_path = "./test-data/disc_test_data.CSV";
    let chase_file_path = "./test-data/chase_test_data.csv";

    let mut disc_vec = Vec::new();
    let mut chase_vec = Vec::new();

    // TEST BEGIN =====>

    let now = Instant::now();

    thread::spawn(|| new_read_csv_statement(discover_file_path, StatementSource::Discover));

    thread::spawn(|| new_read_csv_statement(chase_file_path, StatementSource::Chase));


    let elapsed = now.elapsed();

    println!("Elapsed: {:.2?}", elapsed);

    // TEST END =====>

    let now = Instant::now();

    if let Ok(res) = new_read_csv_statement(discover_file_path, StatementSource::Discover) {
        disc_vec = res;
    } else {
        eprintln!(
            "Discover CSV File could not be parsed: {}",
            discover_file_path
        )
    }

    if let Ok(res2) = new_read_csv_statement(chase_file_path, StatementSource::Chase) {
        chase_vec = res2;
    } else {
        eprintln!(
            "Chase CSV File could not be parsed: {}", 
            chase_file_path
        )
    }

    let elapsed = now.elapsed();

    println!("Elapsed: {:.2?}", elapsed);

    // TEST END =====>


    println!(
        "Discover Total Amount: ${}",
        calculate_total_amount(&disc_vec)
    );
    println!(
        "Chase Total Amount: ${}",
        calculate_total_amount(&chase_vec)
    );

    let pair = calculate_max_amount(&disc_vec);
    let pair2 = calculate_max_amount(&chase_vec);

    println!(
        "Discover Max Amount: ${}, for transaction: {}",
        pair.0, pair.1
    );
    println!(
        "Chase Max Amount: ${}, for transaction: {}",
        pair2.0, pair2.1
    );

    let disc_hm: HashMap<(i32, i32), i32> = calculate_monthly_spending(&disc_vec);
    let chase_hm: HashMap<(i32, i32), i32> = calculate_monthly_spending(&chase_vec);

    println!("\n");

    for (key, value) in disc_hm {
        println!("Discover | Month: {} | Amount: {}", key, value);
    }

    for (key, value) in chase_hm {
        println!("Chase | Month: {} | Amount: {}", key, value);
    }

    let years: [i32; 3] = [2022, 2023, 2024];

    for i in years {

        for j in 1..13 {

            let disc_val = disc_hm.get(&(i,j)).copied().unwrap_or(0);
            let chase_val = chase_hm.get(&(i,j)).copied().unwrap_or(0);

            if disc_val != 0 && chase_val != 0{
                println!("Discover: Year {} Month: {} | Amount ${}", i, j, disc_val);
                println!("Chase: Year {} Month: {} | Amount ${}", i, j, chase_val);

                println!("\n");
            }

        }
    }

    Ok(())
}

fn calculate_total_amount(transactions: &Vec<Transaction>) -> Decimal {
    let mut total_amount: Decimal = Decimal::new(0, 0);

    for transaction in transactions {
        total_amount += transaction.amount;
    }

    total_amount
}

fn calculate_max_amount(transactions: &Vec<Transaction>) -> (Decimal, &Transaction) {
    let mut max: Decimal = Decimal::new(0, 0);

    let mut max_transaction: &Transaction = &transactions[0];

    for transaction in transactions {
        if transaction.amount > max {
            max = transaction.amount;
            max_transaction = transaction;
        }
    }

    (max, max_transaction)
}

// TODO: We are calculating the total of the month for all of the years instead of just doing each year monthly spending
// Take into account the year instead


fn calculate_monthly_spending(transactions: &Vec<Transaction>) -> HashMap<(i32, i32), i32> {
    let mut monthly_spending: HashMap<(i32, i32), i32> = HashMap::new();

    for transaction in transactions {
        let month = transaction.date.month() as i32;
        let year = transaction.date.year() as i32;

        let new_key = (year, month);

        // Get the current total for the month, defaulting to 0 if not present
        let current_total = monthly_spending.get(&new_key).copied().unwrap_or(0);

        // Calculate the new total by adding the transaction amount
        let new_total = add_option_and_decimal(Some(&current_total), transaction.amount);

        // Insert the updated total back into the HashMap
        monthly_spending.insert(new_key, new_total);
    }

    monthly_spending
}

fn add_option_and_decimal(opt: Option<&i32>, dec: Decimal) -> i32 {
    // Handle the Option to get the i32 value or default to 0
    let opt_value = opt.unwrap_or(&0);

    // Convert the Decimal to i32, handling possible conversion issues
    // Here we assume truncating the Decimal to an i32 is acceptable
    let dec_value = dec.to_i32().unwrap_or(0);

    // Perform the addition
    opt_value + dec_value
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

                        let transaction = Transaction {
                            date: NaiveDate::parse_from_str(&record.trans_date, "%m/%d/%Y")
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

                        let replaced_amount = record.amount.replace("-", "");

                        let transaction = Transaction {
                            date: NaiveDate::parse_from_str(&record.trans_date, "%m/%d/%Y")
                                .unwrap_or_default(),

                            description: record.description,

                            amount: replaced_amount.parse::<Decimal>().unwrap_or_default(),

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
