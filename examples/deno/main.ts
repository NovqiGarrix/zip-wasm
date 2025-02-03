import fs from 'node:fs/promises';
import init, { compress_files_to_zip } from '../../pkg/zip_wasm.js';

await init();

const files = [
    {
        name: 'hello.txt',
        content: new TextEncoder().encode('hello world'),
    },
    {
        name: 'hello2.txt',
        content: new TextEncoder().encode('hello world 2'),
    },
];

const compressedZip = await compress_files_to_zip(files);

await fs.writeFile('hello.zip', compressedZip);