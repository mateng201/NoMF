# NoMF

A tiny offline demo for the `erc8004` SDK.

It demonstrates:

- creating an `Erc8004` client with a `Network`;
- reading chain id and registry addresses;
- generating an agent registry prefix;
- building a `RegistrationFile` with `ServiceEndpoint` and `Registration`;
- serializing/deserializing the registration JSON;
- showing the expected error when a client has no configured registry address.

Run:

```powershell
cargo run
```

The provider URL is `https://localhost:1` on purpose. This demo does not call
the chain, so no private key, RPC service, or testnet funds are required.
