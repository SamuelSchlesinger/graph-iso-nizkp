# Non-Interactive Proof of Graph Isomorphism

To create a non-interactive proof of graph isomorphism, we can use
the Fiat-Shamir transform.

## Prover

Given graphs A and B, we begin by generating M random isomorphic graphs to A, a
vector C of graphs. We create a binary string S consisting of the binary
representation of A, B, and all of the Cs. We use a one-way function to map
this into a bit-vector V such that if V[i] then we reveal an isomorphism from
C[i] to A, else we reveal an isomorphism from C[i] to B. In the proof, we
include A, B, all of the Cs, and the array of isomorphisms in question, I.

## Verifier

The verifier generates V, then confirms that if V[i], then I[i] maps C[i] to A,
else I[i] maps C[i] to B.

## Correctness

If the prover does know the isomorphism between A and B, then the resulting
proof is correct with probability 1. If the prover does not know the isomorphism,
then they will need to choose their Cs such that the string V allows them to show
an isomorphism.
