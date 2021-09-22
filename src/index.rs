use napi::{
	CallContext, Either, Env, Error, JsBigint, JsFunction, JsNumber, JsObject, JsString,
	JsUndefined, Property, Result,
};
use std::convert::TryFrom;
use std::path::Path;
use tantivy::{schema::Schema, Index};

use crate::{helpers::get_wrapped_struct, index_writer, schema_builder};

#[inline]
pub fn define_class(env: &Env) -> Result<JsFunction> {
	env.define_class(
		"Index",
		constructor,
		&[
			Property::new(env, "createInDir")?
				.with_method(create_in_dir)
				.with_property_attributes(napi::PropertyAttributes::Static),
			Property::new(env, "createInMemory")?
				.with_method(create_in_memory)
				.with_property_attributes(napi::PropertyAttributes::Static),
		],
	)
}

#[js_function(2)]
pub fn create_in_dir(ctx: CallContext) -> Result<JsObject> {
	let path = ctx.get::<JsString>(0)?.into_utf8()?;
	let schema = ctx.env.unwrap::<Schema>(&ctx.get::<JsObject>(1)?)?;
	let mut instance = schema_builder::define_class(ctx.env)?.new::<JsUndefined>(&[])?;
	ctx.env.wrap(
		&mut instance,
		Index::create_in_dir(Path::new(path.as_str()?), schema.clone()),
	)?;
	Ok(instance)
}

#[js_function(1)]
pub fn create_in_memory(ctx: CallContext) -> Result<JsObject> {
	let schema = ctx.env.unwrap::<Schema>(&ctx.get::<JsObject>(0)?)?;
	let mut instance = schema_builder::define_class(ctx.env)?.new::<JsUndefined>(&[])?;
	ctx.env
		.wrap(&mut instance, Index::create_in_ram(schema.clone()))?;
	Ok(instance)
}

#[js_function]
pub fn constructor(ctx: CallContext) -> Result<JsUndefined> {
	ctx.env.get_undefined()
}

#[js_function(1)]
pub fn writer(ctx: CallContext) -> Result<JsObject> {
	let size = match ctx.get::<Either<JsNumber, JsBigint>>(0)? {
		Either::A(size) => size.get_uint32()? as usize,
		Either::B(size) => u64::try_from(size)? as usize,
	};
	let index = get_wrapped_struct::<Index>(&ctx)?;
	let index_writer = index
		.writer(size)
		.map_err(|e| Error::from_reason(e.to_string()))?;
	let index_writer_class = index_writer::define_class(ctx.env)?;
	let mut instance = index_writer_class.new::<JsUndefined>(&[])?;
	ctx.env.wrap(&mut instance, index_writer)?;
	Ok(instance)
}
