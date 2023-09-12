## Governance Explainer

The parachain governance model is designed to be flexible and adaptable. It can be changed over time to meet the needs of the Polkadot ecosystem. This is illustrated by the fact that most projects, including *Polkadot* started with the **sudo** pallet as an initial form of on-chain authority, capable of performing actions that cannot be done otherwise. Let's understand this better: 

The concept of on-chain authority ties intricately with the concept of [Origin](https://docs.substrate.io/build/origins/). The `Root` origin is a means of indicating usually to a pallet, that the action is taken by the superuser. The pallet would check if the `Origin` is a superuser `Root` origin by the `ensure_root(Origin)` check. 

Now coming back to `pallet-sudo`, this pallet allows you to define one single account key that can act as superuser on-chain. That's convenient for development purposes when you want to quickly test privileged calls, but what about production? 

EPT comes with two runtimes `devnet` and `mainnet`. The `devnet` runtime is targeted solely for development and quick testing of on-chain logic, thus it makes sense to include `pallet-sudo`. But to demonstrate that this is not an ideal production-ready practice, the `mainnet` runtime configures `pallet-sudo` with a **multisig** key. More information on multisigs can be found on this [Multisig Deep Dive](https://youtu.be/J2OAcd4sWfA?si=yC4zKiQGY1FrQt_v).

As an example the `mainnet` runtime comes configured with a 3-account multisig for the sudo key, with a threshold of 2 meaning that at least 2/3 of the initial authority set must sign and send a privileged call for it to pass `ensure_root(Origin)` check. This means increased security, you don't have to rely on a single key to perform superuser actions, and increased decentralization, or at least a taste of it, given that the keys may be distributed across locations to prevent compromise.

### Governance in Production: 

The first iterations of Polkadot started with `pallet-sudo` and when the network operations were recognized as stable, it was removed in favor of a democracy based governance model comprising of `pallet-collective` instances like `Council` and `TechnicalCommittee`, `pallet-democracy` to dispatch privileged calls, and `pallet-membership`. You can read more about them on the [Polkadot wiki (Governance v1)](https://wiki.polkadot.network/docs/learn-governance).

As you might guess, the above setup is not trivial for small networks who are trying out governance models to develop a mental model of how things work on non-sudo land. 

** *Enter `pallet-motion`* **

We developed `pallet-motion` as a means to simplify `Council` based voting on proposals (privileged calls), by directly enabling `pallet-collective` instances, henceforth referred to as `Council`, to dispatch with the `Root` origin. [Read about pallet-motion extrinsics here.](../pallets/motion/README.md)

As a next step towards production readiness, a team might want to perform a runtime upgrade where they remove `pallet-sudo`, have `Council` instances in their runtime, and try making privileged calls through one of the three provided origins by `pallet-motion`. 

