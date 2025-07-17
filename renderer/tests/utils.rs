use renderer::{Color, Document, LayoutExpr, LengthOrExpr, Node, NodeId, Page, Resources};
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};

fn current_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("current")
}

fn snapshots_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("snapshots")
}

// Returns the current function name
#[macro_export]
macro_rules! test_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        let name = &name[..name.len() - 3];
        let name = &name[name.rfind(':').map(|x| x + 1).unwrap_or(0)..];

        name
    }};
}

pub struct TestEnv<'a> {
    name: &'a str,
    width: f32,
    height: f32,
    node_id: NodeId,
}

impl<'a> TestEnv<'a> {
    pub fn new(name: &'a str, width: f32, height: f32) -> Self {
        Self {
            name,
            width,
            height,
            node_id: NodeId::new(0),
        }
    }

    pub fn new_node(&self) -> Node {
        Node::builder(NodeId::new(0))
            .x(LayoutExpr::ZERO)
            .y(LayoutExpr::ZERO)
            .width(LengthOrExpr::points(self.width))
            .height(LengthOrExpr::points(self.height))
            .build()
    }

    pub fn check(&self, nodes: Vec<Node>) {
        let resources = Resources::new(false, false, false, false);
        let mut doc = Document::new();

        let n_pages = nodes.len();
        for node in nodes {
            let page = Page::new(
                node,
                self.width,
                self.height,
                Color::from_str("white").unwrap(),
            );
            doc.add_page(page);
        }
        let snapshots_path = snapshots_dir().join(&self.name);
        let out = doc.render_png_to_vec(&resources).unwrap();
        assert_eq!(out.len(), n_pages);
        let mut errors = Vec::new();

        for (i, out) in out.iter().enumerate() {
            let filename = format!("{}.png", i);
            let current = image::load_from_memory(out).unwrap().to_rgb8();
            if let Ok(reference_data) = std::fs::read(snapshots_path.join(&filename)) {
                let reference = image::load_from_memory(&reference_data).unwrap().to_rgb8();
                if reference != current {
                    let target = current_dir().join(&self.name).join(&filename);
                    current.save(target).unwrap();
                    errors.push(format!("{filename}: images differ"));
                }
            } else {
                errors.push(format!("{filename}: reference image not found",));
            }
        }
        if !errors.is_empty() {
            let current_path = current_dir().join(&self.name);
            create_dir_all(&current_path).unwrap();
            for (i, out) in out.iter().enumerate() {
                let filename = format!("{}.png", i);
                std::fs::write(current_path.join(&filename), out).unwrap();
            }
            let msg = errors.join("\n");
            panic!("Test '{}' failed: {}", self.name, msg);
        }
    }
}
