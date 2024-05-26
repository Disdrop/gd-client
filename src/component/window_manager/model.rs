#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Window {
  pub id: usize,
  pub tabs: Vec<Tab>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tab {
  pub content: String,
  pub title: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WindowSplitter {
  pub tabs: Vec<Tab>,
  pub id: usize,
  pub children: Vec<WindowSplitter>,
  pub direction: Direction,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
  Horizontal,
  Vertical,
}
