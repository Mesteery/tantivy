import { fileURLToPath } from 'node:url';
import { loadBinding } from '@node-rs/helper';

/**
 * @type {import('tanvity')}
 */
const binding = loadBinding(fileURLToPath(new URL('.', import.meta.url)), 'tantivy', 'tantivy');

export const facet = (value) => [0, value];
export const u64 = (value) => [
	1,
	// eslint-disable-next-line no-nested-ternary
	typeof value === 'number'
		? BigInt.asUintN(64, BigInt(value))
		: typeof value === 'bigint'
		? BigInt.asUintN(64, value)
		: value,
];
export const i64 = (value) => [
	2,
	// eslint-disable-next-line no-nested-ternary
	typeof value === 'number'
		? BigInt.asIntN(64, BigInt(value))
		: typeof value === 'bigint'
		? BigInt.asIntN(64, value)
		: value,
];
export const f64 = (value) => [3, value];

export const { Schema, SchemaBuilder, STORED, INDEXED, FAST, TEXT, STRING } = binding;
