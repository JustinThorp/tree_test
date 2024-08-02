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

    fn fit(&mut self, df: &mut Vec<&Data>, id: &mut i64, depth: usize) -> () {
        self.id = *id;
        self.depth = Some(depth);
        *id += 1;
        self.val = Some(df.iter().map(|x| x.y).sum::<f64>() / df.len() as f64);
        if df.len() <= 9000 {
            return ();
        }
        let mut opt_score: Option<f64> = None;
        for i in 0..3 {
            df.sort_by(|a, b| a.x[i].partial_cmp(&b.x[i]).unwrap());

            let mut left_sum = 0.0;
            let mut right_sum = df.iter().map(|x| x.y).sum::<f64>();
            for j in 1..df.len() {
                let left_count = j as f64;
                let right_count = (df.len() - j) as f64;

                left_sum += df[j - 1].y;
                right_sum -= df[j - 1].y;

                let left_mean = left_sum / left_count;
                let right_mean = right_sum / right_count;

                let left_rss = df[..j]
                    .iter()
                    .map(|x| (x.y - left_mean).powi(2))
                    .sum::<f64>();
                let right_rss = df[j..]
                    .iter()
                    .map(|x| (x.y - right_mean).powi(2))
                    .sum::<f64>();

                let score = left_rss + right_rss;
                //let score = rss(&df[0..j]) + rss(&df[j..]);
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
        self.left.as_mut().unwrap().fit(&mut df1, id, depth + 1);
        self.right = Some(Box::new(Node::new(self.id + 2, None, None)));
        self.right.as_mut().unwrap().fit(&mut df2, id, depth + 1);
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

// fn mean(x: Vec<f64>) -> f64 {
//     x.iter().sum::<f64>() / x.len() as f64
// }

// fn rss(x: &[&Data]) -> f64 {
//     let mu: f64 = x.iter().map(|x| x.y).sum::<f64>() / x.len() as f64;
//     x.iter().map(|a: &&Data| (a.y - mu).powi(2)).sum()
// }

// struct Tree {
//     tree: Option<Node>,
//     max_depth: Option<i64>,
// }

// impl Tree {
//     fn new() -> Tree {
//         Tree {
//             tree: None,
//             max_depth: Some(2),
//         }
//     }

//     fn fit(&mut self, df: Vec<Data>) -> () {
//         ()
//     }
// }

fn main() {
    let mut tree = Node::new(1, None, None);
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path("data.csv")
        .unwrap();
    let mut df: Vec<Data> = Vec::new();
    for result in rdr.deserialize() {
        let record: Data = result.unwrap();
        df.push(record)
    }
    let mut id = 1;
    let depth: usize = 0;
    let mut df2: Vec<&Data> = df.iter().collect();
    let now = Instant::now();
    tree.fit(&mut df2, &mut id, depth);
    println!("{:?}", now.elapsed());
    //println!("{:?}", tree);
    tree.traverse();
    println!("{}", tree.predict(Data::new(vec![-1.0, 1.7, 0.0], 1.0)));
}
