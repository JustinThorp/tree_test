// I think there is a slight difference in predictions since
// I use unqiue values for splits and sklearn takes the midpoint between two points
//Dont think it matters that much but may fix
//use rand::Rng;
use std::time::Instant;

use tree_test::{Data, Metric, Tree};

fn main() {
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path("data.csv")
        .unwrap();
    let mut df: Vec<Data> = Vec::new();
    for result in rdr.deserialize() {
        let record: Data = result.unwrap();
        df.push(record)
    }
    let now = Instant::now();
    let mut tree = Tree::new(Some(4), Some(400), Some(100), Some(Metric::MAE));
    tree.fit(&df);
    tree.traverse().unwrap();
    println!("{:?}", now.elapsed());
    // println!(
    //     "{}",
    //     tree.predict(&Data::new(vec![-0.508852, 0.633505, 1.511747], 1.440603))
    // );
    // let rss = df
    //     .iter()
    //     .map(|a| (tree.predict(a) - a.y).powi(2))
    //     .sum::<f64>()
    //     / df.len() as f64;
    // println!("{rss}");
    let mut rdr2 = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path("data_2.csv")
        .unwrap();
    let mut df2: Vec<Data> = Vec::new();
    for result in rdr2.deserialize() {
        let record: Data = result.unwrap();
        df2.push(record)
    }
    let rss2 = df2
        .iter()
        .map(|a| (tree.predict(a).unwrap() - a.y).powi(2))
        .sum::<f64>()
        / df2.len() as f64;

    println!("{}", rss2);
    println!("{:?}", tree.predict(&df2[0]))
    //println!("{}", median(&df[0..]))
}
