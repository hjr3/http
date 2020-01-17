#![no_main]
use libfuzzer_sys::fuzz_target;
extern crate http;

use http::header::*;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = s.parse::<HeaderName>();
    }
});

/*
   let mut headers = HeaderMap::new();

    match headers.entry(&name) {
        Entry::Vacant(e) => {
            e.insert("world".parse().unwrap());
        }
        _ => panic!(),
    }

    assert!(headers.get("hello").is_some());

    match headers.entry(&name) {
        Entry::Occupied(mut e) => {
            assert_eq!(e.get(), &"world");

            // Push another value
            e.append("zomg".parse().unwrap());

            let mut i = e.iter();

            assert_eq!(*i.next().unwrap(), "world");
            assert_eq!(*i.next().unwrap(), "zomg");
            assert!(i.next().is_none());
        }
        _ => panic!(),
    }
});
*/
