use napi::{
	CallContext, Env, Error, JsArrayBuffer, JsBigint, JsDate, JsFunction, JsNumber, JsObject,
	JsString, JsUndefined, JsUnknown, Property, Result, Status, ValueType,
};
use std::convert::TryInto;
use tantivy::{
	chrono::{TimeZone, Utc},
	schema::{Facet, FieldValue, Value},
	DateTime, Document, IndexWriter,
};

use crate::helpers::get_wrapped_struct;

#[inline]
pub fn define_class(env: &Env) -> Result<JsFunction> {
	env.define_class(
		"IndexWriter",
		constructor,
		&[Property::new(env, "addDocument")?.with_method(add_document)],
	)
}

#[js_function]
pub fn constructor(ctx: CallContext) -> Result<JsUndefined> {
	ctx.env.get_undefined()
}

#[js_function(1)]
pub fn add_document(ctx: CallContext) -> Result<JsUndefined> {
	let doc = ctx.get::<JsObject>(0)?;
	let index_writer = get_wrapped_struct::<IndexWriter>(&ctx)?;
	let mut document = Document::default();
	let keys = doc.get_property_names()?;
	for idx in 0..keys.get_array_length()? {
		let value = doc.get_element::<JsUnknown>(idx)?;
		let key = keys.get_element::<JsString>(idx)?.into_utf8()?;
		let key = key.as_str()?;
		let field_value = match value.get_type()? {
			ValueType::String => {
				let value = unsafe { value.cast::<JsString>() }.into_utf8()?;
				Value::Str(value.as_str()?.to_string())
			}
			ValueType::Number => Value::F64(unsafe { value.cast::<JsNumber>() }.get_double()?),
			ValueType::Object => {
				if value.is_date()? {
					let date = unsafe { value.cast::<JsDate>() }.value_of()? * 1_000_000.0;
					Value::Date(DateTime::from(Utc.timestamp_nanos(date as i64)))
				} else if value.is_typedarray()? {
					let array_buffer = unsafe { value.cast::<JsArrayBuffer>() };
					Value::Bytes(array_buffer.into_value()?.to_vec())
				} else if value.is_array()? {
					let array = unsafe { value.cast::<JsObject>() };
					if array.get_array_length()? == 2 {
						let field_type = array.get_element::<JsNumber>(0)?.get_uint32()?;
						match field_type {
							0 /* facet */ => {
								let value = array.get_element::<JsString>(1)?.into_utf8()?;
								Value::Facet(
									Facet::from_text(value.as_str()?)
										.map_err(|e| Error::from_reason(e.to_string()))?,
								)
							}
							1 /* u64 */ => {
								let value: u64 = array.get_element::<JsBigint>(1)?.try_into()?;
								Value::U64(value)
							}
							2 /* i64 */ => {
								let value: i64 = array.get_element::<JsBigint>(1)?.try_into()?;
								Value::I64(value)
							}
							3 /* f64 */ => Value::F64(unsafe { value.cast::<JsNumber>() }.get_double()?),
							_ => return Err(Error::from_status(Status::InvalidArg)),
						}
					} else {
						return Err(Error::from_status(Status::InvalidArg));
					}
				} else {
					return Err(Error::from_status(Status::InvalidArg));
				}
			}
			_ => return Err(Error::from_status(Status::InvalidArg)),
		};
		document.add(FieldValue::new(
			index_writer
				.index()
				.schema()
				.get_field(key)
				.ok_or_else(|| Error::from_reason(format!("Unknown schema field {}.", key)))?,
			field_value,
		));
	}
	ctx.env.get_undefined()
}
