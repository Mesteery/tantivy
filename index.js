import { fileURLToPath } from 'node:url';
import { loadBinding } from '@node-rs/helper';

export default loadBinding(fileURLToPath(new URL('.', import.meta.url)), 'tantivy', 'tantivy');
