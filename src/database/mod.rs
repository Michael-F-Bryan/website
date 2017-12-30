use std::sync::Arc;

pub trait Database {}

impl<D: Database> Database for Arc<D> {}
