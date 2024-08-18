use std::iter::Iterator;
use std::ops::Index;
pub mod decision_tree_regressor;
pub mod linear_reg;
pub mod preprocessing;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
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
    fn fit(&mut self, df: &DataFrame) -> ();
    fn predict(&self, x: &Data) -> Result<f64, String>;
}

pub fn rss(df: &DataFrame, fit: &impl Model) -> f64 {
    df.iter()
        .map(|x| (x.y - fit.predict(x).unwrap()).powi(2))
        .sum()
}

pub fn r2(df: &DataFrame, fit: &impl Model) -> f64 {
    let rss: f64 = df
        .iter()
        .map(|x| (x.y - fit.predict(x).unwrap()).powi(2))
        .sum();
    let mu: f64 = df.iter().map(|x| x.y).sum::<f64>() / df.len() as f64;
    let tss: f64 = df.iter().map(|x| (x.y - mu).powi(2)).sum::<f64>();
    1.0 - rss / tss
}
#[derive(Debug)]
pub struct DataFrame {
    rows: Vec<Data>,
}

impl DataFrame {
    //fn new() -> DataFrame<'a> {
    //    let
    //}

    pub fn from_csv(path: &str) -> Result<DataFrame, String> {
        let mut rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .from_path(path)
            .unwrap();
        let mut df: Vec<Data> = Vec::new();
        for result in rdr.deserialize() {
            let record: Data = result.unwrap();
            df.push(record)
        }
        let n = df[0].x.len();
        for i in 1..df.len() {
            if df[i].x.len() != n {
                return Err("Rows not Equal".to_string());
            }
        }

        Ok(DataFrame { rows: df })
    }
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn iter(&self) -> DataFrameIterator {
        DataFrameIterator {
            df: &self,
            index: 0,
        }
    }
    pub fn choose(&self, rng: &mut ThreadRng) -> Option<&Data> {
        self.rows.choose(rng)
    }
}

impl<'a> Index<usize> for DataFrame {
    type Output = Data;
    fn index(&self, i: usize) -> &Data {
        &self.rows[i]
    }
}

pub struct DataFrameIterator<'a> {
    df: &'a DataFrame,
    index: usize,
}

impl<'a> Iterator for DataFrameIterator<'a> {
    type Item = &'a Data;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.index;
        self.index += 1;
        match self.df.rows.get(current) {
            Some(x) => Some(x),
            None => None,
        }
    }
}
