# Constructive Ordering Finder (Tree Prune Search)
This project computes the [number of constructive orderings](https://pubs.lib.umn.edu/index.php/mjum/article/view/4154) for a provided value of n. This project differs from the linear version by the method of permutation generation. As opossed to being generated in full from index values, permutations are generated and and checked recursively, element by element, using a tree. When a partial permutation is found to be invalid, the branch is abandoned, saving computation of many irrelevant permutation elements. This project is significantly faster than `cof_linear`, and should be used in most cases.  