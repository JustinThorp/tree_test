//use rand::Rng;
//use std::time::Instant;

use tree_test::*;

fn main() {
    //let mut rdr = csv::ReaderBuilder::new()
    //    .flexible(true)
    //    .from_path("data.csv")
    //    .unwrap();
    //let mut df: Vec<Data> = Vec::new();
    //for result in rdr.deserialize() {
    //    let record: Data = result.unwrap();
    //    df.push(record)
    //}
    //let mut temp: Vec<Data> = Vec::new();
    let df = DataFrame::from_csv("data.csv").unwrap();
    //println!("{:?}", df2);
    //let temp: Vec<f64> = df2.iter().map(|x| x.y).collect();
    //println!("{:?}", temp);
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
    let mut ss = preprocessing::StandardScaler::new();
    ss.fit(&df);
    println!("{:?}", &ss.means.as_ref().unwrap());
    println!("{:?}", &ss.sigma.as_ref().unwrap());
    println!("{:?}", &df[0]);
    println!("{:?}", ss.transform(&df[0]).unwrap());
}
