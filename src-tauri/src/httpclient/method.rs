// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// #![allow(unused_imports)]

// use super::InvokeContext;
// use tauri::Runtime;
use serde::Deserialize;
// use tauri_macros::{command_enum, module_command_handler, CommandModule};
use crate::httpclient::{client, client::{ClientBuilder, HttpRequestBuilder, ResponseData}, err};
// #[cfg(http_request)]
use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
  path::{Display, Path},
};


// #[cfg(http_request)]
// use client::{ClientBuilder, HttpRequestBuilder, ResponseData};
// #[cfg(not(http_request))]
// type ClientBuilder = ();
// #[cfg(not(http_request))]
// type HttpRequestBuilder = ();
// #[cfg(not(http_request))]
// #[allow(dead_code)]
// type ResponseData = ();

pub type ClientId = u32;
// #[cfg(http_request)]
type ClientStore = Arc<Mutex<HashMap<ClientId, client::Client>>>;

// #[cfg(http_request)]
fn clients() -> &'static ClientStore {
  use once_cell::sync::Lazy;
  static STORE: Lazy<ClientStore> = Lazy::new(Default::default);
  &STORE
}

/// The API descriptor.
// #[command_enum]
// #[derive(Deserialize, CommandModule)]
// #[cmd(async)]
// #[serde(tag = "cmd", rename_all = "camelCase")]
#[derive(Deserialize)]
pub enum Cmd {
  /// Create a new HTTP client.
  // #[cmd(http_request, "http > request")]
  CreateClient { options: Option<ClientBuilder> },
  /// Drop a HTTP client.
  // #[cmd(http_request, "http > request")]
  DropClient { client: ClientId },
  /// The HTTP request API.
  // #[cmd(http_request, "http > request")]
  HttpRequest {
    client: ClientId,
    options: Box<HttpRequestBuilder>,
  },
}


impl Cmd {
//   #[module_command_handler(http_request)]
  pub async fn create_client(
    // _context: InvokeContext<R>,
    options: Option<ClientBuilder>,
  ) -> err::Result<ClientId> {
    let client = options.unwrap_or_default().build()?;
    let mut store = clients().lock().unwrap();
    let id = rand::random::<ClientId>();
    store.insert(id, client);
    Ok(id)
  }

//   #[module_command_handler(http_request)]
  pub async fn drop_client(
    // _context: InvokeContext<R>
    client: ClientId,
  ) -> err::Result<()> {
    let mut store = clients().lock().unwrap();
    store.remove(&client);
    Ok(())
  }

//   #[module_command_handler(http_request)]
  pub async fn http_request(
    // context: InvokeContext<R>,
    client_id: ClientId,
    options: Box<HttpRequestBuilder>,
  ) -> err::Result<ResponseData> {
    // use crate::Manager;
    // println!("http_request: {:?}", options);
    let client = clients()
        .lock()
        .unwrap()
        .get(&client_id)
        .ok_or_else(|| err::Error::HttpClientNotInitialized)? 
        .clone();
    let options = *options;
    if let Some(client::Body::Form(form)) = &options.body {
        for value in form.0.values() {
          if let client::FormPart::File {
            file: client::FilePart::Path(path),
            ..
          } = value
          {
            if SafePathBuf::new(path.clone()).is_err()
            {
              return Err(err::Error::PathNotAllowed(path.clone()));
            }
          }
        }
      }
      use url::Url;
      if options.url == Url::parse(r"https:\/\/hihuu.club/").unwrap() {
        return Err(err::Error::Fail("Here is hihuu.club from Rust :)".to_string()));
      }
      let response = client.send(options).await?;
      Ok(response.read().await?)
 }
}
// #[cfg(test)]
// mod tests {
//   use super::{ClientBuilder, ClientId};

//   #[tauri_macros::module_command_test(http_request, "http > request")]
//   #[quickcheck_macros::quickcheck]
//   fn create_client(options: Option<ClientBuilder>) {
//     assert!(crate::async_runtime::block_on(super::Cmd::create_client(
//       crate::test::mock_invoke_context(),
//       options
//     ))
//     .is_ok());
//   }

//   #[tauri_macros::module_command_test(http_request, "http > request")]
//   #[quickcheck_macros::quickcheck]
//   fn drop_client(client_id: ClientId) {
//     crate::async_runtime::block_on(async move {
//       assert!(
//         super::Cmd::drop_client(crate::test::mock_invoke_context(), client_id)
//           .await
//           .is_ok()
//       );
//       let id = super::Cmd::create_client(crate::test::mock_invoke_context(), None)
//         .await
//         .unwrap();
//       assert!(
//         super::Cmd::drop_client(crate::test::mock_invoke_context(), id)
//           .await
//           .is_ok()
//       );
//     });
//   }
// }


#[derive(Clone, Debug)]
pub(crate) struct SafePathBuf(std::path::PathBuf);

impl SafePathBuf {
  pub fn new(path: std::path::PathBuf) -> Result<Self, &'static str> {
    if path
      .components()
      .any(|x| matches!(x, std::path::Component::ParentDir))
    {
      Err("cannot traverse directory, rewrite the path without the use of `../`")
    } else {
      Ok(Self(path))
    }
  }

  #[allow(dead_code)]
  pub unsafe fn new_unchecked(path: std::path::PathBuf) -> Self {
    Self(path)
  }

  #[allow(dead_code)]
  pub fn display(&self) -> Display<'_> {
    self.0.display()
  }
}

impl AsRef<Path> for SafePathBuf {
  fn as_ref(&self) -> &Path {
    self.0.as_ref()
  }
}

impl<'de> Deserialize<'de> for SafePathBuf {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let path = std::path::PathBuf::deserialize(deserializer)?;
    SafePathBuf::new(path).map_err(serde::de::Error::custom)
  }
}