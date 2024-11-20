# ai-bouncer

This repo contains the AI Bouncer AVS with its associated types package and
contract.

## Building

### Contract

To build the contract, run:

```sh
./scripts/optimizer.sh
```

This will place the contract in the `artifacts` directory.

### WASI Component

To build the WASI component (AVS), run:

```sh
./scripts/build_wasi.sh
```

This will place the WASI component in the `components` directory.
