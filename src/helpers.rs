use napi::{CallContext, JsObject, Result};

#[inline]
pub fn get_wrapped_struct<'a, T: 'static>(ctx: &'a CallContext) -> Result<&'a mut T> {
	ctx.env.unwrap::<T>(&ctx.this_unchecked::<JsObject>())
}
