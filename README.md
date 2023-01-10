# Overview
Rust program to stream [Discogs](https://discogs.com/) [monthly dump data](https://discogs-data-dumps.s3.us-west-2.amazonaws.com/index.html) XML data into the buffer, with the eventual goal of creating an efficient library for streaming gzip xml files into database tables.

## Why this might be useful 
Having the ability to stream in bytes of data from a gz file directly into a clearable buffer to load into an output (database, parquet files, etc.) will allow for systems with less memory than an XML file size to be able to do a load -- In this case:
```
2022-12-05T16:48:59.000Z        10.8 GB        discogs_20221201_releases.xml.gz
```
on a Macbook Air M1 with 8GB Ram.

## Further Plans
* Read data into a streamed buffer to parse XML of each chunk of data downloaded 
* Functions for cleaning data
* Measuring performance 
