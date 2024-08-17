use rand::seq::SliceRandom;
use std::iter::zip;

#[allow(unused_imports)]
use crate::Data;
use crate::Model;
#[derive(Debug)]
pub struct LinearReg {
    coef: Option<Vec<f64>>,
    intercept: Option<f64>,
}
#[allow(dead_code)]
impl LinearReg {
    pub fn new() -> LinearReg {
        LinearReg {
            coef: None,
            intercept: None,
        }
    }
}

impl Model for LinearReg {
    fn fit(&mut self, df: &Vec<Data>) {
        //let l: f64 = 0.00001;
        self.coef = Some(Vec::new());
        self.intercept = Some(0.0);
        for _ in 0..df[0].x.len() {
            self.coef.as_mut().unwrap().push(0.0)
        }

        for i in 1..df.len() {
            let l = 0.01 / (i as f64).powf(0.25);
            let obs = df.choose(&mut rand::thread_rng()).unwrap();
            let pred = self.predict(obs).unwrap();
            for (i, coeff) in self.coef.as_mut().unwrap().iter_mut().enumerate() {
                let d = l * -2.0 * obs.x[i] * (obs.y - pred);
                *coeff = *coeff - d;
            }
            self.intercept = Some(self.intercept.unwrap() - l * -2.0 * (obs.y - pred));
        }
        ()
    }

    fn predict(&self, x: &Data) -> Result<f64, String> {
        match &self.coef {
            Some(_) => {
                let mut pred: f64 = self.intercept.unwrap();
                for (coeff, val) in zip(self.coef.as_ref().unwrap(), x.x.clone()) {
                    pred += coeff * val;
                }
                Ok(pred)
            }
            None => Err("Uninitialized Model".to_string()),
        }
    }
}
