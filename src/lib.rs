pub mod DecisionTreeRegressor;
pub mod linear_reg;
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
