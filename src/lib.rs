#![deny(clippy::all)]
#![allow(unused_imports)]
#[macro_use]
extern crate napi_derive;
extern crate email_format;
extern crate mailstrom;

use email_format::Email;
use futures::prelude::*;
use mailstrom::config::*;
use mailstrom::storage::MemoryStorage;
use mailstrom::Mailstrom;
use napi::bindgen_prelude::*;
use tokio::*;

#[napi]
pub async fn send(username:String,timestamp:String,authtype:String, password:String, server:String,port:u32, require_tls:bool, from:String, to:String, replay:String, subject:String, body:String) {
  let mut email = Email::new(
    username.as_str(),             // "From:"
    timestamp.as_str(), // "Date:"
  )
  .unwrap();

  email.set_sender(from.as_str()).unwrap();
  email
    .set_reply_to(replay.as_str())
    .unwrap();
  email
    .set_to(to.as_str())
    .unwrap();
  email.set_subject(subject.as_str()).unwrap();
  email
    .set_body(
      body.as_str()
    )
    .unwrap();
    // pub enum Mechanism {
  //   Plain,
  //   Login,
  //   Xoauth2,
  // }
  let _mechanism =  match authtype.as_str() {
    "Login" => Mechanism::Login,
    "Plain" => Mechanism::Plain,
    "Xoauth2" => Mechanism::Xoauth2,
    _ => Mechanism::Plain,
  };
    let auth = SmtpAuth {
      mechanism: _mechanism,
      username: username,
      password: password,
    };
    let relay = RelayConfig {
      domain_name: server.clone(),
      port:(port as u16).into(),
    use_tls: require_tls,
    auth: auth,
    };
  let deliveryconfig = DeliveryConfig::Relay(relay);
  
 
  let mut mailstrom = Mailstrom::new(
    Config {
      helo_name: server,
      delivery: deliveryconfig,
      ..Default::default()
    },
    MemoryStorage::new(),
  );

  // We must explicitly tell mailstrom to start actually sending emails.  If we
  // were only interested in reading the status of previously sent emails, we
  // would not send this command.
  mailstrom.start().unwrap();

  let message_id = mailstrom.send_email(email).unwrap();
  let status = mailstrom.query_status(&*message_id).unwrap();
  println!("{:?}", status);
}
