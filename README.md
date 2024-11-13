# Apple Products Parser

## Links
https://crates.io/crates/apple_products_parser

https://docs.rs/apple_products_parser/0.1.1/apple_products_parser/

## Brief Description

**Apple Product Parser** is a simple Rust tool that processes detailed information from JSON about Apple products such as iPhones, MacBooks, and iPads. It extracts key details like product name, price, category, screen size, storage options, available colors, and year of release, and organizes them in a structured format(prints in console as yaml and toml).

The parsed data can be used for analysis, generating reports, or displaying in user interfaces. This project focuses on ensuring the data is accurately processed and made available for further use.

### Grammar Definition
- **ALPHABETIC**: 
{ ('a'..'z' | 'A'..'Z') }
Any letter, uppercase or lowercase
- **Name**:
{ (ALPHABETIC | ASCII_DIGIT | " ")+ }
The product name can consist of letters, digits, or spaces.
- **Price**:
{ ASCII_DIGIT+ ~ "." ~ ASCII_DIGIT{2} }
The price should be digits, followed by a decimal point and two digits for cents.
- **Date of Release**
{ day ~ "-" ~ month ~ "-" ~ year }
Should be in the format dd-mm-yyyy.
- **Day** 
{ ("0" ~ ('1'..'9')) | ("1" ~ ('0'..'9')) | ("2" ~ ('0'..'9')) | ("3" ~ ('0'..'1')) }
A valid day between "01" and "31".
- **Month**: 
{ ("0" ~ ('1'..'9')) | ("1" ~ ('0'..'2')) }
A valid month between "01" and "12".
- **Year**:
 { ASCII_DIGIT{4} }
A four-digit number (e.g., 2022).
- **Type**: 
{ "Smartphone" | "Laptop" | "Tablet" | "Earphones" | "Watch" | "Other" }
The product type can be one of the predefined options.
- **Screen Size**:
{ ASCII_DIGIT+ ~ "." ~ ASCII_DIGIT+ ~ " inches" | "N/A" }
Screen size represented as digits followed by "inches" (e.g., "14.2 inches") or it can be "N/A".
- **Storage**: 
{ (ASCII_DIGIT+ ~ "GB") | (ASCII_DIGIT+ ~ "TB") }
Storage capacity in GB or TB (e.g., "512GB" or "1TB").
- **RAM**: 
{ (ASCII_DIGIT+ ~ "GB") | (ASCII_DIGIT+ ~ "TB") }
RAM size specified in GB or TB (e.g., "16GB" or "1TB").
- **Color**: 
{ (ALPHABETIC | " ")+ }
Colors are represented by letters and spaces (e.g., "Silver" or "Space Gray").
- **Availability**: 
{ "true" | "false" }
Availability is a boolean value, either "true" or "false".

## Usage of Parsed Results
- Data Analysis: You can compare prices, storage sizes, screen sizes, and other features of different Apple products
- Reports: The parsed data can be used to create reports or dashboards that show information about product availability, trends in product features, and pricing
- User Interface Display: The data can be shown on screens where users can filter and sort Apple products by things like price, release year, and screen size


### Example Input
    {
      "name": "iPhone 15 Pro",
      "price": 999.99,
      "type": "Smartphone",
      "screen_size": "6.1 inches",
      "storage": ["128GB", "256GB", "512GB"],
      "color": ["Silver", "Graphite", "Gold", "Pacific Blue"],
      "date_of_release": "12-09-2023"
      "availability": true
    },
    {
      "name": "MacBook Pro 14",
      "price": 1999.99,
      "type": "Laptop",
      "screen_size": "14.2 inches",
      "storage": ["512GB", "1TB", "2TB"],
      "ram": "16GB",
      "color": ["Silver", "Space Gray"],
      "date_of_release": "24-10-2023"
      "availability": true
    }


### Example Output(toml)
```toml
[[]]
name = "iPhone 15 Pro"
price = 999.99
type = "Smartphone"
screen_size = "6.1 inches"
storage = ["128GB", "256GB", "512GB"]
color = ["Silver", "Graphite", "Gold", "Pacific Blue"]
date_of_release = "12-09-2023"
availability = true

[[]]
name = "MacBook Pro 14"
price = 1999.99
type = "Laptop"
screen_size = "14.2 inches"
storage = ["512GB", "1TB", "2TB"]
ram = "16GB"
color = ["Silver", "Space Gray"]
date_of_release = "24-10-2023"
availability = true
```

### Example Output(yaml)
```
- name: iPhone 15 Pro
  price: 999.99
  type: Smartphone
  screen_size: 6.1 inches
  storage:
    - 128GB
    - 256GB
    - 512GB
  color:
    - Silver
    - Graphite
    - Gold
    - Pacific Blue
  date_of_release: 12-09-2023
  availability: true
- name: MacBook Pro 14
  price: 1999.99
  type: Laptop
  screen_size: 14.2 inches
  storage:
    - 512GB
    - 1TB
    - 2TB
  ram: 16GB
  color:
    - Silver
    - Space Gray
  date_of_release: 24-10-2023
  availability: true
```
