use crate::Data;
use itertools::izip;

pub struct StandardScaler {
    pub means: Option<Vec<f64>>,
    pub sigma: Option<Vec<f64>>,
}

impl StandardScaler {
    pub fn new() -> StandardScaler {
        StandardScaler {
            means: None,
            sigma: None,
        }
    }

    pub fn fit(&mut self, df: &Vec<Data>) {
        self.means = Some(Vec::new());
        for _ in 0..df[0].x.len() {
            self.means.as_mut().unwrap().push(0.0);
        }
        for row in df {
            for i in 0..row.x.len() {
                self.means.as_mut().unwrap()[i] += row.x[i];
            }
        }
        for mu in self.means.as_mut().unwrap() {
            *mu *= 1.0 / df.len() as f64;
        }
        // var
        self.sigma = Some(Vec::new());
        for _ in 0..df[0].x.len() {
            self.sigma.as_mut().unwrap().push(0.0);
        }
        for row in df {
            for i in 0..row.x.len() {
                self.sigma.as_mut().unwrap()[i] +=
                    (row.x[i] - self.means.as_ref().unwrap()[i]).powi(2);
            }
        }
        for s in self.sigma.as_mut().unwrap() {
            *s *= 1.0 / (df.len() as f64);
            *s = s.sqrt();
        }
    }
    pub fn transform(&self, x: &Data) -> Result<Data, String> {
        let mut x_transformed: Vec<f64> = Vec::new();
        match (&self.means, &self.sigma) {
            (Some(mean), Some(sigma)) => {
                for (a, m, s) in izip!(&x.x, mean, sigma) {
                    x_transformed.push((a - m) / s);
                }
                Ok(Data::new(x_transformed, x.y))
            }
            _ => Err("Unit Trans".to_string()),
        }
    }
}
