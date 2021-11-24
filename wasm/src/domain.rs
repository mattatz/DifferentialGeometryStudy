
#[derive(Clone)]
pub struct Domain {
  start: f64,
  end: f64,
}

impl Default for Domain {
  fn default () -> Self {
    Domain {
      start: 0.0,
      end: 0.0,
    }
  }
}

impl Domain {
  pub fn new (s: f64, e: f64) -> Domain {
    Domain {
      start: s,
      end: e,
    }
  }

  pub fn start (&self) -> f64 {
    self.start
  }

  pub fn end (&self) -> f64 {
    self.end
  }

  pub fn map (&self, t01: f64) -> f64 {
    (self.end - self.start) * t01 + self.start
  }

}
