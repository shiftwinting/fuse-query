// Copyright 2020 The FuseQuery Authors.
//
// Code is licensed under AGPL License, Version 3.0.

use async_trait::async_trait;
use std::sync::Arc;

use crate::contexts::Context;
use crate::datasources::Partition;
use crate::datastreams::SendableDataBlockStream;
use crate::error::FuseQueryResult;
use crate::processors::IProcessor;

pub struct SourceTransform {
    ctx: Context,
    db: String,
    table: String,
    partitions: Vec<Partition>,
}

impl SourceTransform {
    pub fn create(ctx: Context, db: &str, table: &str, partitions: Vec<Partition>) -> Self {
        SourceTransform {
            ctx,
            db: db.to_string(),
            table: table.to_string(),
            partitions,
        }
    }
}

#[async_trait]
impl IProcessor for SourceTransform {
    fn name(&self) -> &'static str {
        "SourceTransform"
    }

    fn connect_to(&mut self, _: Arc<dyn IProcessor>) {
        unimplemented!()
    }

    async fn execute(&self) -> FuseQueryResult<SendableDataBlockStream> {
        let table = self.ctx.table(self.db.as_str(), self.table.as_str())?;
        table.read(self.partitions.clone()).await
    }
}
