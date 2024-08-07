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

fn quickselect(l: &Vec<f64>, k: usize) -> f64 {
    if l.len() == 1 {
        return l[0];
    }
    let pivot = l[0];
    let lows: Vec<f64> = l.into_iter().map(|a| *a).filter(|&a| a < pivot).collect();
    let highs: Vec<f64> = l.into_iter().map(|a| *a).filter(|&a| a > pivot).collect();
    let pivots: Vec<f64> = l.into_iter().map(|a| *a).filter(|&a| a == pivot).collect();
    if k < lows.len() {
        return quickselect(&lows, k);
    } else if k < (lows.len() + pivots.len()) {
        return pivots[0];
    } else {
        return quickselect(&highs, k - lows.len() - pivots.len());
    }
}

fn median(l: &[&Data]) -> f64 {
    let vals: Vec<f64> = l.into_iter().map(|a| a.y).collect();
    if l.len() % 2 == 1 {
        return quickselect(&vals, l.len() / 2);
    } else {
        return 0.5 * quickselect(&vals, l.len() / 2 - 1) + 0.5 * quickselect(&vals, l.len() / 2);
    }
}

#[derive(Debug)]
struct Node {
    id: i64,
    depth: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    val: Option<f64>,
    var: Option<usize>,
    split: Option<f64>,
    max_depth: usize,
    min_samples_split: usize,
    min_samples_leaf: usize,
    metric: Metric,
}
impl Node {
    fn new(
        id: i64,
        depth: usize,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
        max_depth: usize,
        min_samples_split: usize,
        min_samples_leaf: usize,
        metric: Metric,
    ) -> Node {
        Node {
            id,
            depth,
            left,
            right,
            val: None,
            var: None,
            split: None,
            max_depth,
            min_samples_split,
            min_samples_leaf,
            metric,
        }
    }

    fn traverse(&self) {
        println!(
            "{:width$}, value : {:.3},split: {}",
            self.id,
            self.val.expect("This Should Always have a value"),
            self.split.unwrap_or(-1.0),
            width = (self.depth * 2) as usize
        );

        match &self.left {
            Some(x) => x.traverse(),
            None => (),
        };

        match &self.right {
            Some(x) => x.traverse(),
            None => (),
        };
    }

    fn fit(
        &mut self,
        old_val: f64,
        df: &mut Vec<&Data>,
        id: &mut i64,
        leaf_nodes: &mut usize,
    ) -> () {
        self.id = *id;
        //self.depth = Some(depth);
        *id += 1;
        self.val = match self.metric {
            Metric::MSE => {
                Some((df.iter().map(|x| x.y).sum::<f64>() / df.len() as f64) * 1.0 + 0.0 * old_val)
            }
            Metric::MAE => Some(median(df)),
        };
        // self.val =
        //     Some((df.iter().map(|x| x.y).sum::<f64>() / df.len() as f64) * 1.0 + 0.0 * old_val);
        // EXPNOTETNIAL SMOOTING IS HERE THATS WHY THE RESUKTS DONT MATHC
        if df.len() <= self.min_samples_split {
            *leaf_nodes += 1;
            return ();
        }
        if self.depth >= self.max_depth {
            *leaf_nodes += 1;
            return;
        }
        let mut opt_score: Option<f64> = None;
        for i in 0..df[0].x.len() {
            df.sort_by(|a, b| a.x[i].partial_cmp(&b.x[i]).unwrap());

            let mut left_sum = 0.0;
            let mut right_sum = df.iter().map(|x| x.y).sum::<f64>();

            let mut left_sum_sq = 0.0;
            let mut right_sum_sq = df.iter().map(|x| x.y.powi(2)).sum::<f64>();
            for j in 1..df.len() {
                let left_count = j as f64;
                let right_count = (df.len() - j) as f64;

                left_sum += df[j - 1].y;
                right_sum -= df[j - 1].y;

                left_sum_sq += df[j - 1].y.powi(2);
                right_sum_sq -= df[j - 1].y.powi(2);
                // let score = (right_sum_sq - right_sum.powi(2) / right_count)
                //     + (left_sum_sq - left_sum.powi(2) / left_count);
                let score: f64 = match self.metric {
                    Metric::MSE => {
                        (right_sum_sq - right_sum.powi(2) / right_count)
                            + (left_sum_sq - left_sum.powi(2) / left_count)
                    }
                    Metric::MAE => {
                        let left_median = median(&df[0..j]);
                        let right_median = median(&df[j..]);
                        df[0..j]
                            .iter()
                            .map(|a| (a.y - left_median).abs())
                            .sum::<f64>()
                            + df[j..]
                                .iter()
                                .map(|a| (a.y - right_median).abs())
                                .sum::<f64>()
                    }
                };

                match opt_score {
                    Some(x) => {
                        if (score < x)
                            && (j as usize >= self.min_samples_leaf)
                            && ((df.len() - j) as usize >= self.min_samples_leaf)
                        {
                            opt_score = Some(score);
                            self.var = Some(i);
                            self.split = Some((df[j].x[i] + df[j - 1].x[i]) / 2.0);
                        }
                    }
                    None => {
                        if (j as usize >= self.min_samples_leaf)
                            && ((df.len() - j) as usize >= self.min_samples_leaf)
                        {
                            self.var = Some(0);
                            opt_score = Some(score);
                            self.split = Some(df[j].x[i]);
                        }
                    }
                }
            }
        }

        match (self.val, self.var, self.split) {
            (Some(val), Some(var), Some(split)) => {
                let (mut df1, mut df2): (Vec<_>, Vec<_>) =
                    df.iter().partition(|a| a.x[var] < split);
                self.left = Some(Box::new(Node::new(
                    self.id + 1,
                    self.depth + 1,
                    None,
                    None,
                    self.max_depth,
                    self.min_samples_split,
                    self.min_samples_leaf,
                    self.metric,
                )));
                self.left
                    .as_mut()
                    .expect("This was just assigned in the prev line")
                    .fit(val, &mut df1, id, leaf_nodes);
                self.right = Some(Box::new(Node::new(
                    self.id + 2,
                    self.depth + 1,
                    None,
                    None,
                    self.max_depth,
                    self.min_samples_split,
                    self.min_samples_leaf,
                    self.metric,
                )));
                self.right
                    .as_mut()
                    .expect("This was just assigned in the prev line")
                    .fit(val, &mut df2, id, leaf_nodes);
            }
            _ => {
                *leaf_nodes += 1;
            }
        };
    }

