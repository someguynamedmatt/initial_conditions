//! Extended Vec traits
//! Addon traits to the standard Rust Vec type.

extern crate num;

#[allow(unused_imports)] use num::Float;

/// # dot (Trait)
///   - `v1.dot(v2) // returns the dot product of the two vectors`
///   - panic occurs when `v1.len() != v2.len()`
pub trait Dot<T> {
    fn dot(self, v2: Vec<T>) -> T where T: Float;
}

/// # scalar_mult (Trait)
///   - multiplies each item in Vec by `k`, mutating self
///
pub trait ScalarMult<T> {
    fn scalar_mult(&mut self, k: T) where T: Float;
}

/// # scalar_div (Trait)
///   - divides each item in Vec by `k`, mutating self
///   - If zero is passed as `k`, value will be `inf`
///
pub trait ScalarDiv<T> {
    fn scalar_div(&mut self, k: T) where T: Float;
}

/// # add_vec (Trait)
///   - adds the ith element of `v1` (`self`) to the ith element of `v2`
///
pub trait AddVec<T> {
    fn add_vec(self, v2: Vec<T>) -> Vec<T> where T: Float;
}

/// # sub_vec (Trait)
///   - subtracts the ith element of `v2` from the ith element of `v1` (`self`)
///
pub trait SubVec<T> {
    fn sub_vec(self, v2: Vec<T>) -> Vec<T> where T: Float;
}

/// # euc_norm (Trait)
///     - Returns the Euclidean Norm (length) of the vector (n-dimensional)
///
pub trait EuclideanNorm<T> {
    fn euc_norm(self) -> T where T: Float;
}

impl<T> Dot<T> for Vec<T> {
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
}

impl<T> ScalarMult<T> for Vec<T> {
    fn scalar_mult(&mut self, k: T)
        where T: Float
    {
        for i in 0..self.len() {
            self[i] = self[i] * k;
        }
    }
}

impl<T> ScalarDiv<T> for Vec<T> {
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
}

impl<T> AddVec<T> for Vec<T> {
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
}

impl<T> SubVec<T> for Vec<T> {
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

impl<T> EuclideanNorm<T> for Vec<T> {
    fn euc_norm(self) -> T
        where T: Float
    {
        if self.len() == 0 { return T::zero(); }

        let mut norm: f64 = 0.0;
        for i in 0..self.len() {
            let x2: f64 = num::cast(self[i] * self[i]).unwrap();
            norm += x2;
        }
        T::sqrt(num::cast(norm).unwrap())
    }
}

#[cfg(test)]
mod tests {
    extern crate num;

    use super::*;
    use num::abs;
    #[allow(unused_imports)] use num::Float;

    const EPSILON: f64 = 1e-10;

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

    #[test]
    fn euc_norm1() {
        let v = vec![1.0, 2.0, 1.0];
        let norm = v.euc_norm();

        let expected = 2.449489742783178;
        let diff = abs(norm - expected);

        assert!(diff <= EPSILON);
    }

    #[test]
    fn euc_norm2() {
        let v: Vec<f64> = Vec::new();
        let norm = v.euc_norm();

        let expected = 0.0;
        assert_eq!(norm, expected);
    }
}
