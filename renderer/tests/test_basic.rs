use crate::utils::TestEnv;
use renderer::{Color, LayoutExpr, LengthOrExpr, NodeId, Page, Resources};
use renderer::{Document, Node};
use std::path::{Path, PathBuf};

mod utils;

#[test]
fn render_node() {
    let env = TestEnv::new(test_name!(), 120.0, 60.0);
    let c1 = Color::from_str("red").unwrap();
    let c2 = Color::from_str("blue").unwrap();
    let mut n1 = env.new_node();
    let n2 = Node::builder(NodeId::new(2))
        .width(LengthOrExpr::points(70.0))
        .height(LengthOrExpr::points(20.0))
        .bg_color(c1)
        .build();
    let n3 = Node::builder(NodeId::new(3))
        .width(LengthOrExpr::points(70.0))
        .height(LengthOrExpr::points(20.0))
        .bg_color(c2)
        .build();
    n1.add_child_node(n2);
    n1.add_child_node(n3);
    env.check(vec![n1]);
}
