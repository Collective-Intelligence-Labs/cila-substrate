#### Build node

```sh
cargo build --release
```

#### Run node

```sh
./target/release/omnichain-prototype --dev
```

#### Connect to the node

- [Web Explorer](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944#/explorer)

#### Set router

In the [Web Explorer](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944#/explorer), navigate to `Developer` -> `Sudo` panel. Select `dispatcher` pallet, select `addRouter` call and provide an address for your router. Submit the extrinsic from the Sudo account (`Alice` in case of `--dev` mode).

#### Set relay

In the [Web Explorer](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944#/explorer), navigate to `Developer` -> `Sudo` panel. Select `eventStore` pallet, select `addRelay` call and provide an address for your relay. Submit the extrinsic from the Sudo account (`Alice` in case of `--dev` mode).

#### Send omnichain operation

In the [Web Explorer](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944#/explorer), navigate to `Developer` -> `Extrinsics` panel. Select `dispatcher` pallet, select `dispatch` call and provide a protobuf serialized operation in HEX format. Here is an example of an operation that includes both `mint` and `transfer` commands: 
```
0x0a140e8ab7131548af0d9798375b1cc9b5d06322bd6012740a140f9ab7131548af0d9798375b1cc9b5d06322bd601220cf7a2518d9f5bdcc018155b327bd397141421e33ebb2c5c61b3ae987def9f317180122380a20cf12fb4b2580ddd7b7b7952a1c9311441d0bfd8c667188e695d9c8527c549b9412145fd258e46cf5c5b2ad252c37b48aef887e65fd6a12740a140f9ab7131548af0d9798375b1cc9b5d06322bd601220cf7a2518d9f5bdcc018155b327bd397141421e33ebb2c5c61b3ae987def9f317180222380a20cf12fb4b2580ddd7b7b7952a1c9311441d0bfd8c667188e695d9c8527c549b9412144e49ec51fa1ca06af05a2039f603855ccbe1ab2b
```

You can use [Protobufpal](https://www.protobufpal.com/) to serialize and deserialize messages.


## Docker run

docker run -v $(pwd)/substrate-data:/substrate substrate-node-image