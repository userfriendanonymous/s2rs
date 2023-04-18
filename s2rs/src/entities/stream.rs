use std::sync::Arc;
use async_trait::async_trait;
use crate::api::Api;
use crate::cursor::Cursor;

pub type StreamDataResult<S> = Vec<Arc<<S as Stream>::Data>>;
pub type StreamResult<S> = Result<StreamDataResult<S>, <S as Stream>::Error>;

#[async_trait]
pub trait Stream where Self: Sized + Send + Sync {
    type Data: Send + Sync;
    type Error: Send + Sync;

    async fn progress(&mut self) -> StreamResult<Self>;
    fn can_progress(&self) -> bool;

    async fn next(&mut self) -> Option<StreamResult<Self>> {
        if self.can_progress() {
            Some(self.progress().await)
        } else {
            None
        }
    }

    async fn collect(&mut self) -> StreamResult<Self> {
        let mut result = Vec::new();
        while let Some(data) = self.next().await {
            result.append(&mut data?);
        }
        Ok(result)
    }

    async fn for_each<Cb: FnMut(StreamDataResult<Self>) + Send + Sync>(&mut self, mut callback: Cb) -> Result<(), Self::Error> {
        while let Some(data) = self.next().await {
            callback(data?);
        }
        Ok(())
    }
}

pub type GeneralStreamResult<S> = Result<Vec<Arc<<S as GeneralStreamGen>::Data>>, <S as GeneralStreamGen>::Error>;

// region: GeneralStream
#[async_trait]
pub trait GeneralStreamGen {
    type Data: Send + Sync;
    type Error: Send + Sync;
    type This: Send + Sync;
    async fn gen(&self, cursor: Cursor, this: &Arc<Self::This>, api: &Arc<Api>) -> GeneralStreamResult<Self>;
}

pub struct GeneralStream<G: GeneralStreamGen + Clone + Send + Sync> {
    api: Arc<Api>,
    this: Arc<G::This>,
    cursor: Cursor,
    gen: G,
}

impl<G: GeneralStreamGen + Clone + Send + Sync> GeneralStream<G> {
    pub fn with_this(gen: G, cursor: Cursor, this: Arc<G::This>, api: Arc<Api>) -> Self {
        Self {
            api,
            cursor,
            gen,
            this
        }
    }
}

#[async_trait]
impl<G: GeneralStreamGen + Clone + Send + Sync> Stream for GeneralStream<G> {
    type Data = G::Data;
    type Error = G::Error;
    async fn progress(&mut self) -> StreamResult<Self> {
        let result = self.gen.gen(self.cursor.clone(), &self.this, &self.api).await?;
        self.cursor.progress(result.len());
        if result.is_empty() {
            self.cursor.kill();
        }
        Ok(result)
    }
    fn can_progress(&self) -> bool {
        self.cursor.can_progress()
    }
}
// endregion: GeneralStream
