# Jamfrs

## Purpose
Jamfrs is a command line tool for querying the Jamf Pro server built using Rust.
It can be used to quickly export computer, mobile device, or user records to XML
or JSON. Beyond simple look-ups, it can be used to pipe along device information
to other services. 

## Usage
```
Usage: jamfrs [OPTIONS] --server <SERVER_ADDRESS> --user <USERNAME> --password <PASSWORD> <COMMAND>

Commands:
  computer  Work with computer records
  mobile    Work with mobile device records
  user      Work with users records
  help      Print this message or the help of the given subcommand(s)

Options:
  -s, --server <SERVER_ADDRESS>  Hostname or IP address for Jamf Pro server
      --port <PORT>              Port that the Jamf Pro server is listening to; Defaults to 443 for Jamf cloud instances, 8443 otherwise
  -u, --user <USERNAME>          Username used for API calls
  -p, --password <PASSWORD>      Password used by API user
      --pretty                   Pretty print output
      --json                     Request JSON data instead of the default XML
  -h, --help                     Print help information
  -V, --version                  Print version information
  ```

##### Command: computer
```
Work with computer records

Usage: jamfrs --server <SERVER_ADDRESS> --user <USERNAME> --password <PASSWORD> computer <COMMAND>

Commands:
  delete  Delete an existing computer record
  show    Show an existing computer record
  search  Search for existing computer records by name, MAC address, username, etc; Accepts * wildcard Note: Wildcard searches must be wrapped in quotes or escaped to prevent shell expansion
  list    List all computers
  ```
##### Command: mobile
```
Work with mobile device records

Usage: jamfrs --server <SERVER_ADDRESS> --user <USERNAME> --password <PASSWORD> mobile <COMMAND>

Commands:
  delete  Delete an existing mobile device record
  show    Show an existing mobile device record
  search  Search for existing mobile device records by name, MAC address, username, etc. Accepts * wildcard
  list    List all mobile device records
```

##### Command: user
```
Work with users records

Usage: jamfrs --server <SERVER_ADDRESS> --user <USERNAME> --password <PASSWORD> user <COMMAND>

Commands:
  delete  Delete an existing user record
  show    Show an existing user record
  list    List all user records
```

## Build
Being a Rust application, Jamfrs is built using `cargo`. To build, simply clone the repo, navigate to
the `jamfrs` directory containing the `Cargo.toml` file, and run `cargo build --release`.

## License
MIT License

Copyright (c) 2022 Corey Oliphant

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
