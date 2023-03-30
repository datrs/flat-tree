//! ## Usage
//! ```rust
//! let mut iter = flat_tree::Iterator::new(0);
//! assert_eq!(iter.next(), Some(2));
//! assert_eq!(iter.next(), Some(4));
//! assert_eq!(iter.next(), Some(6));
//! assert_eq!(iter.parent(), 5);
//! ```
use super::*;

use std::iter;

/// Iterator over a flat-tree.
#[derive(Debug)]
pub struct Iterator {
  index: u64,
  offset: u64,
  factor: u64,
}

impl Iterator {
  /// Create a new iterator.
  ///
  /// ## Examples
  /// ```rust
  /// use flat_tree::Iterator;
  /// assert_eq!(Iterator::new(0).take(3).collect::<Vec<u64>>(), [2, 4, 6]);
  /// ```
  #[inline]
  pub fn new(index: u64) -> Self {
    let mut instance = Self {
      index: 0,
      offset: 0,
      factor: 0,
    };

    instance.seek(index);
    instance
  }

  /// Get the current index.
  #[inline]
  pub fn index(&self) -> u64 {
    self.index
  }

  /// Get the current offset.
  #[inline]
  pub fn offset(&self) -> u64 {
    self.offset
  }

  /// Get the current factor.
  #[inline]
  pub fn factor(&self) -> u64 {
    self.factor
  }

  /// Seek to a position in the iterator.
  ///
  /// ## Examples
  /// ```rust
  /// let mut iter = flat_tree::Iterator::new(0);
  /// iter.seek(4);
  /// assert_eq!(iter.next(), Some(6));
  /// iter.seek(2);
  /// assert_eq!(iter.next(), Some(4));
  /// ```
  #[inline]
  pub fn seek(&mut self, index: u64) {
    self.index = index;
    if is_odd(self.index) {
      self.offset = offset(index);
      self.factor = two_pow(depth(index) + 1);
    } else {
      self.offset = index / 2;
      self.factor = 2;
    }
  }

  /// Check if the position of the iterator is currently on a left node.
  ///
  /// ## Examples
  /// ```rust
  /// assert_eq!(flat_tree::Iterator::new(0).is_left(), true);
  /// assert_eq!(flat_tree::Iterator::new(2).is_left(), false);
  /// assert_eq!(flat_tree::Iterator::new(1).is_left(), true);
  /// ```
  #[inline]
  pub fn is_left(&self) -> bool {
    is_even(self.offset)
  }

  /// Check if the position of the iterator is currently on a right node.
  ///
  /// ## Examples
  /// ```rust
  /// assert_eq!(flat_tree::Iterator::new(0).is_right(), false);
  /// assert_eq!(flat_tree::Iterator::new(2).is_right(), true);
  /// assert_eq!(flat_tree::Iterator::new(1).is_right(), false);
  /// ```
  #[inline]
  pub fn is_right(&self) -> bool {
    is_odd(self.offset)
  }

  /// Check if the the iterator contains given index.
  ///
  /// ## Examples
  /// ```rust
  /// let iter = flat_tree::Iterator::new(3);
  /// assert_eq!(iter.contains(0), true);
  /// assert_eq!(iter.contains(1), true);
  /// assert_eq!(iter.contains(2), true);
  /// assert_eq!(iter.contains(3), true);
  /// assert_eq!(iter.contains(4), true);
  /// assert_eq!(iter.contains(5), true);
  /// assert_eq!(iter.contains(6), true);
  /// assert_eq!(iter.contains(7), false);
  /// assert_eq!(iter.contains(8), false);
  ///
  /// let iter = flat_tree::Iterator::new(9);
  /// assert_eq!(iter.contains(8), true);
  /// assert_eq!(iter.contains(9), true);
  /// assert_eq!(iter.contains(10), true);
  /// assert_eq!(iter.contains(6), false);
  /// assert_eq!(iter.contains(7), false);
  /// assert_eq!(iter.contains(12), false);
  /// assert_eq!(iter.contains(13), false);
  ///
  /// let iter = flat_tree::Iterator::new(8);
  /// assert_eq!(iter.contains(8), true);
  /// assert_eq!(iter.contains(6), false);
  /// assert_eq!(iter.contains(7), false);
  /// assert_eq!(iter.contains(9), false);
  /// assert_eq!(iter.contains(10), false);
  /// ```
  #[inline]
  #[allow(clippy::comparison_chain)]
  pub fn contains(&self, index: u64) -> bool {
    if index > self.index {
      index < (self.index + self.factor / 2)
    } else if index < self.index {
      let comp = self.factor / 2;
      self.index < comp || index > (self.index - comp)
    } else {
      true
    }
  }

