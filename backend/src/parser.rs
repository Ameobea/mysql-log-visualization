//! Contains functions for parsing the data form the MySQL text log.

use std::convert::TryFrom;

use chrono::NaiveDateTime;
use regex::{Captures, Match, Regex};

use util::debug;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum LineType {
    Connect,
    Query,
    Quit,
}

impl<'a> TryFrom<Match<'a>> for LineType {
    type Error = String;

    fn try_from(m: Match<'a>) -> Result<Self, Self::Error> {
        let m_str = m.as_str();

        match m_str {
            "Connect" => Ok(LineType::Connect),
            "Query" => Ok(LineType::Query),
            "Quit" => Ok(LineType::Quit),
            _ => Err(format!("Unable to parse line type of type: {}", m_str)),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogLine {
    date: NaiveDateTime,
    event_type: LineType,
    query_type: Option<QueryType>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum QueryType {
    Insert,
    Select,
    Update,
    Delete,
    Other,
    Transaction,
    Setting,
}

lazy_static! {
    // Regex for parsing lines of the MySQL log.
    static ref LINE_PARSER_REGEX: Regex = Regex::new("(\\d{4}\\-\\d{2}\\-\\d{2}T\\d{2}:\\d{2}:\\d{2}\\.\\d{6}Z)\\s*\\d*\\s*(\\w*)\\s*(.*)")
        .expect("Unable to create the line parser regex!");
}

/// Converts a SQL query string into our internal representation of queries, representing its type.
fn parse_query(query_str: &str) -> Option<QueryType> {
    let words: Vec<&str> = query_str.split_whitespace().collect();
    if words.len() > 0 {
        match words[0].to_uppercase().as_str() {
            "INSERT" => Some(QueryType::Insert),
            "SELECT" => Some(QueryType::Select),
            "UPDATE" => Some(QueryType::Update),
            "DELETE" => Some(QueryType::Delete),
            "START" | "COMMIT" => Some(QueryType::Transaction),
            "SET" | "SHOW" => Some(QueryType::Setting),
            _ => None,
        }
    } else {
        None
    }
}

pub fn parse_line(line: &str) -> Result<LogLine, String> {
    let caps: Captures = LINE_PARSER_REGEX.captures(line)
        .ok_or(format!("Getting captures on line failed: {}", line))?;

    // Dates are in the format 2017-07-29T18:27:33.562444Z
    let cap1 = caps.get(1).ok_or(String::from("No capture group 0 found for log line!"))?;
    let cap2 = caps.get(2).ok_or(String::from("No capture group 1 found for log line!"))?;
    let cap3 = caps.get(3).ok_or(String::from("No capture group 1 found for log line!"))?;

    let date = NaiveDateTime::parse_from_str(cap1.as_str(), "%Y-%m-%dT%H:%M:%S%.6fZ")
        .map_err(debug)?;
    let event_type = LineType::try_from(cap2)?;
    let query_type = parse_query(cap3.as_str());

    Ok(LogLine { date, event_type, query_type })
}


