// Copyright 2022 BohuTANG.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;

use common_exceptions::Result;
use ethetl::exporters::ReceiptExporter;
use web3::types::H256;

use crate::common::create_config;
use crate::common::create_ctx;

#[ignore]
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_receipts_exporters() -> Result<()> {
    let mut conf = create_config();
    conf.export.start_block = 15340159;
    conf.export.end_block = 15340160;
    let ctx = create_ctx(&conf).await;

    let path = "tests/it/testdata/transactions/_transactions_hash_15340159_15340160.txt";
    let file = File::open(path)?;
    let buffered = BufReader::new(file);

    let mut tx_hashes = vec![];
    for line in buffered.lines() {
        let line_str = &line?;
        tx_hashes.push(H256::from_str(line_str).unwrap());
    }

    let range_path = format!("{}_{}", conf.export.start_block, conf.export.end_block);

    // CSV.
    {
        let exporter =
            ReceiptExporter::create(&ctx, ctx.get_output_dir(), &range_path, tx_hashes.to_vec());
        exporter.export().await?;

        goldenfile::differs::text_diff(
            Path::new("tests/it/testdata/receipts/receipts_15340159_15340160.csv"),
            Path::new("_datas/_test_output_dir/receipts/receipts_15340159_15340160.csv"),
        );

        goldenfile::differs::text_diff(
            Path::new("tests/it/testdata/logs/logs_15340159_15340160.csv"),
            Path::new("_datas/_test_output_dir/logs/logs_15340159_15340160.csv"),
        );

        goldenfile::differs::text_diff(
            Path::new("tests/it/testdata/token_transfers/token_transfers_15340159_15340160.csv"),
            Path::new(
                "_datas/_test_output_dir/token_transfers/token_transfers_15340159_15340160.csv",
            ),
        );

        goldenfile::differs::text_diff(
            Path::new("tests/it/testdata/ens/ens_15340159_15340160.csv"),
            Path::new("_datas/_test_output_dir/ens/ens_15340159_15340160.csv"),
        );
    }
    Ok(())
}
