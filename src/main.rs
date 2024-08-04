// I think there is a slight difference in predictions since
// I use unqiue values for splits and sklearn takes the midpoint between two points
//Dont think it matters that much but may fix
//use rand::Rng;
use std::time::Instant;

#[derive(serde::Deserialize, Debug)]
struct Data {
    y: f64,
    x: Vec<f64>,
}
#[allow(dead_code)]
impl Data {
    fn new(x: Vec<f64>, y: f64) -> Data {
        Data { y, x }
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
        }
    }

    fn traverse(&self) {
        println!(
            "{:width$}, value : {:.3},split: {}",
            self.id,
            self.val.unwrap(),
            self.split.unwrap_or(-1.0),
            width = (self.depth * 2) as usize
        );
        if self.left.is_some() {
            self.left.as_ref().unwrap().traverse();
        }
        if self.right.is_some() {
            self.right.as_ref().unwrap().traverse();
        }
    }

    fn fit(&mut self, old_val: f64, df: &mut Vec<&Data>, id: &mut i64) -> () {
        self.id = *id;
        //self.depth = Some(depth);
        *id += 1;
        self.val =
            Some((df.iter().map(|x| x.y).sum::<f64>() / df.len() as f64) * 1.0 + 0.0 * old_val);
        // EXPNOTETNIAL SMOOTING IS HERE THATS WHY THE RESUKTS DONT MATHC
        if df.len() <= self.min_samples_split {
            return ();
        }
        if self.depth >= self.max_depth {
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
                    if (j as usize > self.min_samples_leaf)
                        && ((df.len() - j) as usize > self.min_samples_leaf)
                    {
                        self.var = Some(0);
                        opt_score = Some(score);
                        self.split = Some(df[j].x[i]);
                    }
                } else if (score < opt_score.unwrap())
                    && (j as usize > self.min_samples_leaf)
                    && ((df.len() - j) as usize > self.min_samples_leaf)
                {
                    opt_score = Some(score);
                    self.var = Some(i);
                    self.split = Some((df[j].x[i] + df[j - 1].x[i]) / 2.0);
                }
            }
        }
        let (mut df1, mut df2): (Vec<_>, Vec<_>) = df
            .iter()
            .partition(|a| a.x[self.var.unwrap()] < self.split.unwrap());
        self.left = Some(Box::new(Node::new(
            self.id + 1,
            self.depth + 1,
            None,
            None,
            self.max_depth,
            self.min_samples_split,
            self.min_samples_leaf,
        )));
        self.left
            .as_mut()
            .unwrap()
            .fit(self.val.unwrap(), &mut df1, id);
        self.right = Some(Box::new(Node::new(
            self.id + 2,
            self.depth + 1,
            None,
            None,
            self.max_depth,
            self.min_samples_split,
            self.min_samples_leaf,
        )));
        self.right
            .as_mut()
            .unwrap()
            .fit(self.val.unwrap(), &mut df2, id);
    }

    fn predict(&self, x: &Data) -> f64 {
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
    min_samples_leaf: Option<usize>,
}

impl Tree {
    fn new(
        max_depth: Option<usize>,
        min_samples_split: Option<usize>,
        min_samples_leaf: Option<usize>,
    ) -> Tree {
        Tree {
            tree: None,
            max_depth,
            min_samples_split,
            min_samples_leaf,
        }
    }

    fn fit(&mut self, df: &Vec<Data>) -> () {
        self.tree = Some(Node::new(
            1,
            0,
            None,
            None,
            self.max_depth.unwrap(),
            self.min_samples_split.unwrap(),
            self.min_samples_leaf.unwrap(),
        ));
        let mut id = 0;
        //let depth: usize = 0;
        let mut df2: Vec<&Data> = df.iter().collect();
        let val = df2.iter().map(|x| x.y).sum::<f64>() / df.len() as f64;
        self.tree.as_mut().unwrap().fit(val, &mut df2, &mut id);
    }

    fn traverse(&self) -> () {
        self.tree.as_ref().unwrap().traverse();
    }

    fn predict(&self, x: &Data) -> f64 {
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
    let mut tree = Tree::new(Some(8), Some(500), Some(20));
    tree.fit(&df);
    tree.traverse();
    println!("{:?}", now.elapsed());
    // println!(
    //     "{}",
    //     tree.predict(&Data::new(vec![-0.508852, 0.633505, 1.511747], 1.440603))
    // );
    let rss = df
        .iter()
        .map(|a| (tree.predict(a) - a.y).powi(2))
        .sum::<f64>()
        / df.len() as f64;
    println!("{rss}");
    let mut rdr2 = csv::ReaderBuilder::new()
        .flexible(true)
        .from_path("data_2.csv")
        .unwrap();
    let mut df2: Vec<Data> = Vec::new();
    for result in rdr2.deserialize() {
        let record: Data = result.unwrap();
        df2.push(record)
    }
    let rss2 = df2
        .iter()
        .map(|a| (tree.predict(a) - a.y).powi(2))
        .sum::<f64>()
        / df2.len() as f64;
    println!("{:?}", df2[5]);
    println!("{}", tree.predict(&df2[5]));
    println!("{}", rss2)
}
