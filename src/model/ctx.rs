use key_path::path;
use crate::value::Value;
use crate::connection::transaction;
use crate::model;
use crate::model::Model;

#[derive(Clone)]
pub struct Ctx {
    pub transaction_ctx: transaction::Ctx,
    pub model: &'static Model,
}

impl Ctx {

    pub async fn find_unique<T: From<model::Object>>(&self, finder: &Value) -> teo_result::Result<Option<T>> {
        self.transaction_ctx.find_unique(self.model, finder, None, path![]).await
    }

    pub async fn find_first<T: From<model::Object>>(&self, finder: &Value) -> teo_result::Result<Option<T>> {
        self.transaction_ctx.find_first(self.model, finder, None, path![]).await
    }

    pub async fn find_many<T: From<model::Object>>(&self, finder: &Value) -> teo_result::Result<Vec<T>> {
        self.transaction_ctx.find_many(self.model, finder, None, path![]).await
    }

    pub async fn count(&self, finder: &Value) -> teo_result::Result<Value> {
        self.transaction_ctx.count(self.model, finder, path![]).await
    }

    pub async fn count_objects(&self, finder: &Value) -> teo_result::Result<usize> {
        self.transaction_ctx.count_objects(self.model, finder, path![]).await
    }

    pub async fn count_fields<T, E>(&self, finder: &Value) -> teo_result::Result<T> where T: TryFrom<Value, Error=E>, teo_result::Error: From<E> {
        self.transaction_ctx.count_fields(self.model, finder, path![]).await
    }

    pub async fn aggregate<T, E>(&self, finder: &Value) -> teo_result::Result<T> where T: TryFrom<Value, Error=E>, teo_result::Error: From<E> {
        Ok(self.transaction_ctx.aggregate(self.model, finder, path![]).await?.try_into()?)
    }

    pub async fn group_by<T, E>(&self, finder: &Value) -> teo_result::Result<Vec<T>> where T: TryFrom<Value, Error=E>, teo_result::Error: From<E> {
        Ok(self.transaction_ctx.group_by(self.model, finder, path![]).await?.into_iter().map(|t| T::try_from(t)).collect::<Result<Vec<T>, E>>()?)
    }

    pub async fn sql<T, E>(&self, sql: &str) -> teo_result::Result<Vec<T>> where T: TryFrom<Value, Error=E>, teo_result::Error: From<E> {
        self.transaction_ctx.sql(self.model, sql).await
    }

    pub async fn create_object<T>(&self, input: &Value) -> teo_result::Result<T> where T: From<model::Object> {
        Ok(self.transaction_ctx.create_object(self.model, input, None).await?.into())
    }
}