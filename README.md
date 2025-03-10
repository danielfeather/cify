# Cify

A creatively named Rust crate (I know right), implementing data types found in the [Common Interface File (CIF) format](https://wiki.openraildata.com/index.php/CIF_File_Format) used to exchange train schedule information between different systems in the UK.

This create is being implemented using the "**RSPS5046 Timetable Information Data Feed Interface Specification**", published by the [Rail Delivery Group](https://www.raildeliverygroup.com/).

The RSPS5046 specification can be downloaded from https://www.rspaccreditation.org/publicDocumentation.php

I am using the data provided from the [SCHEDULE](https://wiki.openraildata.com/index.php?title=SCHEDULE) data feed to test this crates functionality. The feed provides information on train timetables in the UK, and is available to the public via registration at https://publicdatafeeds.networkrail.co.uk

I'm using this opportunity to learn [Serde's](https://docs.rs/serde/) to very much a hobby project.

Resources:

https://wiki.openraildata.com

https://www.networkrail.co.uk/who-we-are/transparency-and-ethics/transparency/open-data-feeds/
