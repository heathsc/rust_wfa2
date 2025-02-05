#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
include!(concat!(env!("OUT_DIR"), "/bindings_wfa.rs"));

mod aligner;
mod alignment;
mod attributes;
mod cigar;
mod heuristic;
mod plot;
mod profile_timer;