  /// Returns how many nodes are in the tree of the current position.
  ///
  /// ## Examples
  /// ```rust
  /// assert_eq!(flat_tree::Iterator::new(0).count_nodes(), 1);
  /// assert_eq!(flat_tree::Iterator::new(1).count_nodes(), 3);
  /// assert_eq!(flat_tree::Iterator::new(3).count_nodes(), 7);
  /// assert_eq!(flat_tree::Iterator::new(5).count_nodes(), 3);
  /// assert_eq!(flat_tree::Iterator::new(23).count_nodes(), 15);
  /// assert_eq!(flat_tree::Iterator::new(27).count_nodes(), 7);
  /// ```
  #[inline]
  pub fn count_nodes(&self) -> u64 {
    if is_even(self.index) {
      1
    } else {
      self.factor - 1
    }
  }

  /// Returns how many leaves are in the tree of the current position.
  ///
  /// ## Examples
  /// ```rust
  /// assert_eq!(flat_tree::Iterator::new(0).count_leaves(), 1);
  /// assert_eq!(flat_tree::Iterator::new(1).count_leaves(), 2);
  /// assert_eq!(flat_tree::Iterator::new(2).count_leaves(), 1);
  /// assert_eq!(flat_tree::Iterator::new(3).count_leaves(), 4);
  /// assert_eq!(flat_tree::Iterator::new(5).count_leaves(), 2);
  /// assert_eq!(flat_tree::Iterator::new(23).count_leaves(), 8);
  /// assert_eq!(flat_tree::Iterator::new(27).count_leaves(), 4);
  /// ```
  #[inline]
  pub fn count_leaves(&self) -> u64 {
    (self.count_nodes() + 1) / 2
  }

  /// Move the cursor and get the previous item from the current position.
  ///
  /// ## Examples
  /// ```rust
  /// let mut iter = flat_tree::Iterator::new(6);
  /// assert_eq!(iter.prev(), 4);
  /// assert_eq!(iter.prev(), 2);
  /// assert_eq!(iter.prev(), 0);
  /// ```
  #[inline]
  pub fn prev(&mut self) -> u64 {
    if self.offset == 0 {
      return self.index;
    }
    self.offset -= 1;
    self.index -= self.factor;
    self.index
  }

  /// Get the sibling for the current position and move the cursor.
  ///
  /// ## Examples
  /// ```rust
  /// assert_eq!(flat_tree::Iterator::new(0).sibling(), 2);
  /// assert_eq!(flat_tree::Iterator::new(1).sibling(), 5);
  /// assert_eq!(flat_tree::Iterator::new(4).sibling(), 6);
  /// ```
  #[inline]
  pub fn sibling(&mut self) -> u64 {
    if self.is_left() {
      self.next().unwrap() // this is always safe
    } else {
      self.prev()
    }
  }

  /// Get the parent for the current position and move the cursor.
  ///
  /// ## Examples
  /// ```rust
  /// assert_eq!(flat_tree::Iterator::new(0).parent(), 1);
  /// assert_eq!(flat_tree::Iterator::new(1).parent(), 3);
  /// assert_eq!(flat_tree::Iterator::new(4).parent(), 5);
  /// ```
  #[inline]
  pub fn parent(&mut self) -> u64 {
    if is_odd(self.offset) {
      self.index -= self.factor / 2;
      self.offset = (self.offset - 1) / 2;
    } else {
      self.index += self.factor / 2;
      self.offset /= 2;
    }
    self.factor *= 2;
    self.index
  }

  /// Get the left_span for the current position and move the cursor.
  ///
  /// ## Examples
  /// ```rust
  /// assert_eq!(flat_tree::Iterator::new(0).left_span(), 0);
  /// assert_eq!(flat_tree::Iterator::new(1).left_span(), 0);
  /// assert_eq!(flat_tree::Iterator::new(3).left_span(), 0);
  /// assert_eq!(flat_tree::Iterator::new(23).left_span(), 16);
  /// assert_eq!(flat_tree::Iterator::new(27).left_span(), 24);
  /// ```
  #[inline]
  pub fn left_span(&mut self) -> u64 {
    self.index = self.index + 1 - self.factor / 2;
    self.offset = self.index / 2;
    self.factor = 2;
    self.index
  }

  /// Get the right_span for the current position and move the cursor.
  ///
  /// ## Examples
  /// ```rust
  /// assert_eq!(flat_tree::Iterator::new(0).right_span(), 0);
  /// assert_eq!(flat_tree::Iterator::new(1).right_span(), 2);
  /// assert_eq!(flat_tree::Iterator::new(3).right_span(), 6);
  /// assert_eq!(flat_tree::Iterator::new(23).right_span(), 30);
  /// assert_eq!(flat_tree::Iterator::new(27).right_span(), 30);
  /// ```
  #[inline]
  pub fn right_span(&mut self) -> u64 {
    self.index = self.index + self.factor / 2 - 1;
    self.offset = self.index / 2;
    self.factor = 2;
    self.index
  }

  /// Get the left_child for the current position and move the cursor.
  ///
  /// ## Examples
  /// ```rust
  /// assert_eq!(flat_tree::Iterator::new(1).left_child(), 0);
  /// assert_eq!(flat_tree::Iterator::new(3).left_child(), 1);
  /// assert_eq!(flat_tree::Iterator::new(7).left_child(), 3);
  /// ```
  #[inline]
  pub fn left_child(&mut self) -> u64 {
    if self.factor == 2 {
      return self.index;
    }
    self.factor /= 2;
    self.index -= self.factor / 2;
    self.offset *= 2;
    self.index
  }

