use std::fmt::{Display, Error, Formatter};
use std::ops::{Index, IndexMut};
use std::usize;

use crate::vector::Vector;

pub struct Matrix<K> {
    buff: Vec<K>,
    w: usize,
    h: usize,
}

impl<K: Clone> Matrix<K> {
    pub fn from_multiple(buff: Vec<Vec<K>>) -> Result<Matrix<K>, &'static str> {
        let expected = match buff.first() {
            Some(v) => v.len(),
            None => 0,
        };
        let h = buff.len();
        if !buff.iter().all(|v| v.len() == expected) {
            return Err("Mismatched Matrix from vec of vec, not all rows have the same lenght");
        }

        if expected == 0 {
            return Ok(Matrix {
                buff: vec![],
                w: 0,
                h: 0,
            });
        }
        Ok(Matrix {
            buff: buff.into_iter().flatten().collect(),
            h,
            w: expected,
        })
    }

    pub fn from_buff(buff: Vec<K>, w: usize, h: usize) -> Result<Matrix<K>, &'static str> {
        if buff.len() != w * h {
            return Err(
                "Mismatched Matrix from vec, given width and height are impossible, they don't multiply to len of the buffer",
            );
        }

        Ok(Matrix { buff: buff, h, w })
    }
}

impl<K> Matrix<K> {
    pub fn width(&self) -> usize {
        self.w
    }

    pub fn height(&self) -> usize {
        self.h
    }

    pub fn mut_width(&mut self) -> usize {
        self.w
    }

    pub fn mut_height(&mut self) -> usize {
        self.h
    }
}

impl<K> Matrix<K> {
    pub fn iter_buff(&self) -> std::slice::Iter<'_, K> {
        self.buff.iter()
    }

    pub fn iter_mut_buff(&mut self) -> std::slice::IterMut<'_, K> {
        self.buff.iter_mut()
    }
}

impl<K> IntoIterator for Matrix<K> {
    type Item = K;
    type IntoIter = std::vec::IntoIter<K>;

    fn into_iter(self) -> Self::IntoIter {
        self.buff.into_iter()
    }
}

impl<'a, K> IntoIterator for &'a Matrix<K> {
    type Item = &'a K;
    type IntoIter = std::slice::Iter<'a, K>;

    fn into_iter(self) -> Self::IntoIter {
        self.buff.iter()
    }
}

impl<'a, K> IntoIterator for &'a mut Matrix<K> {
    type Item = &'a mut K;
    type IntoIter = std::slice::IterMut<'a, K>;

    fn into_iter(self) -> Self::IntoIter {
        self.buff.iter_mut()
    }
}

impl<K> Matrix<K> {
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

impl<K: Copy> Matrix<K> {
    pub fn copy(&self) -> Matrix<K> {
        Matrix {
            buff: self.buff.iter().map(|x| *x).collect(),
            w: self.w,
            h: self.h,
        }
    }
}

impl<K> Index<usize> for Matrix<K> {
    type Output = [K];

    fn index(&self, index: usize) -> &Self::Output {
        &self.buff[self.width() * index..self.width() * (index + 1)]
    }
}

impl<K> IndexMut<usize> for Matrix<K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buff[self.w * index..self.w * (index + 1)]
    }
}

impl<K> Matrix<K> {
    pub fn take(&self, i: usize, j: usize) -> &K {
        assert!(i < self.height() && j < self.width());
        &self.buff[i * self.width() + j]
    }

    pub fn mut_take(&mut self, i: usize, j: usize) -> &mut K {
        assert!(i < self.height() && j < self.width());
        &mut self.buff[i * self.w + j]
    }
}

impl<K> Matrix<K> {
    pub fn get_buff(self) -> Vec<K> {
        self.buff
    }

    pub fn get_buff_ref(&self) -> &Vec<K> {
        &self.buff
    }

    pub fn get_buff_ref_mut(&mut self) -> &mut Vec<K> {
        &mut self.buff
    }
}

impl<K: Display> Display for Matrix<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for (i, elem) in self.buff.iter().enumerate() {
            if i % self.w == 0 {
                write!(f, "[")?;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{elem}")?;
            if i % self.w == self.w - 1 {
                write!(f, "]\n")?;
            }
        }
        write!(f, "")
    }
}

impl<K: Clone> Matrix<K> {
    pub fn rows(&self) -> Vec<Vector<K>> {
        (0..self.height())
            .map(|i| Vector::from(self.buff[i * self.width()..(i + 1) * self.width()].to_vec()))
            .collect()
    }
}

impl<K: Clone> Matrix<K> {
    pub fn mut_rows(&self) -> Vec<Vector<K>> {
        (0..self.height())
            .map(|i| Vector::from(self.buff[i * self.width()..(i + 1) * self.width()].to_vec()))
            .collect()
    }
}
