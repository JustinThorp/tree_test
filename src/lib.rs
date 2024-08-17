pub mod decision_tree_regressor;
pub mod linear_reg;
pub mod preprocessing;

#[derive(serde::Deserialize, Debug)]
pub struct Data {
    pub y: f64,
    pub x: Vec<f64>,
}
#[allow(dead_code)]
impl Data {
    fn new(x: Vec<f64>, y: f64) -> Data {
        Data { y, x }
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Metric {
    MSE,
    MAE,
}

pub trait Model {
    fn fit(&mut self, df: &Vec<Data>) -> ();
    fn predict(&self, x: &Data) -> Result<f64, String>;
}

pub fn rss(df: &Vec<Data>, fit: &impl Model) -> f64 {
    df.iter()
        .map(|x| (x.y - fit.predict(x).unwrap()).powi(2))
        .sum()
}

pub fn r2(df: &Vec<Data>, fit: &impl Model) -> f64 {
    let rss: f64 = df
        .iter()
        .map(|x| (x.y - fit.predict(x).unwrap()).powi(2))
        .sum();
    let mu: f64 = df.iter().map(|x| x.y).sum::<f64>() / df.len() as f64;
    let tss: f64 = df.iter().map(|x| (x.y - mu).powi(2)).sum::<f64>();
    1.0 - rss / tss
}
