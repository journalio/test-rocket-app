#![feature(decl_macro)]
#![forbid(unsafe_code)]
#![cfg_attr(feature = "strict", deny(warnings))]

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod controllers;
pub mod database;
pub mod models;
pub mod repositories;
pub mod schema;
pub(crate) use database::*;