    fn predict(&self, x: &Data) -> f64 {
        match (&self.right, &self.left, self.val, self.var, self.split) {
            (Some(r), Some(l), Some(_), Some(var), Some(split)) => {
                if x.x[var] >= split {
                    r.predict(x)
                } else {
                    l.predict(x)
                }
            }
            (None, None, Some(val), None, None) => val,
            _ => panic!("Unitialized Tree"),
        }
    }
}

pub struct Tree {
    tree: Option<Node>,
    max_depth: usize,
    min_samples_split: usize,
    min_samples_leaf: usize,
    leaf_nodes: usize,
    metric: Metric,
}

impl Tree {
    pub fn new(
        max_depth: Option<usize>,
        min_samples_split: Option<usize>,
        min_samples_leaf: Option<usize>,
        metric: Option<Metric>,
    ) -> Tree {
        Tree {
            tree: None,
            max_depth: match max_depth {
                Some(x) => x,
                None => 8,
            },
            min_samples_split: match min_samples_split {
                Some(x) => x,
                None => 2,
            },
            min_samples_leaf: match min_samples_leaf {
                Some(x) => x,
                None => 1,
            },
            leaf_nodes: 0,
            metric: match metric {
                Some(x) => x,
                None => Metric::MSE,
            },
        }
    }

    pub fn fit(&mut self, df: &Vec<Data>) -> () {
        self.tree = Some(Node::new(
            1,
            0,
            None,
            None,
            self.max_depth,
            self.min_samples_split,
            self.min_samples_leaf,
            self.metric,
        ));
        let mut id = 0;
        //let depth: usize = 0;
        let mut df2: Vec<&Data> = df.iter().collect();
        let val = df2.iter().map(|x| x.y).sum::<f64>() / df.len() as f64;
        self.tree
            .as_mut()
            .expect("Was just assigned several lines ago")
            .fit(val, &mut df2, &mut id, &mut self.leaf_nodes);
    }

    pub fn traverse(&self) -> Result<(), String> {
        match &self.tree {
            Some(t) => Ok(t.traverse()),
            None => Err("Uninitialized Tree".to_string()),
        }
    }

    pub fn predict(&self, x: &Data) -> Result<f64, String> {
        match &self.tree {
            Some(t) => Ok(t.predict(x)),
            None => Err("Uninitialized Tree".to_string()),
        }
    }
}
