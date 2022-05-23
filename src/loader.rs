use csv;

pub trait FromRecord {
    fn from_record(record: &csv::StringRecord) -> Self;
}

pub fn from_csv<T: FromRecord>(data: &str) -> Vec<T> {
    let mut rdr = csv::Reader::from_reader(data.as_bytes());

    let mut vec = Vec::new();
    for result in rdr.records() {
        let record = result.unwrap();
        vec.push(T::from_record(&record));
    }
    return vec;
}
