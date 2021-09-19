#[macro_use]
extern crate napi_derive;

use napi::{Env, JsObject, Result};

#[cfg(all(
	any(windows, unix),
	target_arch = "x86_64",
	not(target_env = "musl"),
	not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[module_exports]
fn init(mut exports: JsObject, env: Env) -> Result<()> {
	exports.set_named_property("TODO", env.create_string("TODO")?)?;
	Ok(())
}
