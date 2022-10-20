use std::path::PathBuf;

pub use rspc::{Config, RouterBuilder};

#[derive(Clone)]
pub struct Ctx {}

pub type Router = rspc::Router<Ctx>;

pub(crate) fn new() -> RouterBuilder<Ctx> {
    Router::new()
        .config(
            Config::new()
                // TODO: This messes with Tauri's hot reload so we can't use it until their is a solution upstream. https://github.com/tauri-apps/tauri/issues/4617
                .export_ts_bindings(
                    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../src/types/rspc/index.ts"),
                )
                .set_ts_bindings_header("/* eslint-disable */"),
        )
        .query("version", |t| t(|_, _: ()| "teste"))
}
