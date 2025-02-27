// Copyright (c) 2022 by Rivos Inc.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

//! Confidential Containers Key Broker Service

extern crate anyhow;

use anyhow::Result;
use api_server::{config::Config, ApiServer};
use attestation_service::AttestationService;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;

use clap::Parser;

static SESSION_TIMEOUT: i64 = 5;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Socket address (IP:port) to listen to, e.g. 127.0.0.1:8080.
    /// This can be set multiple times.
    #[arg(required = true, short, long)]
    socket: Vec<SocketAddr>,

    /// HTTPS session timeout (in minutes)
    #[arg(default_value_t = SESSION_TIMEOUT, short, long)]
    timeout: i64,

    /// KBS config file path.
    #[arg(default_value_t = String::default(), short, long)]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let cli = Cli::parse();
    let kbs_config = match cli.config.as_str() {
        "" => Config::default(),
        _ => Config::try_from(Path::new(&cli.config))?,
    };

    let api_server = ApiServer::new(
        kbs_config,
        cli.socket,
        Arc::new(AttestationService::new()?),
        cli.timeout,
    );
    api_server.serve().await.map_err(anyhow::Error::from)
}
