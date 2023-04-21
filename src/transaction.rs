// use crate::location::{Country, Continent};
// - Define Transaction struct with fields
// - transaction_id: unsigned 32bit integer
// - client_id: unsigned 32bit integer
// - asset_name: owned string
// - country: Country enum
// - continent: Continent enum
// - amount: float 64bit number
// - days_under_management: signed 64bit integer

use std::io::Split;
use crate::location::*;
use chrono::NaiveDate;

#[derive(Debug)]
pub struct Transaction {
    transaction_id: u32,
    client_id: u32,
    asset_name: String,
    country: Country,
    continent: Continent,
    amount: f64,
    days_under_management: i64,

}
 
// Implement a public function from_csv_line on Transaction type. The function should take in an input, line, of type string slice (1 row of data) and output a Result of type Transaction if Ok, String if Err
// a. create a variable fields by calling the split method using line with the delimiter ',' followed by the collect method to transform it to a Vec of string slices
// b. check if the length of fields is equals to 7, if it is not, return an Err with an appropriate message in String
// c. create a transaction_id variable by using the first item of fields to call the parse::<u32> method followed by an unwrap method
// d. create a client_id variable by using the second item of fields to call the parse::<u32> method followed by an unwrap method
// e. create a asset_name variable by using the third item of fields to call the to_uppercase method

// f. create a transaction_start_date variable by passing the fourth item of lines into NaiveDate::parse_from_str function followed by an unwrap method
// g. create a transaction_end_date variable by passing the fifth item of lines into NaiveDate::parse_from_str function followed by an unwrap method
// h. create a country variable by using the sixth item of fields to call the parse::<Country> method followed by the ? operator
// i. create a amount variable by using the seventh item of fields to call the parse::<f64> method followed by an unwrap method

// j. create a days_under_management variable by taking the result of subtracting transaction_start_date with transaction_end_date followed by calling num_days method
// k. create a continent variable by passing a reference of country into the country_to_continent functionNaiveDate::parse_from_str function followed by an unwrap method
// l. create a transaction variable by instantiating an instance of Transaction with the variables we have just created m. return an Ok which encapsulates the transaction variable we just created
 
 
 
 impl Transaction {
    pub fn from_csv_line(line: &str) -> Result<Transaction, String> {
        let fields: Vec<_> = line.split(',').collect();
        if fields.len() != 7 {
            return Err("Incorrect number of columns".to_owned())
        }

        let transaction_id = fields[0].parse::<u32>().unwrap();
        let client_id = fields[1].parse::<u32>().unwrap();
        let asset_name = fields[2].to_uppercase();
        let transaction_start_date = NaiveDate::parse_from_str(fields[3], "%Y-%m-%d").unwrap();
        let transaction_end_date = NaiveDate::parse_from_str(fields[4], "%Y-%m-%d").unwrap();
        let country = fields[5].parse::<Country>()?;
        let amount = fields[6].parse::<f64>().unwrap();
        let days_under_management = (transaction_end_date - transaction_start_date).num_days();

        let continent = country.country_to_continent();

        let transaction = Transaction{
            transaction_id: transaction_id,
            client_id: client_id,
            asset_name: asset_name,
            country: country,
            continent: continent,
            amount: amount,
            days_under_management: days_under_management,

        };

        Ok(transaction)


    }

 }
