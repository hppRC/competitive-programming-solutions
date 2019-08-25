use search::BinarySearch;
use std::mem::swap;


struct PartiallyPersistentUnionFindTree {
    nodes: Vec<PersistentNode>,
    version: Version,
}

    type Version = usize;

#[derive(Clone, Copy)]
enum Node {
    Root { node_count: usize },
    Descendant { parent_index: usize },
}

    #[derive(Clone)]
    struct PersistentNode {
      node: Node,
      parent_updated_version: Option<Version>,
      node_count_history: Vec<(Version, usize)>,
    }
 
    impl PartiallyPersistentUnionFindTree {
      pub fn new(size: usize) -> Self {
        PartiallyPersistentUnionFindTree {
          nodes: vec![
            PersistentNode {
              node: Node::Root { node_count: 1 },
              parent_updated_version: None,
              node_count_history: vec![],
            };
            size
          ],
          version: 0,
        }
      }
 
      pub fn len(&self) -> usize {
        self.nodes.len()
      }
 
      pub fn unite(&mut self, l_index: usize, r_index: usize) -> bool {
        self.version += 1;
        let (mut l_root_index, mut r_root_index) = {
          let version = self.version;
          (self.find(version, l_index), self.find(version, r_index))
        };
        if l_root_index == r_root_index {
          return false;
        }
        match (self.nodes[l_root_index].node, self.nodes[r_root_index].node) {
          (Node::Root { node_count: l_node_count }, Node::Root { node_count: r_node_count }) => {
            let node_count = l_node_count + r_node_count;
            if l_node_count < r_node_count {
              swap(&mut l_root_index, &mut r_root_index);
            }
            self.nodes[l_root_index].node = Node::Root { node_count: node_count };
            self.nodes[l_root_index].node_count_history.push((self.version, node_count));
            self.nodes[r_root_index].node = Node::Descendant { parent_index: l_root_index };
            self.nodes[r_root_index].parent_updated_version = Some(self.version);
          }
          _ => unreachable!("`find` must return root index"),
        }
        true
      }
 
      pub fn find(&self, version: Version, index: usize) -> usize {
        assert!(version <= self.version());
        debug_assert!(index < self.len());
        match self.nodes[index].parent_updated_version {
          None => index,
          Some(parent_updated_version) if version < parent_updated_version => index,
          _ => match self.nodes[index].node {
            Node::Descendant { parent_index } => self.find(version, parent_index),
            _ => unreachable!("`parent_updated_version` is invalid"),
          },
        }
      }
 
      pub fn is_same_group(&self, version: Version, l_index: usize, r_index: usize) -> bool {
        assert!(version <= self.version());
        debug_assert!(l_index < self.len());
        debug_assert!(r_index < self.len());
        self.find(version, l_index) == self.find(version, r_index)
      }
 
      pub fn count_elements(&self, version: Version, index: usize) -> usize {
        assert!(version <= self.version());
        debug_assert!(index < self.len());
        let node_count_history = self.nodes[index].node_count_history.as_slice();
        match BinarySearch::binary_search(node_count_history, |&(updated_version, _)| updated_version <= version) {
          Some(i) => node_count_history[i].1,
          None => 1,
        }
      }
 
      pub fn version(&self) -> Version {
        self.version
      }
    }
  }
