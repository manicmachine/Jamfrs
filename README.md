# Jamfrs

## Purpose
Sometimes you just want to get some information from the JPS but the UI can be slow at times, especially when looking
for several records at a time. Or maybe you want a quick and easy way to export data from the JPS into another script.

Jamfrs is a command line tool for quickly querying the Jamf Pro server built using Rust. It can be used to lookup and 
export data either as XML or JSON. 


### Currently Jamfrs supports the following queries:
| Entity                  | Capabilities                                                                                 |
|-------------------------|----------------------------------------------------------------------------------------------|
| Computer                | - Delete (ID)<br/>- Show (ID)<br/>- Search* (ID, Name, UUID, Serial, MAC Address)<br/>- List |
| Mobile Devices          | - Delete (ID)<br/>- Show (ID)<br/>- Search* (ID, Name, UUID, Serial, MAC Address)<br/>- List |
| Users                   | - Delete<br/>- Show<br/>- Search* (ID, Name, UUID, Serial, MAC Address)<br/>- List           |
| Policy                  | - Delete<br/>- Show<br/>- List                                                               |
| Package                 | - Delete<br/>- Show<br/>- List                                                               |
| Category                | - Delete<br/>- Show<br/>- List                                                               |
| Department              | - Delete<br/>- Show<br/>- List                                                               |
| Ebook                   | - Delete<br/>- Show<br/>- List                                                               |
| Building                | - Delete<br/>- Show<br/>- List                                                               |
| Mac App                 | - Delete<br/>- Show<br/>- List                                                               |
| Mobile Device App       | - Delete<br/>- Show<br/>- List                                                               |
| Script                  | - Delete<br/>- Show<br/>- List                                                               |
| Groups (Smart & Static) | - Delete<br/>- Show<br/>- List                                                               |
| Advanced Search         | - Delete<br/>- Show<br/>- List                                                               |

**Supports wildcards*
 
## Usage
```
Usage: jamfrs [OPTIONS] --server <SERVER_ADDRESS> --user <USERNAME> --password <PASSWORD> <COMMAND>

Commands:
  computer    Work with computer records
  mobile      Work with mobile device records
  user        Work with users records
  policy      Work with policies
  package     Work with packages
  category    Work with categories
  department  Work with departments
  ebook       Work with ebooks
  building    Work with buildings
  mac-app     Work with mac applications
  mobile-app  Work with mobile device applications
  script      Work with scripts
  group       Work with smart & static groups
  adv-search  Work with advanced searches
  help        Print this message or the help of the given subcommand(s)

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

<details><summary>Extended Help</summary>

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

##### Command: policy
```
Work with policies

Usage: jamfrs --server <SERVER_ADDRESS> --user <USERNAME> --password <PASSWORD> policy <COMMAND>

Commands:
  delete  Delete an existing policy record
  show    Show an existing policy record
  list    List all policy records
```

##### Command: package
```
Work with policies

Usage: jamfrs --server <SERVER_ADDRESS> --user <USERNAME> --password <PASSWORD> policy <COMMAND>

Commands:
  delete  Delete an existing policy record
  show    Show an existing policy record
  list    List all policy records
```

##### Command: category
```
Work with categories

Usage: jamfrs --server <SERVER_ADDRESS> --user <USERNAME> --password <PASSWORD> category <COMMAND>

Commands:
  delete  Delete an existing category record
  show    Show an existing category record
  list    List all category records
```

##### Command: department
```
Work with departments

Usage: jamfrs --server <SERVER_ADDRESS> --user <USERNAME> --password <PASSWORD> department <COMMAND>

Commands:
  delete  Delete an existing department record
  show    Show an existing department record
  list    List all department records
```

##### Command: ebook
```
Work with ebooks

Usage: jamfrs --server <SERVER_ADDRESS> --user <USERNAME> --password <PASSWORD> ebook <COMMAND>

Commands:
  delete  Delete an existing ebook record
  show    Show an existing ebook record
  list    List all ebook records
```

##### Command: building
```
Work with buildings

Usage: jamfrs --server <SERVER_ADDRESS> --user <USERNAME> --password <PASSWORD> building <COMMAND>

Commands:
  delete  Delete an existing buildings record
  show    Show an existing buildings record
  list    List all buildings records
```

##### Command: mac-app
```
Work with mac applications

Usage: jamfrs --server <SERVER_ADDRESS> --user <USERNAME> --password <PASSWORD> mac-app <COMMAND>

Commands:
  delete  Delete an existing mac application record
  show    Show an existing mac application record
  list    List all mac application records
```

##### Command: mobile-app
```
Work with mobile device applications

Usage: jamfrs --server <SERVER_ADDRESS> --user <USERNAME> --password <PASSWORD> mobile-app <COMMAND>

Commands:
  delete  Delete an existing mobile device application record
  show    Show an existing mobile device application record
  list    List all mobile device application records
```

##### Command: script
```
Work with scripts

Usage: jamfrs --server <SERVER_ADDRESS> --user <USERNAME> --password <PASSWORD> script <COMMAND>

Commands:
  delete  Delete an existing script
  show    Show an existing script
  list    List all script
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

##### Command: adv-search
```
Work with advanced searches

Usage: jamfrs --server <SERVER_ADDRESS> --user <USERNAME> --password <PASSWORD> adv-search <COMMAND>

Commands:
  computer  Work with computer advanced searches
  mobile    Work with mobile device advanced searches
  user      Work with user advanced searches
```

##### Advanced Search Subcommand: computer
```
Work with computer advanced searches

Usage: jamfrs adv-search computer <COMMAND>

Commands:
  delete  Delete an existing computer advanced search
  show    Show an existing computer advanced search
  list    List all computer advanced searches
```

##### Advanced Search Subcommand: mobile
```
Work with mobile device advanced searches

Usage: jamfrs adv-search mobile <COMMAND>

Commands:
  delete  Delete an existing mobile device advanced search
  show    Show an existing mobile device advanced search
  list    List all mobile device advanced searches
```

##### Advanced Search Subcommand: user
```
Work with user advanced searches

Usage: jamfrs adv-search user <COMMAND>

Commands:
  delete  Delete an existing user advanced search
  show    Show an existing user advanced search
  list    List all user advanced searches
```
</details>

## Build
Being a Rust application, Jamfrs is built using `cargo` ([get cargo](https://www.rust-lang.org/learn/get-started)). To build, simply clone the repo, navigate to
the `jamfrs` directory containing the `Cargo.toml` file, and run `cargo build --release`. You'll find the newly created executable in `jamfrs/target/release`. 

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
