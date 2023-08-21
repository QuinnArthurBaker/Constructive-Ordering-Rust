# COF 

This project is a supporting library for the linear Constructive Ordering finder project. It contains four functions.

## `factorial(BigUint) -> BigUint`
A simple factorial calculator using BigUInt values. 

## `validate_perm(&[usize]) -> usize
Given an array of `usize`s, returns 1 if it is a [valid constructive ordering](https://pubs.lib.umn.edu/index.php/mjum/article/view/4154), and 0 if not. Numerical values are used to allow this function to be treated as a counter function when calling this function many times in succession (See `cof_linear`).

## `kth_permuation(n: usize, k: mut BigUint) -> Vec<usize>`
Using a [factorial base representation](https://en.wikipedia.org/wiki/Factorial_number_system), this function transforms an input index value `k` into the `k`th corresponding permutation of a particular size `n`, in lexicographical order. E.g. `kth_permutation(4,0)` returns a `Vec` of `[0,1,2,3]`, `kth_permutation(4,1)` returns `[0,1,3,2]`, `kth_permutation(4,2)` returns `[0,2,1,3]`, and so on.

## `next_permutation<T: Ord>(&mut [T]) -> bool`
Taken from [Project Nayuki](https://www.nayuki.io/page/next-lexicographical-permutation-algorithm), this function takes a reference to an array, and transforms it into the next lexicographical permutation, if possible. This function returns false if the input array was the last lexicographical permutation, or true otherwise. 
