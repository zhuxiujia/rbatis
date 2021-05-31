use async_trait::async_trait;
use rbatis_core::db::DBExecResult;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::core::db::{DBPool, DBPoolConn, DBQuery, DBTx};
use crate::core::Error;
use crate::DriverType;

#[async_trait]
pub trait Executor {
    async fn execute(&mut self) -> Result<DBExecResult, Error>;
    async fn fetch<T>(&mut self) -> Result<T, Error> where T: DeserializeOwned;

    /// bind arg into DBQuery
    fn bind_arg<'arg>(
        &self,
        driver_type: &DriverType,
        sql: &'arg str,
        arg: &Vec<serde_json::Value>,
    ) -> Result<DBQuery<'arg>, Error> {
        let mut q: DBQuery = DBPool::make_db_query(driver_type, sql)?;
        for x in arg {
            q.bind_value(x);
        }
        return Ok(q);
    }
}

#[derive(Debug)]
pub struct RBatisConnExecutor {
    pub sql: String,
    pub args: Vec<serde_json::Value>,
    pub conn: DBPoolConn,
}

#[async_trait]
impl Executor for RBatisConnExecutor {
    async fn execute(&mut self) -> Result<DBExecResult, Error> {
        if self.args.len() > 0 {
            let q: DBQuery = self.bind_arg(&self.conn.driver_type, &self.sql, &self.args)?;
            let result = self.conn.exec_prepare(q).await;
            return result;
        } else {
            let result = self.conn.execute(&self.sql).await;
            return result;
        }
    }

    async fn fetch<T>(&mut self) -> Result<T, Error> where T: DeserializeOwned {
        if self.args.len() > 0 {
            let q: DBQuery = self.bind_arg(&self.conn.driver_type, &self.sql, &self.args)?;
            let result: (T, usize) = self.conn.fetch_parperd(q).await?;
            return Ok(result.0);
        } else {
            let result: (T, usize) = self.conn.fetch(&self.sql).await?;
            return Ok(result.0);
        }
    }
}

#[derive(Debug)]
pub struct RBatisTxExecutor {
    pub sql: String,
    pub args: Vec<serde_json::Value>,
    pub conn: DBTx,
}

#[async_trait]
impl Executor for RBatisTxExecutor {
    async fn execute(&mut self) -> Result<DBExecResult, Error> {
        if self.args.len() > 0 {
            let q: DBQuery = self.bind_arg(&self.conn.driver_type, &self.sql, &self.args)?;
            let result = self.conn.exec_prepare(q).await;
            return result;
        } else {
            let result = self.conn.execute(&self.sql).await;
            return result;
        }
    }

    async fn fetch<T>(&mut self) -> Result<T, Error> where T: DeserializeOwned {
        if self.args.len() > 0 {
            let q: DBQuery = self.bind_arg(&self.conn.driver_type, &self.sql, &self.args)?;
            let result: (T, usize) = self.conn.fetch_parperd(q).await?;
            return Ok(result.0);
        } else {
            let result: (T, usize) = self.conn.fetch(&self.sql).await?;
            return Ok(result.0);
        }
    }
}