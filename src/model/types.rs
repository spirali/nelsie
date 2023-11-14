use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use crate::model::PosAndSizeExpr::Const;

#[derive(Debug, Deserialize, Copy, Clone, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub(crate) struct NodeId(u32);

impl NodeId {
    #[cfg(test)]
    pub fn new(node_id: u32) -> Self {
        NodeId(node_id)
    }
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Size {
    Points(f32),
    Percent(f32),
    Auto,
}


#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(PartialEq))]
pub(crate) enum PosAndSizeExpr {
    Const { value: f32 },
    X { node_id: NodeId },
    Y { node_id: NodeId },
    Width { node_id: NodeId, fraction: f32 },
    Height { node_id: NodeId, fraction: f32 },
    Sum { expressions: Vec<PosAndSizeExpr> },
}

impl PosAndSizeExpr {
    pub fn new_const(value: f32) -> Self {
        Const { value }
    }
    pub fn new_sum(expr1: &Self, expr2: &Self) -> Self {
        match (expr1, expr2) {
            (x, Const { value: 0.0 }) | (Const { value: 0.0 }, x) => x.clone(),
            (Const { value: value1 }, Const { value: value2 }) => Const { value: value1 + value2 },
            (PosAndSizeExpr::Sum { expressions: es }, x) => {
                let mut expressions = es.clone();
                expressions.push(x.clone());
                PosAndSizeExpr::Sum { expressions }
            }
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Color(svgtypes::Color);

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let color = svgtypes::Color::from_str(&value)
            .map_err(|_| serde::de::Error::custom("Invalid color"))?;
        Ok(Color(color))
    }
}

impl From<&Color> for svgtypes::Color {
    fn from(value: &Color) -> Self {
        value.0
    }
}


// A conditionally-compiled module
#[cfg(test)]
mod test {
    use crate::model::{NodeId, PosAndSizeExpr};
    use crate::model::PosAndSizeExpr::{Const, Sum, X};

    #[test]
    pub fn test_expressions_sum_build() {
        let e0 = Const { value: 0.0 };
        let e1 = Const { value: 1.0 };
        let e2 = Const { value: 2.0 };
        let e4 = X { node_id: NodeId::new(5) };
        let e5 = Sum { expressions: vec![e2.clone(), e4.clone()] };
        let e6 = X { node_id: NodeId::new(5) };

        assert_eq!(PosAndSizeExpr::new_sum(&e0, &e1), e1);
        assert_eq!(PosAndSizeExpr::new_sum(&e2, &e0), e2);
        assert_eq!(PosAndSizeExpr::new_sum(&e5, &e6), Sum { expressions: vec![e2.clone(), e4.clone(), e6.clone()] });
    }
}