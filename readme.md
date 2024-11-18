## Weisfeiler-Lehmann Graph hashing in rust.

At the moment, only implemented for DAGs, as this is relevant for my use case.

This is a stepping stone for integration into the rustworkx library.

As a blueprint, I used the python implementation for the networkx library.

## Details

The function operates by iteratively aggregating and hashing the neighborhoods of each node. At each iteration, the neighbors' labels are hashed to update the node labels, and a hashed histogram of these updated labels is generated as the final hash.

This process ensures identical hashes for isomorphic graphs while providing strong guarantees that non-isomorphic graphs will produce different hashes. For further details, refer to [Shervashidze, Nino, Pascal Schweitzer, Erik Jan Van Leeuwen, Kurt Mehlhorn, and Karsten M. Borgwardt. Weisfeiler Lehman Graph Kernels. Journal of Machine Learning Research. 2011.](http://www.jmlr.org/papers/volume12/shervashidze11a/shervashidze11a.pdf)

