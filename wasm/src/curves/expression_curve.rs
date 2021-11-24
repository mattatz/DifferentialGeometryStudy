
use crate::curves::parametric_curve::ParametricCurve;
use std::collections::BTreeMap;
use crate::optimizer::df::F;
use crate::domain::Domain;

pub struct ExpressionCurve {
  x_expr: Box<F>,
  y_expr: Box<F>,
  z_expr: Box<F>,
  domain: Domain,
}

fn expression(expr: String) -> Box<F> {
  Box::new(move |t| {
    let mut map : BTreeMap<String, f64> = BTreeMap::new();
    map.insert("t".to_string(), t);
    return fasteval::ez_eval(&expr.clone().into_boxed_str(), &mut map).unwrap()
  })
}

impl ExpressionCurve {
  pub fn expression (x: String, y: String, z: String, d: Domain) -> Self {
    Self {
      x_expr: expression(x),
      y_expr: expression(y),
      z_expr: expression(z),
      domain: d,
    }
  }
}

impl Default for ExpressionCurve {
  fn default() -> Self {
    ExpressionCurve::expression(
      String::from("cos(t)"),
      String::from("sin(t)"),
      String::from("t"),
      Domain::new(0.0, 10.0),
    )
  }
}

impl ParametricCurve for ExpressionCurve {
  fn domain(&self) -> Domain {
    self.domain.clone()
  }
  fn x_expr(&self) -> &Box<F> {
    &self.x_expr
  }
  fn y_expr(&self) -> &Box<F> {
    &self.y_expr
  }
  fn z_expr(&self) -> &Box<F> {
    &self.z_expr
  }
}
