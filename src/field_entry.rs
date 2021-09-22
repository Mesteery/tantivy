use napi::{
	CallContext, Env, Error, JsBoolean, JsFunction, JsObject, JsString, JsUndefined, Property,
	PropertyAttributes, Result,
};
use serde_json;
use tantivy::schema::{FieldEntry, FieldType};

use crate::helpers::get_wrapped_struct;

#[inline]
pub fn define_class(env: &Env) -> Result<JsFunction> {
	env.define_class(
		"FieldEntry",
		constructor,
		&[
			Property::new(env, "name")?.with_getter(name),
			Property::new(env, "type")?.with_getter(field_type),
			Property::new(env, "indexed")?.with_getter(is_indexed),
			Property::new(env, "fast")?.with_getter(is_fast),
			Property::new(env, "stored")?.with_getter(is_stored),
			Property::new(env, "fromJson")?
				.with_method(from_json)
				.with_property_attributes(PropertyAttributes::Static),
			Property::new(env, "toJson")?.with_method(to_json),
		],
	)
}

#[js_function]
pub fn is_stored(ctx: CallContext) -> Result<JsBoolean> {
	let this = ctx.this_unchecked::<JsObject>();
	let field_entry = ctx.env.unwrap::<FieldEntry>(&this)?;
	ctx.env.get_boolean(field_entry.is_stored())
}

#[js_function]
pub fn is_indexed(ctx: CallContext) -> Result<JsBoolean> {
	let this = ctx.this_unchecked::<JsObject>();
	let field_entry = ctx.env.unwrap::<FieldEntry>(&this)?;
	ctx.env.get_boolean(field_entry.is_indexed())
}

#[js_function]
pub fn is_fast(ctx: CallContext) -> Result<JsBoolean> {
	let this = ctx.this_unchecked::<JsObject>();
	let field_entry = ctx.env.unwrap::<FieldEntry>(&this)?;
	ctx.env.get_boolean(field_entry.is_fast())
}

#[js_function]
pub fn name(ctx: CallContext) -> Result<JsString> {
	let this = ctx.this_unchecked::<JsObject>();
	let field_entry = ctx.env.unwrap::<FieldEntry>(&this)?;
	ctx.env.create_string(field_entry.name())
}

#[js_function]
pub fn field_type(ctx: CallContext) -> Result<JsString> {
	let this = ctx.this_unchecked::<JsObject>();
	let field_entry = ctx.env.unwrap::<FieldEntry>(&this)?;
	ctx.env.create_string(match field_entry.field_type() {
		FieldType::Bytes(_) => "bytes",
		FieldType::Date(_) => "date",
		FieldType::F64(_) => "f64",
		FieldType::Str(_) => "text",
		FieldType::U64(_) => "u64",
		FieldType::I64(_) => "i64",
		FieldType::HierarchicalFacet(_) => "hierarchical_facet",
	})
}

#[js_function]
pub fn constructor(ctx: CallContext) -> Result<JsUndefined> {
	ctx.env.get_undefined()
}

#[js_function(1)]
pub fn from_json(ctx: CallContext) -> Result<JsObject> {
	let json = ctx.get::<JsString>(0)?.into_utf8()?;
	let field_entry =
		serde_json::from_str(json.as_str()?).map_err(|e| Error::from_reason(e.to_string()))?;
	let field_entry_class = define_class(ctx.env)?;
	let mut instance = field_entry_class.new::<JsUndefined>(&[])?;
	ctx.env.wrap(&mut instance, field_entry)?;
	Ok(instance)
}

#[js_function(1)]
pub fn to_json(ctx: CallContext) -> Result<JsString> {
	let field_entry = get_wrapped_struct::<FieldEntry>(&ctx)?;
	ctx.env.create_string_from_std(
		serde_json::to_string(field_entry).map_err(|e| Error::from_reason(e.to_string()))?,
	)
}
