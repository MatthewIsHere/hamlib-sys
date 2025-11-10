#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod lists {
    include!(concat!(env!("OUT_DIR"), "/bindings_lists.rs"));
}