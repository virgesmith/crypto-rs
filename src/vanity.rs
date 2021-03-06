
use crate::key::Key;
use crate::address;
use crate::error::Error;
use crate::CryptoResult;

use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::sync::{Arc, Condvar};

pub fn search(pattern: String, threads: usize) -> CryptoResult<(Key, usize)> {

  // TODO is there a better way to test validity?
  match bs58::decode(&pattern).into_vec() {
    Ok(_) => (),
    Err(_) => { return Err(Box::new(Error::InvalidBase58Digits(pattern))); }
  }

  // be realistic: 58^8 > 1e14
  if pattern.len() > 7 {
    return Err(Box::new(Error::SearchStringTooLong(pattern)));
  }

  openssl::init();

  let pair = Arc::new((AtomicBool::new(false), Condvar::new()));
  let pattern = Arc::new(pattern);

  let mut handles = vec![];

  for _ in 0..threads {
    let p = pair.clone();
    let v = pattern.clone();
    handles.push( thread::spawn(move || { worker(v, p) }));
  }

  let mut k = Key::new()?;
  let mut total_tries = 0;
  for (_, e) in handles.into_iter().enumerate() {
    let result = e.join().unwrap();
    total_tries += result.1;
    match result.0 {
      Some(r) => k = r,
      None => continue
    }
  }
  Ok((k, total_tries))
}

fn worker(pattern: Arc<String>, pair: Arc<(AtomicBool, Condvar)>) -> (Option<Key>, usize) {

  let &(ref found, ref cvar) = &*pair;

  let mut i = 0;

  loop {

    let key = Key::new().unwrap();

    let bytes = key.compressed_public_key().unwrap();

    let addr = address::p2pkh(&bytes);
    let cmp = &addr[1..pattern.len()+1];
    i += 1;
    if *pattern == cmp {
      found.store(true, Ordering::Relaxed);
      cvar.notify_all();
      return (Some(key), i);
    }
    if found.load(Ordering::Relaxed) {
      return (None, i);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test1() {
    let (k, _) = search("A".to_string(), 1).unwrap();
    let a = address::p2pkh(&k.compressed_public_key().unwrap());
    assert_eq!(&a[..2], "1A");
    let (k, _) = search("BB".to_string(), 4).unwrap();
    let a = address::p2pkh(&k.compressed_public_key().unwrap());
    assert_eq!(&a[..3], "1BB");
  }

  #[test]
  fn test_failures() {
    // invalid
    match search("0".to_string(), 1) {
      Ok(_) => assert!(false, "invalid base 58 digit should fail"),
      Err(e) => assert_eq!(e.to_string(), "invalid search string: 0")
    }
    match search("AAAAAAAAAAAA".to_string(), 1) {
      Ok(_) => assert!(false, "too long search string should fail"),
      Err(e) => assert_eq!(e.to_string(), "search string is too long: AAAAAAAAAAAA")
    }
  }
}
