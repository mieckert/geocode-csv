use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;
use serde::Serialize;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct CliOptions {
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,

    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

    #[structopt(short, long)]
    key: String,

    #[structopt(short, long)]
    street: String,
    #[structopt(short, long)]
    postalcode: String,
    #[structopt(short, long)]
    city: String,
    #[structopt(short = "y", long)]
    country: String,

    #[structopt(short = "t", long)]
    lat: String,
    #[structopt(short = "g", long)]
    lng: String,
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
        let country = opts.postalcode.parse::<usize>().or( headers
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
