use actix::{prelude::*, Actor, SyncArbiter};
use num_cpus::get as num_cpus;

use ::clap::Parser;
use ::sha2::{Digest, Sha256};
use log::*;

use libhf::is_zero_terminated;

mod arg;

mod worker;
use worker::*;

mod manager;
use manager::*;

mod messages;
use messages::*;

#[actix_rt::main]
async fn main() {
    let args = arg::Args::parse();
    if args.debug {
        env_logger::init();
    }

    let manager = ManagerBuilder::default()
        .with_workers(args.threads.unwrap_or(num_cpus()))
        .with_records(args.records)
        .with_zeroes(args.count_zeroes)
        .build()
        .start();

    while manager.connected() {
        ::actix_rt::time::sleep(std::time::Duration::from_millis(10)).await;
    }
}
