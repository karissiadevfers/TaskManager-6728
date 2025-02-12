Враховуючи, є невеликий приклад програми на Rust, яка здійснює базову обробку даних. Програма читає CSV-файл, проводить базову обробку даних і записує результати в новий CSV-файл.

```rust
extern crate csv;
extern crate serde;

use std::error::Error;
use std::fs::File;
use std::process;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize)]
struct Record {
    field1: String,
    field2: String,
    field3: String,
}

#[derive(Debug, Serialize)]
struct ProcessedRecord {
    field1: String,
    field3: String,
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut wtr = csv::Writer::from_path("output.csv")?;

    for result in rdr.deserialize() {
        let record: Record = result?;
        let processed_record = process_record(record);
        wtr.serialize(processed_record)?;
    }

    wtr.flush()?;

    Ok(())
}

fn process_record(record: Record) -> ProcessedRecord {
    let field1 = record.field1;
    let field3 = record.field3;

    let processed_record = ProcessedRecord {
        field1: field1,
        field3: field3,
    };

    processed_record
}
```

Цей код має менше 150 рядків, але він повноцінно демонструє базову обробку даних в Rust. Ця програма читає CSV-файл, обробляє кожен запис (в цьому випадку, просто видаляє друге поле) і записує оброблені записи в новий CSV-файл.

Код може бути розширений за потреби, наприклад, додаванням більш складної обробки записів або підтримкою більшої кількості форматів файлів.