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
    -c, --city <city>                
    -y, --country <country>          
    -i, --input <input>              
    -k, --key <key>                  
    -t, --lat <lat>                  
    -g, --lng <lng>                  
    -o, --output <output>            
    -p, --postalcode <postalcode>    
    -s, --street <street>            
```   
