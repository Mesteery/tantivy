declare module 'tanvity' {
	type Stored = 1;
	type Indexed = 2;
	type StoredIndexed = 3;
	type Fast = 4;
	type StoredFast = 5;
	type IndexedFast = 6;
	type StoredIndexedFast = 7;
	type Text = 8;
	type StoredText = 9;
	type String = 16;
	type StoredString = 17;
	export const STORED: Stored;
	export const INDEXED: Indexed;
	export const FAST: Fast;
	export const TEXT: Text;
	// eslint-disable-next-line @typescript-eslint/ban-types
	export const STRING: String;

	type Brand<T, I> = T & { readonly '': I };

	const fieldBrand: unique symbol;
	type Field = Brand<number, typeof fieldBrand>;

	type AddField<T extends number> = (name: string, options: T) => Field;
	type IntOptions = Stored | Indexed | Fast | StoredFast | StoredIndexed | IndexedFast | StoredIndexedFast;

	const facetBrand: unique symbol;
	type Facet = Brand<[0, string], typeof facetBrand>;
	export const facet: (value: string) => Facet;

	const u64Brand: unique symbol;
	type U64 = Brand<[1, bigint], typeof u64Brand>;
	export const u64: (value: number | bigint) => U64;

	const i64Brand: unique symbol;
	type I64 = Brand<[2, bigint], typeof i64Brand>;
	export const i64: (value: number | bigint) => I64;

	const f64Brand: unique symbol;
	type F64 = Brand<[3, number], typeof f64Brand>;
	export const f64: (value: number) => F64;

	export class SchemaBuilder {
		// eslint-disable-next-line @typescript-eslint/ban-types
		public addTextField: AddField<Stored | Text | String | StoredText | StoredString>;
		public addU64Field: AddField<IntOptions>;
		public addI64Field: AddField<IntOptions>;
		public addF64Field: AddField<IntOptions>;
		public addBytesField: AddField<IntOptions>;
		public addDateField: AddField<IntOptions>;
		public addFacetField: AddField<Stored | Indexed | StoredIndexed>;

		public build(): Schema;
	}

	class FieldEntry {
		private constructor();
		public get name(): string;
		public get type(): number;
		public get indexed(): number;
		public get fast(): number;
		public get stored(): number;
		// TODO: replace these:
		public static fromJson(json: string): FieldEntry;
		public toJson(): string;
		// by these:
		// public static fromJSON(json: Record<string, TODO>): FieldEntry;
		// public toJSON(): Record<string, TODO>;
	}

	export class Schema {
		private constructor();
		public static builder(): SchemaBuilder;

		public getField(name: string): Field | undefined;
		public getFieldEntry(field: Field): FieldEntry;
		public getFieldName(field: Field): string;
		public get fields(): [Field, FieldEntry][];
	}

	class Index {
		private constructor();
		public static createInDir(path: string, schema: Schema): Index;
		public static createInMemory(schema: Schema): Index;

		public writer(budget: number): IndexWriter;
	}

	class IndexWriter {
		private constructor();
		public addDocument(
			document: Record<string, string | Date | Uint8Array | ArrayBuffer | number | Facet | U64 | I64 | F64>,
		): void;
		public commit(): void;
		public reader(options: { reloadPolicy: 'manual' | 'oncommit'; numSearchers: number }): IndexReader;
	}

	// TODO
	class IndexReader {
		private constructor();
		public reload(): void;
		public searcher(): Searcher;
	}

	// TODO
	class QueryParser {
		private constructor();
		public static forIndex(index: Index, fields: Field[]): QueryParser;
		public parseQuery(query: string): Query;
	}

	// TODO
	class Query {
		private constructor();
	}

	// TODO
	// eslint-disable-next-line @typescript-eslint/no-empty-interface
	export interface Fruit {}

	// TODO
	export interface Scorer {
		score(): number;
	}

	// TODO
	export interface Explanation {
		value(): number;
		addDetail(explanation: Explanation): void;
		addContext(context: string): void;
		addConst(name: string, value: number): void;
		toJSON(): Record<string, TODO>;
	}

	// TODO
	export interface Weight {
		scorer(reader: SegmentReader, boost: number): Scorer;
		// eslint-disable-next-line unicorn/prevent-abbreviations
		explain(reader: SegmentReader, doc: number): Explanation;
	}

	// TODO
	type TODO = unknown;
	export interface Collector {
		forSegment(segmentLocalId: number, reader: SegmentReader): SegmentCollector;
		requiresScoring(): boolean;
		mergeFruits(childFruits: [number, DocAddress][][]): Fruit[];
		collectSegment(weigth: TODO, segmentOrd: number, reader: SegmentReader): Fruit;
	}

	export interface SegmentCollector {
		// eslint-disable-next-line unicorn/prevent-abbreviations
		collect(doc: number, score: number): void;
		hervest(): TODO;
	}

	// TODO
	export class SegmentReader {}

	// TODO
	// eslint-disable-next-line unicorn/prevent-abbreviations
	export interface DocAddress {
		segmentOrd: number;
		docId: number;
	}

	// eslint-disable-next-line unicorn/prevent-abbreviations
	export class TopDocs implements Collector {
		private constructor();
		public static withLimit(limit: number | bigint): TopDocs;
		public andOffset(offset: number | bigint): TopDocs;
		public orderByU64Field(field: Field): [bigint, DocAddress][];

		public forSegment(segmentLocalId: number, reader: SegmentReader): SegmentCollector;
		public requiresScoring(): boolean;
		public mergeFruits(childFruits: [number, DocAddress][][]): Fruit[];
		public collectSegment(weigth: TODO, segmentOrd: number, reader: SegmentReader): Fruit;
	}

	class Searcher {
		private constructor();
		public search(query: Query, collector: Collector): TODO;
	}

	// TODO
	export class Document {}
}
