# Apple Products Parser

## Brief Description

**Apple Product Parser** is a simple Rust tool that processes detailed information from JSON about Apple products such as iPhones, MacBooks, and iPads. It extracts key details like product name, price, category, screen size, storage options, available colors, and year of release, and organizes them in a structured format(prints in console as yaml and toml).

The parsed data can be used for analysis, generating reports, or displaying in user interfaces. This project focuses on ensuring the data is accurately processed and made available for further use.

### Grammar for Input
- **Name**: The product's name (e.g., MacBook Pro 14-inch)
- **Price**: The price of the product in USD (e.g., 1999.99)
- **Type**: The type of product (e.g., Laptop)
- **Screen Size**: The display size (e.g., 14.2 inches)
- **Storage**: A list of available storage options (e.g., ["512GB", "1TB", "2TB"])
- **RAM**: The amount of RAM (e.g., 16GB)
- **Color**: Available color options (e.g., ["Silver", "Space Gray"])
- **Year of Release**: The year the product was released (e.g., 2023)
- **Availability**: Whether the product is available (true/false)

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

### Example Output(yaml)
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

