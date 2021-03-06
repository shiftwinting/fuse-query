// Copyright 2020 The FuseQuery Authors.
//
// Code is licensed under AGPL License, Version 3.0.

pub type Partitions = Vec<Partition>;

#[derive(Clone)]
pub struct Partition {
    pub name: String,
    pub version: u64,
}
