// Copyright 2020 The FuseQuery Authors.
//
// Code is licensed under AGPL License, Version 3.0.

use std::fmt;

use crate::datablocks::DataBlock;
use crate::datavalues::{DataArrayRef, DataSchema, DataType};
use crate::error::{FuseQueryError, FuseQueryResult};
use crate::functions::{arithmetics, AggregatorFunction, ConstantFunction, VariableFunction};

#[derive(Clone)]
pub enum Function {
    Constant(ConstantFunction),
    Variable(VariableFunction),
    Add(arithmetics::AddFunction),
    Sub(arithmetics::SubFunction),
    Div(arithmetics::DivFunction),
    Mul(arithmetics::MulFunction),
    Aggregator(AggregatorFunction),
}

impl Function {
    pub fn name(&self) -> &'static str {
        match self {
            Function::Constant(v) => v.name(),
            Function::Variable(v) => v.name(),
            Function::Add(v) => v.name(),
            Function::Div(v) => v.name(),
            Function::Mul(v) => v.name(),
            Function::Sub(v) => v.name(),
            Function::Aggregator(v) => v.name(),
        }
    }

    pub fn return_type(&self, input_schema: &DataSchema) -> FuseQueryResult<DataType> {
        match self {
            Function::Constant(v) => v.return_type(input_schema),
            Function::Variable(v) => v.return_type(input_schema),
            Function::Add(v) => v.return_type(input_schema),
            Function::Div(v) => v.return_type(input_schema),
            Function::Mul(v) => v.return_type(input_schema),
            Function::Sub(v) => v.return_type(input_schema),
            Function::Aggregator(v) => v.return_type(),
        }
    }

    pub fn nullable(&self, input_schema: &DataSchema) -> FuseQueryResult<bool> {
        match self {
            Function::Constant(v) => v.nullable(input_schema),
            Function::Variable(v) => v.nullable(input_schema),
            Function::Add(v) => v.nullable(input_schema),
            Function::Div(v) => v.nullable(input_schema),
            Function::Mul(v) => v.nullable(input_schema),
            Function::Sub(v) => v.nullable(input_schema),
            Function::Aggregator(v) => v.nullable(input_schema),
        }
    }

    pub fn evaluate(&self, block: &DataBlock) -> FuseQueryResult<DataArrayRef> {
        match self {
            Function::Constant(v) => v.evaluate(block),
            Function::Variable(v) => v.evaluate(block),
            Function::Add(v) => v.evaluate(block),
            Function::Div(v) => v.evaluate(block),
            Function::Mul(v) => v.evaluate(block),
            Function::Sub(v) => v.evaluate(block),
            _ => Err(FuseQueryError::Unsupported(format!(
                "Unsupported evaluate() for function {}",
                self.name()
            ))),
        }
    }

    pub fn accumulate(&mut self, block: &DataBlock) -> FuseQueryResult<()> {
        match self {
            Function::Aggregator(ref mut v) => v.accumulate(block),
            _ => Err(FuseQueryError::Unsupported(format!(
                "Unsupported accumulate() for function {}",
                self.name()
            ))),
        }
    }

    pub fn aggregate(&self) -> FuseQueryResult<DataArrayRef> {
        match self {
            Function::Aggregator(v) => v.aggregate(),
            _ => Err(FuseQueryError::Unsupported(format!(
                "Unsupported aggregators() for function {}",
                self.name()
            ))),
        }
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Function::Constant(v) => write!(f, "{}", v),
            Function::Variable(v) => write!(f, "{}", v),
            Function::Add(v) => write!(f, "{}", v),
            Function::Div(v) => write!(f, "{}", v),
            Function::Mul(v) => write!(f, "{}", v),
            Function::Sub(v) => write!(f, "{}", v),
            Function::Aggregator(v) => write!(f, "{}", v),
        }
    }
}
