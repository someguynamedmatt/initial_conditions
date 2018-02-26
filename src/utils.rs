//! Utility functions, etc

extern crate num;

#[allow(unused_imports)] use num::Float;

/// # Extended Vec traits
///
///
/// ## dot
///   - `v1.dot(v2) // returns the dot product of the two vectors`
///   - panic occurs when `v1.len() != v2.len()`
///
/// ## scalar_mult
///   - multiplies each item in Vec by `k`, mutating self
///
/// ## add_vec
///   - adds the ith element of v1 to the ith element of v2
///
/// ## sub_vec
///   - subtracts the ith element of v2 from the ith element of v1
///
pub trait VecTraits<T> {
    fn dot(self, v2: Vec<T>) -> T where T: Float;
    fn scalar_mult(&mut self, k: T) where T: Float;
    fn scalar_div(&mut self, k: T) where T: Float;
    fn add_vec(self, v2: Vec<T>) -> Vec<T> where T: Float;
    fn sub_vec(self, v2: Vec<T>) -> Vec<T> where T: Float;
}

impl<T> VecTraits<T> for Vec<T> {
    fn dot(self, v2: Vec<T>) -> T
        where T: Float
    {
        assert!(self.len() == v2.len(),
        "Vectors are not of equal length"
        );

        let mut dot_product: f64 = 0.0;
        for i in 0..self.len() {
            let item_from_v1: f64 = num::cast(self[i]).unwrap();
            let item_from_v2: f64 = num::cast(v2[i]).unwrap();
            dot_product += item_from_v1 * item_from_v2;
        }
        num::cast(dot_product).unwrap()
    }

    fn scalar_mult(&mut self, k: T)
        where T: Float
    {
        for i in 0..self.len() {
            self[i] = self[i] * k;
        }
    }

    fn scalar_div(&mut self, k: T)
        where T: Float
    {
        for i in 0..self.len() {
            if k != T::zero() {
                self[i] = self[i] / k;
            }
            else {
                self[i] = T::infinity();
            }
        }
    }

    fn add_vec(self, v2: Vec<T>) -> Vec<T>
        where T: Float
    {
        assert!(self.len() == v2.len(),
        "Vectors are not of equal length"
        );

        let mut return_vec: Vec<T> = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            return_vec.push(self[i] + v2[i]);
        }
        return_vec
    }

    fn sub_vec(self, v2: Vec<T>) -> Vec<T>
        where T: Float
    {
        assert!(self.len() == v2.len(),
        "Vectors are not of equal length"
        );

        let mut return_vec: Vec<T> = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            return_vec.push(self[i] - v2[i]);
        }
        return_vec
    }
}

#[cfg(test)]
mod tests {
    extern crate num;

    use super::*;
    #[allow(unused_imports)] use num::Float;

    #[test]
    fn dot() {
        let v = vec![2.0, 2.0, 2.0];
        let v2 = vec![2.0, 2.0, 2.0];

        let expected = 12 as f64;
        assert_eq!(v.dot(v2), expected);
    }

    #[test]
    #[should_panic]
    fn dot_should_panic() {
        let v = vec![2.0, 2.0, 2.0];
        let v2 = vec![2.0, 2.0];
        v.dot(v2);
    }

    #[test]
    fn scalar_mult() {
        let mut v = vec![2.0, 2.0, 2.0];
        v.scalar_mult(2.0);

        let expected = vec![4.0, 4.0, 4.0];
        assert_eq!(&v, &expected);
    }

    #[test]
    fn scalar_div() {
        let mut v = vec![2.0, 2.0, 2.0];
        v.scalar_div(2.0);

        let expected = vec![1.0, 1.0, 1.0];
        assert_eq!(&v, &expected);
    }

    #[test]
    fn add_vec() {
        let v = vec![2.0, 2.0, 2.0];
        let v2 = vec![1.0, -2.0, 4.0];
        let new_v = v.add_vec(v2);

        let expected = vec![3.0, 0.0, 6.0];
        assert_eq!(new_v, expected);
    }

    #[test]
    #[should_panic]
    fn add_vec_should_panic() {
        let v = vec![2.0, 2.0, 2.0];
        let v2 = vec![2.0, 2.0];
        v.add_vec(v2);
    }

    #[test]
    fn sub_vec() {
        let v = vec![2.0, 2.0, 2.0];
        let v2 = vec![1.0, -2.0, 4.0];
        let new_v = v.sub_vec(v2);

        let expected = vec![1.0, 4.0, -2.0];
        assert_eq!(new_v, expected);
    }

    #[test]
    #[should_panic]
    fn sub_vec_should_panic() {
        let v = vec![2.0, 2.0, 2.0];
        let v2 = vec![2.0, 2.0];
        v.add_vec(v2);
    }
}
