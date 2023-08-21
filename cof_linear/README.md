# Constructive Ordering Finder (Linear)

This project computes the [number of constructive orderings](https://pubs.lib.umn.edu/index.php/mjum/article/view/4154) for a provided value of n, using a multithreaded, linear approach. For k threads, the entire search space of permutations of size n are divided into k even partitions, one per thread. Each thread computes the first permutation of its assigned partition, checks if its a valid ordering or not, and computes the next permutation. Permutations were modified in place, and are not precomputed, as this is infeasible for all but the smallest vales for n. The count of valid orderings are tracked per thread, and each returned through a MPSC channel. 

