use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;
use serde::Serialize;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic", set_term_width(80))]
struct CliOptions {
    /// File name for the input CSV file
    #[structopt(short, long, parse(from_os_str), display_order(10))]
    input: PathBuf,

    /// File name for the output CSV file
    #[structopt(short, long, parse(from_os_str), display_order(11))]
    output: PathBuf,

    /// Your key for accessing locationiq.com
    #[structopt(short, long, display_order(20))]
    key: String,

    /// Street input column (0-indexed or header name)
    ///
    /// Street (input) column in the CSV file, specified either as 0-indexed number or as the text of the column header
    #[structopt(short, long, display_order(50))]
    street: String,
    /// Postalcode input column (0-indexed or header name)
    ///
    /// Postal code (input) column in the CSV file, specified either as 0-indexed number or as the text of the column header
    #[structopt(short, long, display_order(51))]
    postalcode: String,
    /// City input column (0-indexed or header name)
    ///
    /// City (input) column in the CSV file, specified either as 0-indexed number or as the text of the column header
    #[structopt(short, long, display_order(52))]
    city: String,
    /// Country input column (0-indexed or header name)
    ///
    /// Country (input) column in the CSV file, specified either as 0-indexed number or as the text of the column header
    #[structopt(short = "y", long, display_order(53))]
    country: String,

    /// Latitude output column (0-indexed or header name)
    /// 
    /// Latitude output column in the CSV file, specified either as 0-indexed number or as the text of the column header.
    /// Note that this column must already be present in the input CSV file (can be empty) and that any contents
    /// in the corresponding fields will be overwritten
    #[structopt(short = "t", long, display_order(70))]
    lat: String,
    /// Longitude output column (0-indexed or header name)
    /// 
    /// Longitude output column in the CSV file, specified either as 0-indexed number or as the text of the column header.
    /// Note that this column must already be present in the input CSV file (can be empty) and that any contents
    /// in the corresponding fields will be overwritten
    #[structopt(short = "g", long, display_order(71))]
    lng: String,

    // TODO:
    // - make wait between two requests to locationiq.com configurable (currently hardcoded to 2sec)
    // - give possibility to add the output columns rather than having them already be present in the input file
}

#[derive(Debug)]
struct Columns {
    street: usize,
    postalcode: usize,
    city: usize,
    country: usize,
    lat: usize,
    lng: usize,
}

impl Columns {
    fn from_opts_and_header(
        opts: &CliOptions,
        headers: &csv::StringRecord,
    ) -> Result<Self, Box<dyn Error>> {
        let street = opts.street.parse::<usize>().or( headers
            .iter()
            .position(|s| s == opts.street)
            .ok_or("Header columns for street not found"))?;
        let postalcode = opts.postalcode.parse::<usize>().or( headers
            .iter()
            .position(|s| s == opts.postalcode)
            .ok_or("Header columns for postalcode not found"))?;
        let city = opts.city.parse::<usize>().or( headers
            .iter()
            .position(|s| s == opts.city)
            .ok_or("Header columns for city not found"))?;
        let country = opts.country.parse::<usize>().or( headers
            .iter()
            .position(|s| s == opts.country)
            .ok_or("Header columns for country not found"))?;
        let lat = headers
            .iter()
            .position(|s| s == opts.lat)
            .ok_or("Header columns for lat not found")?;
        let lng = headers
            .iter()
            .position(|s| s == opts.lng)
            .ok_or("Header columns for lng not found")?;    

        Ok(Self { street, postalcode, city, country, lat, lng })
    }
}

#[derive(Serialize, Debug)]
struct SearchQuery {
    key: String,
    street: String,
    postalcode: String,
    city: String,
    country: String,
    format: String
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = CliOptions::from_args();

    //let input_file = File::open(&opts.input)?;
    //let mut csv_reader = csv::Reader::from_reader(BufReader::new(input_file));
    let mut csv_reader = csv::Reader::from_path(&opts.input)?;
    let headers = csv_reader.headers()?;

    let columns = Columns::from_opts_and_header(&opts, headers)?;

    //println!("{:?}", &columns);
    
    let mut csv_writer = csv::WriterBuilder::new()
        .from_path(&opts.output)?;
        
    csv_writer.write_record(headers)?;
    
    let http_client = reqwest::blocking::Client::new();
    for result in csv_reader.records() {
        let record = result?;
        
        std::thread::sleep(std::time::Duration::from_millis(2000));
        println!("Processing line {}", &record.position().unwrap().line());

        let search_query = SearchQuery {
            key: opts.key.clone(),
            street: record.get(columns.street).unwrap_or("").to_string(),
            postalcode: record.get(columns.postalcode).unwrap_or("").to_string(),
            city: record.get(columns.city).unwrap_or("").to_string(),
            country: record.get(columns.country).unwrap_or("").to_string(),
            format: String::from("json")
        };

        let url = "https://eu1.locationiq.com/v1/search.php";
        let res = http_client.get(url).query(&search_query).send()?;
        let text = res.text()?;
        let result : serde_json::Value = serde_json::from_str( &text )?;
        let lat = result[0]["lat"].as_str().unwrap_or("");
        let lng = result[0]["lon"].as_str().unwrap_or("");

        if lat.is_empty() || lng.is_empty() {
            println!("No result for: {:?}", search_query);
            println!("{}", &text);
        }

        let mut record_out = csv::StringRecord::new();
        for i in 0..record.len() {
            if i == columns.lat {
                record_out.push_field(&format!("{}", &lat));
            }
            else if i == columns.lng {
                record_out.push_field(&format!("{}", &lng));
            }
            else {
                record_out.push_field(record.get(i).unwrap_or(""));
            }
        }
        csv_writer.write_record(&record_out)?;
        csv_writer.flush()?;
    }

    csv_writer.flush()?;

    Ok(())
}
