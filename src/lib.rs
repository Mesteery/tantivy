#[macro_use]
extern crate napi_derive;

mod field_entry;
mod helpers;
mod index;
mod index_writer;
mod schema;
mod schema_builder;

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
pub fn init(mut exports: JsObject, env: Env) -> Result<()> {
	exports.set_named_property("SchemaBuilder", schema_builder::define_class(&env)?)?;
	exports.set_named_property("Schema", schema::define_class(&env)?)?;

	exports.set_named_property("STORED", env.create_uint32(schema_builder::STORED as u32)?)?;
	exports.set_named_property(
		"INDEXED",
		env.create_uint32(schema_builder::INDEXED as u32)?,
	)?;
	exports.set_named_property("FAST", env.create_uint32(schema_builder::FAST as u32)?)?;
	exports.set_named_property("TEXT", env.create_uint32(schema_builder::TEXT as u32)?)?;
	exports.set_named_property("STRING", env.create_uint32(schema_builder::STRING as u32)?)?;

	Ok(())
}
