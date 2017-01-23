#[macro_use(s)]
extern crate ndarray;

use ndarray::{Array3, arr3};

fn main() {
    // Create a three-dimensional f64 array, initialized with zeros
    let mut temperature = Array3::<f64>::zeros((3, 4, 5));

    // Increase the temperature in this location, notice the
    // double-brackets indexing `temperature`
    temperature[[2, 2, 2]] += 0.5;

    // Create a 3-dimensional matrix,
    // 2 submatrices of 2 rows with 3 elements per row, means a shape
    // of `[2, 2, 3]`.
    let a = arr3(&[[[ 1,  2,  3],     // -- 2 rows  \_
                    [ 4,  5,  6]],    // --         /
                   [[ 7,  8,  9],     //            \_ 2 submatrices
                    [10, 11, 12]]]);  //            /
    //  3 columns ..../.../.../

    // This is a 2 x 2 x 3 array
    assert_eq!(a.shape(), &[2, 2, 3]);

    // Let’s create a slice of `a` with
    //
    // - Both of the submatrices of the greatest dimension: `..`
    // - Only the first row in each submatrix: `0..1`
    // - Every element in each row: `..`
    let b = a.slice(s![.., 0..1, ..]);

    // This is the result of the above slice into `a`
    let c = arr3(&[[[ 1,  2,  3]],
                   [[ 7,  8,  9]]]);
    assert_eq!(b, c);
    assert_eq!(b.shape(), &[2, 1, 3]);
}
