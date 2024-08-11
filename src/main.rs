//use rand::Rng;
//use std::time::Instant;

use tree_test::*;

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
    let mut tree = decision_tree_regressor::DecisionTreeRegressor::new(
        Some(4000),
        Some(2),
        Some(1),
        Some(Metric::MSE),
    );
    tree.fit(&df);
    //tree.traverse().unwrap();
    //tree.predict();

    let mut lr = linear_reg::LinearReg::new();
    lr.fit(&df);
    //println!("{:?}", lr);
    println!("Linear Regresion: {}", r2(&df, &lr));
    println!("Decision Tree: {}", r2(&df, &tree));
}
