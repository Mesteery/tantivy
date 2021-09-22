use napi::{
	CallContext, Env, JsFunction, JsNumber, JsObject, JsString, JsUndefined, JsUnknown, Property,
	Result,
};
use tantivy::schema::{Field, FieldEntry, Schema};

use crate::{field_entry, helpers::get_wrapped_struct, schema_builder};

#[inline]
pub fn define_class(env: &Env) -> Result<JsFunction> {
	env.define_class(
		"Schema",
		constructor,
		&[
			Property::new(env, "builder")?
				.with_method(builder)
				.with_property_attributes(napi::PropertyAttributes::Static),
			Property::new(env, "fields")?.with_getter(fields),
			Property::new(env, "getField")?.with_method(get_field),
			Property::new(env, "getFieldEntry")?.with_method(get_field_entry),
			Property::new(env, "getFieldName")?.with_method(get_field_name),
		],
	)
}

#[js_function]
pub fn builder(ctx: CallContext) -> Result<JsObject> {
	schema_builder::define_class(ctx.env)?.new::<JsUndefined>(&[])
}

#[js_function]
pub fn constructor(ctx: CallContext) -> Result<JsUndefined> {
	ctx.env.get_undefined()
}

#[js_function(1)]
pub fn get_field(ctx: CallContext) -> Result<JsUnknown> {
	let field_name = ctx.get::<JsString>(0)?.into_utf8()?;
	let schema = get_wrapped_struct::<Schema>(&ctx)?;
	Ok(
		if let Some(field) = schema.get_field(field_name.as_str()?) {
			JsNumber::into_unknown(ctx.env.create_uint32(field.field_id())?)
		} else {
			JsUndefined::into_unknown(ctx.env.get_undefined()?)
		},
	)
}

#[inline]
fn new_field_entry(env: &Env, field_entry: FieldEntry) -> Result<JsObject> {
	let field_entry_class = field_entry::define_class(env)?;
	let mut instance = field_entry_class.new::<JsUndefined>(&[])?;
	env.wrap(&mut instance, field_entry)?;
	Ok(instance)
}

#[js_function(1)]
pub fn get_field_entry(ctx: CallContext) -> Result<JsObject> {
	let field = Field::from_field_id(ctx.get::<JsNumber>(0)?.get_uint32()?);
	let schema = get_wrapped_struct::<Schema>(&ctx)?;
	let field_entry = schema.get_field_entry(field).clone();
	new_field_entry(ctx.env, field_entry)
}

#[js_function(1)]
pub fn get_field_name(ctx: CallContext) -> Result<JsString> {
	let field = Field::from_field_id(ctx.get::<JsNumber>(0)?.get_uint32()?);
	let schema = get_wrapped_struct::<Schema>(&ctx)?;
	ctx.env.create_string(schema.get_field_name(field))
}

#[js_function(1)]
pub fn fields(ctx: CallContext) -> Result<JsObject> {
	let schema = get_wrapped_struct::<Schema>(&ctx)?;
	let mut array = ctx.env.create_array()?;
	for field in schema.fields() {
		let mut tuple = ctx.env.create_array_with_length(2)?;
		tuple.set_element(0, ctx.env.create_uint32(field.0.field_id())?)?;
		tuple.set_element(1, new_field_entry(ctx.env, field.1.clone())?)?;
		array.set_element(field.0.field_id(), tuple)?;
	}
	Ok(array)
}
