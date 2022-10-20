use std::{path::PathBuf, sync::Arc};

pub use rspc::{Config, RouterBuilder};

use crate::prisma;

// #[derive(Clone)]
pub type Ctx = Arc<prisma::PrismaClient>;

pub type Router = rspc::Router<Ctx>;

pub(crate) fn new() -> RouterBuilder<Ctx> {
    let config = Config::new()
        // TODO: This messes with Tauri's hot reload so we can't use it until their is a solution upstream. https://github.com/tauri-apps/tauri/issues/4617
        .export_ts_bindings(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../src/types/rspc/index.ts"),
        )
        .set_ts_bindings_header("/* eslint-disable */");

    Router::new()
        .config(config)
        // .query("version", |t| t(|ctx, _: ()| async move {
        //     ctx.db.
        // }))
        .query("version", |t| t(|_, _: ()| "teste"))
}
