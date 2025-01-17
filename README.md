<p align="center"><b>Mars: The powerful analysis platform to explore and visualize data from Web3</b></p>

## Features

- __Blazing Fast__ Create from scratch with Rust.

- __Pipeline Processor__ Export Ethereum chain-data to structured-data in hours.

- __Low Cost__ Store structured-data to AWS S3, Azure Blob.

- __Easy to Use__ Web3 visualization and analysis at your fingertips.

## ethetl

- __ethetl__ Lets you export Ethereum data into CSV/Parquet/JSON file format and databases, blazing fast.

### Schema

Databend:
https://github.com/deepeth/mars/tree/main/schemas/databend

Snowflake:
https://github.com/deepeth/mars/tree/main/schemas/snowflake


### How to Use

```shell
$ make build

./target/release/ethetl -p https://mainnet.infura.io/v3/6e83aaa316ef4a8c947b949364f81619 -s 15340159 -e 15340160  -w 16
[2022-08-15T08:25:47Z WARN ] collect: No such file or directory (os error 2)
[2022-08-15T08:25:47Z INFO ] Config: EthConfig { log: LogConfig { level: "INFO", dir: "_logs" }, export: ExportConfig { provider_uri: "https://mainnet.infura.io/v3/6e83aaa316ef4a8c947b949364f81619", start_block: 15340159, end_block: 15340160, batch_size: 1000, max_worker: 16, web3_batch_size: 100, output_dir: "/tmp/eth", output_format: "csv" }, storage: StorageConfig { storage_type: "fs", fs: FsStorageConfig { data_path: "_datas" }, s3: S3StorageConfig { endpoint_url: "https://s3.amazonaws.com", region: "", bucket: "", root: "", access_key_id: "", secret_access_key: "" }, azblob: AzureStorageBlobConfig { endpoint_url: "", container: "", root: "", account_name: "", account_key: "" } }, config_file: "" }
[2022-08-15T08:25:47Z INFO ] backend build started: Builder { root: Some("/home/bohu/github/deepeth/mars/_datas") }
[2022-08-15T08:25:47Z INFO ] backend build finished: Builder { root: Some("/home/bohu/github/deepeth/mars/_datas") }
[2022-08-15T08:25:53Z INFO ] 2 blocks processed, 703 transactions processed, 0 receipts processed, 0 logs processed, 0 token_transfers processed, 0 ens processed. Progress is 100%
[2022-08-15T08:25:55Z INFO ] 2 blocks processed, 703 transactions processed, 0 receipts processed, 0 logs processed, 0 token_transfers processed, 0 ens processed. Progress is 100%
[2022-08-15T08:25:57Z INFO ] 2 blocks processed, 703 transactions processed, 100 receipts processed, 0 logs processed, 0 token_transfers processed, 0 ens processed. Progress is 100%
[2022-08-15T08:25:59Z INFO ] 2 blocks processed, 703 transactions processed, 200 receipts processed, 0 logs processed, 0 token_transfers processed, 0 ens processed. Progress is 100%
[2022-08-15T08:26:01Z INFO ] 2 blocks processed, 703 transactions processed, 300 receipts processed, 0 logs processed, 0 token_transfers processed, 0 ens processed. Progress is 100%
[2022-08-15T08:26:03Z INFO ] 2 blocks processed, 703 transactions processed, 500 receipts processed, 0 logs processed, 0 token_transfers processed, 0 ens processed. Progress is 100%
[2022-08-15T08:26:05Z INFO ] 2 blocks processed, 703 transactions processed, 600 receipts processed, 0 logs processed, 0 token_transfers processed, 0 ens processed. Progress is 100%
[2022-08-15T08:26:07Z INFO ] 2 blocks processed, 703 transactions processed, 703 receipts processed, 1542 logs processed, 777 token_transfers processed, 3 ens processed. Progress is 100%
... ...
```

## License

Mars is licensed under [Apache 2.0](LICENSE).
