use csv;
use serde::Deserialize;
use stopwatch::Stopwatch;

#[derive(Debug, Deserialize, Clone)]
pub struct BAR {
    pub code: String,
    pub datetime: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

fn main() {
    let sw = Stopwatch::start_new();
    let mut rdr = csv::Reader::from_path("data/PTA.csv").unwrap();
    for result in rdr.deserialize() {
        let bar: BAR = result.unwrap();
        //println!("{:?}", bar);
    }
    println!("It took {0:.8} ms", sw.elapsed_ms());
}
