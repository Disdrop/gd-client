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

pub struct WindowSplitter {
  pub children: Vec<WindowSplitter>,
  pub direction: Direction,
}

enum Direction {
  Horizontal,
  Vertical,
}

enum WindowOrSplitter {
  WindowSplitter,
  Window,
}
