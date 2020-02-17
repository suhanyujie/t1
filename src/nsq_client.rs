extern crate asyncnsq;
extern crate async_std;
extern crate serde;

use async_std::{io, task};
use asyncnsq::data::Address;
use asyncnsq::{NsqHttpdInitConfig};
use serde::{Deserialize, Serialize};


pub #[derive(Serialize, Deserialize)]
pub struct TestMsg {
    name: String,
}

impl TestMsg {
    fn new() -> TestMsg {
        TestMsg {
            name: "test".to_string(),
        }
    }
}

impl MsgHandler for TestMsg {
    async fn handler(&self, msg: Msg) ->Option<Msg> {
        dbg!(&self.name);
        let res = msg.finish().await;
        dbg!(res);
        None
    }
}

async fn test_reader() -> Result<()> {
    let nsq_init_config: NsqHttpdInitConfig = read_toml_config::<NsqHttpdInitConfig>("tests/config.toml".to_string()).await?;
    let pt = serde_json
}