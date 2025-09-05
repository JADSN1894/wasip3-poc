mod error;

use crate::error::AppResult;
use std::fmt::Debug;
use wasmtime::{
    Config, Engine, Store,
    component::{Component, Linker, ResourceTable, bindgen},
};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView, p3::bindings::LinkOptions};

// bindgen!({world : "toggler", path: "../wit"});
bindgen!("toggler" in "../wit/toggler.wit");

pub struct ComponentRunStates {
    wasi_ctx: WasiCtx,
    resource_table: ResourceTable,
    store: Store<()>,
}

impl Debug for ComponentRunStates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowStates")
            .field("table", &self.resource_table)
            .field("store", &self.store)
            .finish()
    }
}

impl WasiView for ComponentRunStates {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi_ctx,
            table: &mut self.resource_table,
        }
    }
}

#[tokio::main]
// #[wit_bindgen::async]
async fn main() -> AppResult<()> {
    let mut config = Config::new();
    config.wasm_component_model_async(true);
    config.wasm_component_model_async_stackful(true);
    config.async_support(true);

    let engine = Engine::new(&config)?;

    let state: ComponentRunStates = ComponentRunStates {
        wasi_ctx: WasiCtxBuilder::new().inherit_stdio().inherit_args().build(),
        resource_table: ResourceTable::new(),
        store: Store::<()>::default(),
    };

    let mut store = Store::new(&engine, state);
    let component = Component::from_file(&engine, "wasm/plugin.wasm")?;

    let mut linker = Linker::new(&engine);

    let _ = linker.define_unknown_imports_as_traps(&component)?;
    // let mut linker =   linker.allow_shadowing(true);

    let link_options = LinkOptions::default();
    wasmtime_wasi::p3::add_to_linker_with_options(&mut linker, &link_options)?;

    let instance = linker.instantiate_async(&mut store, &component).await?;
    let func =
        instance.get_typed_func::<(TogglerWrapper,), (TogglerWrapper,)>(&mut store, "toggle")?; // .ok_or(AppError::Custom("Error on get_func".into()))?;

    let (output,) = func
        .call_async(&mut store, (TogglerWrapper { inner: false },))
        .await?;

    println!("Output: {output:?}");

    Ok(())
}
