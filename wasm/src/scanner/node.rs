use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Node {
  kind: super::Kind,
  id: u32,
  name: String,
  path: String,
  value: u64,
  parent: Option<u32>,
  children: Option<Vec<Node>>,
}

impl Node {
  pub fn new(
    kind: super::Kind,
    id: u32,
    name: String,
    path: String,
    value: u64,
    parent: Option<u32>,
    children: Option<Vec<Node>>,
  ) -> Node {
    Node {
      kind: kind,
      id: id,
      name: name,
      path: path,
      value: value,
      parent: parent,
      children: children,
    }
  }

  pub fn get_value(&self) -> u64 {
    self.value
  }

  pub fn get_id(&self) -> u32 {
    self.id
  }

  pub fn increment_dir_size(&mut self, size: u64) {
    self.value += size;
  }

  pub fn add_child(&mut self, node: Node) {
    if let Some(ref mut children) = self.children {
      children.push(node);
    } else {
      self.children = Some(vec![node]);
    }
  }

  pub fn get_last_child(&mut self) -> &mut Node {
    if let Some(ref mut children) = self.children {
      let index = children.len() - 1;
      return &mut children[index];
    } else {
      panic!("Cannot get last child");
    }
  }
}
