use cupid::{Config, walk};

fn main() {
    let conf = Config::new(".", 1024);
    let graph = walk(conf);
    graph.export_dot("graph.dot")
}
