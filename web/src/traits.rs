use activitystreams_kinds as Kind;

pub trait Object: Sized + Debug {
  type DataType: Clone + Send + Sync;
  type Kind;
  type Error;

  // Required methods
  fn read_from_id<'life0, 'async_trait>(
      object_id: Url,
      data: &'life0 Data<Self::DataType>
  ) -> Pin<Box<dyn Future<Output = Result<Option<Self>, Self::Error>> + Send + 'async_trait>>
     where Self: 'async_trait,
           'life0: 'async_trait;
  fn into_json<'life0, 'async_trait>(
      self,
      data: &'life0 Data<Self::DataType>
  ) -> Pin<Box<dyn Future<Output = Result<Self::Kind, Self::Error>> + Send + 'async_trait>>
     where Self: 'async_trait,
           'life0: 'async_trait;
  fn verify<'life0, 'life1, 'life2, 'async_trait>(
      json: &'life0 Self::Kind,
      expected_domain: &'life1 Url,
      data: &'life2 Data<Self::DataType>
  ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'async_trait>>
     where Self: 'async_trait,
           'life0: 'async_trait,
           'life1: 'async_trait,
           'life2: 'async_trait;
  fn from_json<'life0, 'async_trait>(
      json: Self::Kind,
      data: &'life0 Data<Self::DataType>
  ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Error>> + Send + 'async_trait>>
     where Self: 'async_trait,
           'life0: 'async_trait;

  // Provided methods
  fn last_refreshed_at(&self) -> Option<NaiveDateTime> { ... }
  fn delete<'life0, 'async_trait>(
      self,
      _data: &'life0 Data<Self::DataType>
  ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'async_trait>>
     where Self: Send + 'async_trait,
           'life0: 'async_trait { ... }
}

pub trait Actor: Object + Send + 'static {
  // Required methods
  fn id(&self) -> Url;
  fn public_key_pem(&self) -> &str;
  fn private_key_pem(&self) -> Option<String>;
  fn inbox(&self) -> Url;

  // Provided methods
  fn public_key(&self) -> PublicKey { ... }
  fn shared_inbox(&self) -> Option<Url> { ... }
  fn shared_inbox_or_inbox(&self) -> Url { ... }
}