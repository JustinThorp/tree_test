// Vec of data is the data used to build tree
// at each split we sort by each x position and evailate potetnial splits
// each node aves the varaible used the split and the prediction
//use rand::Rng;
use std::time::Instant;

#[derive(serde::Deserialize, Debug)]
struct Data {
    y: f64,
    x: Vec<f64>,
}

impl Data {
    fn new(x: Vec<f64>, y: f64) -> Data {
        Data { x, y }
    }
}

#[derive(Debug)]
struct Node {
    id: i64,
    depth: Option<usize>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    val: Option<f64>,
    var: Option<usize>,
    split: Option<f64>,
}
impl Node {
    fn new(id: i64, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Node {
        Node {
            id,
            depth: None,
            left,
            right,
            val: None,
            var: None,
            split: None,
        }
    }

    fn traverse(&self) {
        println!(
            "{:width$}, value : {:.3}",
            self.id,
            self.val.unwrap(),
            width = (self.depth.unwrap() * 2) as usize
        );
        if self.left.is_some() {
            self.left.as_ref().unwrap().traverse();
        }
        if self.right.is_some() {
            self.right.as_ref().unwrap().traverse();
        }
    }

    fn fit(
        &mut self,
        df: &mut Vec<&Data>,
        id: &mut i64,
        depth: usize,
        max_depth: usize,
        min_samples_split: usize,
    ) -> () {
        self.id = *id;
        self.depth = Some(depth);
        *id += 1;
        self.val = Some(df.iter().map(|x| x.y).sum::<f64>() / df.len() as f64);
        if df.len() <= min_samples_split {
            return ();
        }
        if self.depth.unwrap() >= max_depth {
            return;
        }
        let mut opt_score: Option<f64> = None;
        for i in 0..3 {
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

                let score = (right_sum_sq - right_sum.powi(2) / right_count)
                    + (left_sum_sq - left_sum.powi(2) / left_count);

                if opt_score.is_none() {
                    self.var = Some(0);
                    opt_score = Some(score);
                    self.split = Some(df[j].x[i]);
                } else if score < opt_score.unwrap() {
                    opt_score = Some(score);
                    self.var = Some(i);
                    self.split = Some(df[j].x[i]);
                }
            }
        }
        let (mut df1, mut df2): (Vec<_>, Vec<_>) = df
            .iter()
            .partition(|a| a.x[self.var.unwrap()] < self.split.unwrap());
        self.left = Some(Box::new(Node::new(self.id + 1, None, None)));
        self.left
            .as_mut()
            .unwrap()
            .fit(&mut df1, id, depth + 1, max_depth, min_samples_split);
        self.right = Some(Box::new(Node::new(self.id + 2, None, None)));
        self.right
            .as_mut()
            .unwrap()
            .fit(&mut df2, id, depth + 1, max_depth, min_samples_split);
    }

    fn predict(&self, x: Data) -> f64 {
        match &self.right {
            Some(_) => {
                if (x.x[self.var.unwrap()] >= self.split.unwrap()) && (self.right.is_some()) {
                    return self.right.as_ref().unwrap().predict(x);
                } else if (x.x[self.var.unwrap()] < self.split.unwrap()) && (self.left.is_some()) {
                    return self.left.as_ref().unwrap().predict(x);
                } else {
                    return 5.0;
                }
            }
            None => self.val.unwrap(),
        }
    }
}

struct Tree {
    tree: Option<Node>,
    max_depth: Option<usize>,
    min_samples_split: Option<usize>,
}

impl Tree {
    fn new(max_depth: Option<usize>, min_samples_split: Option<usize>) -> Tree {
        Tree {
            tree: None,
            max_depth,
            min_samples_split,
        }
    }

    fn fit(&mut self, df: Vec<Data>) -> () {
        self.tree = Some(Node::new(1, None, None));
        let mut id = 1;
        let depth: usize = 0;
        let mut df2: Vec<&Data> = df.iter().collect();
        self.tree.as_mut().unwrap().fit(
            &mut df2,
            &mut id,
            depth,
            self.max_depth.unwrap(),
            self.min_samples_split.unwrap(),
        );
    }

    fn traverse(&self) -> () {
        self.tree.as_ref().unwrap().traverse();
    }

    fn predict(&self, x: Data) -> f64 {
        self.tree.as_ref().unwrap().predict(x)
    }
}

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
    let mut tree = Tree::new(Some(40), Some(10));
    tree.fit(df);
    tree.traverse();
    println!("{:?}", now.elapsed());
    println!("{}", tree.predict(Data::new(vec![-1.0, 1.7, 0.0], 1.0)));
}
