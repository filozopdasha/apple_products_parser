# Apple Products Parser Documentation

## Overview

**Apple Product Parser** processes JSON data containing detailed information about Apple products such as iPhones, MacBooks, and iPads. It extracts key details including the product's name, price, category, screen size, storage options, available colors, and release date. The data is parsed and formatted as either TOML or YAML for easy consumption in reports, analysis, or user interface display.

This documentation provides details on the key components, including structs, enums, and the primary parsing logic used to process Apple product data.

### Rules
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

## Parser and Error Handling

### `Grammar`
The `Grammar` struct is responsible for defining the parser, which is tied to the `grammar.pest` file. It uses the `pest_derive::Parser` macro to automate parsing based on the defined grammar rules.

### `JsonParserError`
The `JsonParserError` struct is a custom error type used for handling JSON parsing errors. It utilizes the `thiserror` crate to simplify error creation and integrates with `pest` errors, providing clear error messages when parsing fails.
