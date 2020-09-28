# geocode-csv

Geocode addresses in a CSV file (i.e., add latitude and longitue) with locationiq

Command line utitliy to geocode addresses in a CSV File.  You specify the address elements (steet, city, etc.)
by giving either the column number or the name of the column in the CSV header.  The specified columns for
latitude and logitude will be filled-in in the resulting output file (note that the columns must already
be present in the input CSV file).

You will need a key from https://locationiq.com to run this.

```
USAGE:
    geocode-csv --city <city> --country <country> --input <input> --key <key> --lat <lat> --lng <lng> --output <output> --postalcode <postalcode> --street <street>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>              File name for the input CSV file
    -o, --output <output>            File name for the output CSV file
    -k, --key <key>                  Your key for accessing locationiq.com
    -s, --street <street>
            Street input column (0-indexed or header name)

    -p, --postalcode <postalcode>
            Postalcode input column (0-indexed or header name)

    -c, --city <city>
            City input column (0-indexed or header name)

    -y, --country <country>
            Country input column (0-indexed or header name)

    -t, --lat <lat>
            Latitude output column (0-indexed or header name)

    -g, --lng <lng>
            Longitude output column (0-indexed or header name)
```   
