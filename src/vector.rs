use std::fmt::{Display, Formatter, Result};
use std::ops::{Index, IndexMut};

use Vec;

#[derive(Debug, Clone)]
pub struct Vector<K> {
    buff: Vec<K>,
}

impl<K: Clone> Vector<K> {
    pub fn to_vector(buff: &[K]) -> Vector<K> {
        Vector {
            buff: buff.to_vec(),
        }
    }

    pub fn from(buff: Vec<K>) -> Vector<K> {
        Vector { buff }
    }
}

impl<K> Vector<K> {
    pub fn dim(&self) -> usize {
        self.buff.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, K> {
        self.buff.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, K> {
        self.buff.iter_mut()
    }
}

impl<K: Display> Display for Vector<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "(")?;
        for (i, elem) in self.buff.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{elem}")?;
        }

        write!(f, ")")
    }
}

impl<K> IntoIterator for Vector<K> {
    type Item = K;
    type IntoIter = std::vec::IntoIter<K>;

    fn into_iter(self) -> Self::IntoIter {
        self.buff.into_iter()
    }
}

impl<'a, K> IntoIterator for &'a Vector<K> {
    type Item = &'a K;
    type IntoIter = std::slice::Iter<'a, K>;

    fn into_iter(self) -> Self::IntoIter {
        self.buff.iter()
    }
}

impl<'a, K> IntoIterator for &'a mut Vector<K> {
    type Item = &'a mut K;
    type IntoIter = std::slice::IterMut<'a, K>;

    fn into_iter(self) -> Self::IntoIter {
        self.buff.iter_mut()
    }
}

impl<K> Index<usize> for Vector<K> {
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buff[index]
    }
}

impl<K> IndexMut<usize> for Vector<K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buff[index]
    }
}

impl<K> FromIterator<K> for Vector<K> {
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        Vector {
            buff: iter.into_iter().collect(),
        }
    }
}

impl<K> Vector<K> {
    pub fn get_buff(self) -> Vec<K> {
        self.buff
    }
}

impl<K> Vector<K> {
    pub fn get_buff_ref(&self) -> &Vec<K> {
        &self.buff
    }
}

impl<K> Vector<K> {
    pub fn get_mut_buff_ref(&mut self) -> &mut Vec<K> {
        &mut self.buff
    }
}
