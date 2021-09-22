use crate::schema as schema_binding;

use napi::{
	CallContext, Env, Error, JsFunction, JsNumber, JsObject, JsString, JsUndefined, Property,
	Result,
};
use tantivy::schema::{
	self, BytesOptions, FacetOptions, IntOptions, Schema, SchemaBuilder, TextOptions,
};

use crate::helpers::get_wrapped_struct;

pub const STORED: u8 = 1 << 0;
pub const INDEXED: u8 = 1 << 1;
pub const FAST: u8 = 1 << 2;
pub const TEXT: u8 = 1 << 3;
pub const STRING: u8 = 1 << 4;

#[inline]
pub fn define_class(env: &Env) -> Result<JsFunction> {
	env.define_class(
		"SchemaBuilder",
		constructor,
		&[
			Property::new(env, "addTextField")?.with_method(add_text_field),
			Property::new(env, "addU64Field")?.with_method(add_u64_field),
			Property::new(env, "addI64Field")?.with_method(add_i64_field),
			Property::new(env, "addF64Field")?.with_method(add_f64_field),
			Property::new(env, "addBytesField")?.with_method(add_bytes_field),
			Property::new(env, "addDateField")?.with_method(add_date_field),
			Property::new(env, "addFacetField")?.with_method(add_facet_field),
			Property::new(env, "build")?.with_method(build),
		],
	)
}

#[js_function]
pub fn constructor(ctx: CallContext) -> Result<JsUndefined> {
	let mut this: JsObject = ctx.this_unchecked();
	ctx.env.wrap(&mut this, Some(Schema::builder()))?;
	ctx.env.get_undefined()
}

macro_rules! add_field_binding {
	($name:ident, $T: ident, $( $x:ident ), *) => {
		#[js_function(2)]
		pub fn $name(ctx: CallContext) -> Result<JsNumber> {
			let field_name = ctx.get::<JsString>(0)?.into_utf8()?;
			let field_options_ = ctx.get::<JsNumber>(1)?.get_uint32()? as u8;
			let mut field_options = $T::default();
			$(
				if field_options_ & $x == 0 {	field_options = field_options | schema::$x; }
			)*
			let schema_builder = get_wrapped_struct::<Option<SchemaBuilder>>(&ctx)?.as_mut().ok_or(Error::from_reason(
				"SchemaBuilder has already been built.".to_string(),
			))?;
			let field = schema_builder.$name(field_name.as_str()?, field_options);
			ctx.env.create_uint32(field.field_id())
		}
	};
}

add_field_binding!(add_text_field, TextOptions, STORED, TEXT, STRING);
add_field_binding!(add_u64_field, IntOptions, STORED, INDEXED, FAST);
add_field_binding!(add_i64_field, IntOptions, STORED, INDEXED, FAST);
add_field_binding!(add_f64_field, IntOptions, STORED, INDEXED, FAST);
add_field_binding!(add_bytes_field, BytesOptions, STORED, INDEXED, FAST);
add_field_binding!(add_date_field, IntOptions, STORED, INDEXED, FAST);
add_field_binding!(add_facet_field, FacetOptions, STORED, INDEXED);

#[js_function]
pub fn build(ctx: CallContext) -> Result<JsObject> {
	let schema_builder = get_wrapped_struct::<Option<SchemaBuilder>>(&ctx)?;
	let schema = schema_builder
		.take()
		.ok_or(Error::from_reason(
			"SchemaBuilder has been built.".to_string(),
		))?
		.build();
	let mut instance = schema_binding::define_class(ctx.env)?.new::<JsUndefined>(&[])?;
	ctx.env.wrap(&mut instance, schema)?;
	Ok(instance)
}
