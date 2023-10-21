# f2q 🎇

[![Test](https://github.com/Quantum-for-Life/f2q/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/Quantum-for-Life/f2q/actions/workflows/test.yml)

Fermion-to-qubit mappings.

High-octane representation of Pauli Hamiltonians with up to 64 qubits.

WIP 🚧

- Parse and convert Hamiltonian representations used in quantum chemical
  calculation into sums of Pauli strings
- Canonical representation of second-quantized Hamiltonian
- `SumRepr` can be serialized/deserialized quickly
- `Hamil` is dynamical and can store: various encodings, functions generating
  sum terms, iterators etc., all at the same time
- Interface easily extendible to other mappings by implementing `Terms` trait.
- Implement time evolution / dump to QASM file

To use the command line tool:

```sh
cargo install f2q
```

To see documentation:

```sh
cargo doc --open
```

or [online](https://docs.rs/f2q/0.1.0/f2q/).