  /// Get the right_child for the current position and move the cursor.
  ///
  /// ## Examples
  /// ```rust
  /// assert_eq!(flat_tree::Iterator::new(1).right_child(), 2);
  /// assert_eq!(flat_tree::Iterator::new(3).right_child(), 5);
  /// assert_eq!(flat_tree::Iterator::new(7).right_child(), 11);
  /// ```
  #[inline]
  pub fn right_child(&mut self) -> u64 {
    if self.factor == 2 {
      return self.index;
    }
    self.factor /= 2;
    self.index += self.factor / 2;
    self.offset = 2 * self.offset + 1;
    self.index
  }

  /// Move to the next tree from the current position and return
  /// new index.
  ///
  /// ## Examples
  /// ```rust
  /// assert_eq!(flat_tree::Iterator::new(0).next_tree(), 2);
  /// assert_eq!(flat_tree::Iterator::new(1).next_tree(), 4);
  /// assert_eq!(flat_tree::Iterator::new(3).next_tree(), 8);
  /// assert_eq!(flat_tree::Iterator::new(7).next_tree(), 16);
  /// ```
  #[inline]
  pub fn next_tree(&mut self) -> u64 {
    self.index = self.index + self.factor / 2 + 1;
    self.offset = self.index / 2;
    self.factor = 2;
    self.index
  }

  /// Move to the previous tree from the current position and return
  /// new index.
  ///
  /// ## Examples
  /// ```rust
  /// assert_eq!(flat_tree::Iterator::new(0).prev_tree(), 0);
  /// assert_eq!(flat_tree::Iterator::new(1).prev_tree(), 0);
  /// assert_eq!(flat_tree::Iterator::new(2).prev_tree(), 0);
  /// assert_eq!(flat_tree::Iterator::new(3).prev_tree(), 0);
  /// assert_eq!(flat_tree::Iterator::new(7).prev_tree(), 0);
  /// assert_eq!(flat_tree::Iterator::new(5).prev_tree(), 2);
  /// assert_eq!(flat_tree::Iterator::new(9).prev_tree(), 6);
  /// assert_eq!(flat_tree::Iterator::new(11).prev_tree(), 6);
  /// ```
  #[inline]
  pub fn prev_tree(&mut self) -> u64 {
    if self.offset == 0 {
      self.index = 0;
      self.factor = 2;
    } else {
      self.index = self.index - self.factor / 2 - 1;
      self.offset = self.index / 2;
      self.factor = 2;
    }
    self.index
  }

  /// Move cursor to the full root of given leaf index that the iterator
  /// index is a part of. If the cursor is already there, nothing is
  /// changed.
  ///
  /// Returns true if a full root exists (moved or not), false if
  /// there are no full roots for the cursor or if given index is not a
  /// leaf.
  ///
  /// See full_roots() for what is meant by a full root.
  ///
  /// ## Examples
  /// ```rust
  /// let mut iter = flat_tree::Iterator::new(0);
  /// assert_eq!(iter.full_root(0), false);
  /// assert_eq!(iter.full_root(22), true);
  /// assert_eq!(iter.index(), 7);
  /// assert_eq!(iter.next_tree(), 16);
  /// assert_eq!(iter.full_root(22), true);
  /// assert_eq!(iter.index(), 17);
  /// assert_eq!(iter.next_tree(), 20);
  /// assert_eq!(iter.full_root(22), true);
  /// assert_eq!(iter.index(), 20);
  /// assert_eq!(iter.next_tree(), 22);
  /// assert_eq!(iter.full_root(22), false);
  /// ```
  #[inline]
  pub fn full_root(&mut self, index: u64) -> bool {
    if index <= self.index || is_odd(self.index) {
      return false;
    }
    while index > self.index + self.factor + self.factor / 2 {
      self.index += self.factor / 2;
      self.factor *= 2;
      self.offset /= 2;
    }
    true
  }
}

impl iter::Iterator for Iterator {
  type Item = u64;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.offset += 1;
    self.index += self.factor;
    Some(self.index)
  }
}

impl Default for Iterator {
  #[inline]
  fn default() -> Self {
    Self::new(0)
  }
}

#[inline]
fn two_pow(n: u64) -> u64 {
  if n < 31 {
    1 << n
  } else {
    (1 << 30) * (1 << (n - 30))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_two_pow() {
    assert_eq!(two_pow(0), 1);
    assert_eq!(two_pow(1), 2);
    assert_eq!(two_pow(2), 4);
    assert_eq!(two_pow(3), 8);
    assert_eq!(two_pow(31), 2147483648);
  }

  #[cfg(target_pointer_width = "64")]
  #[test]
  fn test_two_pow_64bit() {
    assert_eq!(two_pow(34), 17179869184);
    assert_eq!(two_pow(63), 9223372036854775808);
  }
}
