# Jamfrs

## Table of Contents
- [Purpose](#purpose)
- [Usage](#usage)
  - [Command: computer](#command-computer)
  - [Command: mobile](#command-mobile)
  - [Command: user](#command-user)
  - [Command: group](#command-group)
    - [Group Subcommand: computer](#group-subcommand-computer)
    - [Group Subcommand: mobile](#group-subcommand-mobile)
    - [Group Subcommand: user](#group-subcommand-user)
- [Build](#build)
- [License](#license)

## Purpose
Sometimes you just want to get some information from the JPS but the UI can be slow at times, especially when looking
for several records at a time. Or maybe you want a quick and easy way to export data from the JPS into another script.

Jamfrs is a command line tool for quickly querying the Jamf Pro server built using Rust. It can be used to lookup and 
export data either as XML or JSON. 


### Currently Jamfrs supports the following queries:
| Entity   | Capabilities                                                                                 |
|----------|----------------------------------------------------------------------------------------------|
| Computer | - Delete (ID)<br/>- Show (ID)<br/>- Search* (ID, Name, UUID, Serial, MAC Address)<br/>- List |
| Mobile Devices| - Delete (ID)<br/>- Show (ID)<br/>- Search* (ID, Name, UUID, Serial, MAC Address)<br/>- List |
| Users | - Delete<br/>- Show<br/>- Search* (ID, Name, UUID, Serial, MAC Address)<br/>- List                                                               |
| Groups (Smart & Static) | - Delete<br/>- Show<br/>- List                                                               |

**Supports wildcards*
 
## Usage
```
Usage: jamfrs [OPTIONS] --server <SERVER_ADDRESS> --user <USERNAME> --password <PASSWORD> <COMMAND>

Commands:
  computer  Work with computer records
  mobile    Work with mobile device records
  user      Work with users records
  group     Work with smart & static groups
  help      Print this message or the help of the given subcommand(s)

Options:
  -s, --server <SERVER_ADDRESS>  Hostname or IP address for Jamf Pro server
      --port <PORT>              Port that the Jamf Pro server is listening to; Defaults to 443 for Jamf cloud instances, 8443 otherwise
  -u, --user <USERNAME>          Username used for API calls
  -p, --password <PASSWORD>      Password used by API user
      --pretty                   Pretty print output
      --json                     Request JSON data instead of the default XML
      --insecure                 Allow insecure traffic; Defaults to False. Useful with HTTP or untrusted SSL certificates
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

##### Command: group
```
Work with smart & static groups

Usage: jamfrs --server <SERVER_ADDRESS> --user <USERNAME> --password <PASSWORD> group <COMMAND>

Commands:
  computer  Work with computer groups
  mobile    Work with mobile device groups
  user      Work with user groups
```
##### Group Subcommand: computer
```
Work with computer groups

Usage: jamfrs group computer <COMMAND>

Commands:
  delete  Delete an existing computer group
  show    Show an existing computer group
  list    List all computer groups
```

##### Group Subcommand: mobile
```
Work with mobile device groups

Usage: jamfrs group mobile <COMMAND>

Commands:
  delete  Delete an existing mobile device group
  show    Show an existing mobile device group
  list    List all mobile device groups
```

##### Group Subcommand: user
```
Work with user groups

Usage: jamfrs group user <COMMAND>

Commands:
  delete  Delete an existing user group
  show    Show an existing user group
  list    List all user groups
```

## Build
Being a Rust application, Jamfrs is built using `cargo` ([get cargo](https://www.rust-lang.org/learn/get-started)). To build, simply clone the repo, navigate to
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
