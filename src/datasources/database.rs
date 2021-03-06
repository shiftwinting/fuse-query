// Copyright 2020 The FuseQuery Authors.
//
// Code is licensed under AGPL License, Version 3.0.

use std::collections::HashMap;
use std::sync::Arc;

use crate::datasources::ITable;
use crate::error::{FuseQueryError, FuseQueryResult};

pub trait IDatabase: Sync + Send {
    fn name(&self) -> &str;
    fn add_table(&mut self, table: Arc<dyn ITable>) -> FuseQueryResult<()>;
    fn get_table(&self, table_name: &str) -> FuseQueryResult<Arc<dyn ITable>>;
}

pub struct Database {
    name: String,
    tables: HashMap<String, Arc<dyn ITable>>,
}

impl Database {
    pub fn create(name: &str) -> Self {
        Database {
            name: name.to_string(),
            tables: Default::default(),
        }
    }
}

impl IDatabase for Database {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn add_table(&mut self, table: Arc<dyn ITable>) -> FuseQueryResult<()> {
        self.tables.insert(table.name().to_string(), table);
        Ok(())
    }

    fn get_table(&self, table: &str) -> FuseQueryResult<Arc<dyn ITable>> {
        Ok(self
            .tables
            .get(table)
            .ok_or_else(|| FuseQueryError::Internal(format!("Can not find the table: {}", table)))?
            .clone())
    }
}
