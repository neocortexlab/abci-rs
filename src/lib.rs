extern crate bytes;
extern crate byteorder;
extern crate protobuf;
extern crate integer_encoding;
extern crate tokio;
extern crate tokio_io;
extern crate tokio_service;
extern crate tokio_proto;
extern crate futures;

pub mod server;
pub mod types;

mod abci_service;
mod abci_codec;

use types::*;

pub trait Application {

    // Info/Query connection
    fn info(&self, req: &RequestInfo) -> ResponseInfo;

    fn set_option(&self, req: &RequestSetOption) -> ResponseSetOption;

    fn query(&self, p: &RequestQuery) -> ResponseQuery;

    // Mempool connection
    fn check_tx(&self, p: &RequestCheckTx) -> ResponseCheckTx;

    // Consensus connection
    fn init_chain(&self, p: &RequestInitChain) -> ResponseInitChain;

    fn begin_block(&self, p: &RequestBeginBlock) -> ResponseBeginBlock;

    fn deliver_tx(&self, p: &RequestDeliverTx) -> ResponseDeliverTx;

    fn end_block(&self, p: &RequestEndBlock) -> ResponseEndBlock;

    fn commit(&self, p: &RequestCommit) -> ResponseCommit;

    // Miscellaneous connection
    fn echo(&self, p: &RequestEcho) -> ResponseEcho;

    fn flush(&self, p: &RequestFlush) -> ResponseFlush;
}
