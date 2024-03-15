# This crate is still in development 
# [crate link](https://crates.io/crates/EasyAlgolia)

# EasyAlgolia is a Rust crate designed for utilizing the Algolia admin client. It simplifies the process of updating and inserting documents into Algolia's search index.

![Alt text](https://upload.wikimedia.org/wikipedia/commons/thumb/d/da/Algolia_logo.svg/1200px-Algolia_logo.svg.png "a title")

# This crate is still in development 

###  Usage 
```rust
   use EasyAlgolia::client_builder::ClientBuilder ;
   fn main()<(),std:io::Error>{
       let client = ClientBuilder::build_from_env()?;
       client.upload()?;
   }
  ```