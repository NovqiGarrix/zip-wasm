# zip-wasm

A WebAssembly module for handling ZIP file operations in the browser, built with Rust and wasm-bindgen.

## Features

- Perform ZIP file operations directly in the browser
- Built with Rust for performance and safety
- WebAssembly integration for native-like speed
- Zero dependencies on server-side processing

## Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/NovqiGarrix/zip-wasm.git
cd zip-wasm
```

2. Build the WebAssembly module:
```bash
wasm-pack build --target web
```

## Usage

1. Usage in Deno:

```typescript
// examples/deno/main.ts
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
```

2. Or you can open the index.html file in your browser and try it yourelf.

## Development

1. Make changes to the Rust code in `src/lib.rs`
2. Rebuild the WebAssembly module:
```bash
wasm-pack build --target web
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.