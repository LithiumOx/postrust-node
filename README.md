# @postrust/lookup

Ultra-fast Dutch postcode lookup with embedded data. This package provides lightning-fast postcode and address lookups for the Netherlands, with all data embedded directly in the native binary.

## 🚀 Features

- **Blazing Fast**: Sub-millisecond lookup times using Finite State Transducers (FST)
- **Self-Contained**: All data embedded in the binary - no external files needed
- **Tiny Footprint**: Only ~4MB compressed data using brotli compression
- **Memory Efficient**: ~15MB RAM usage with intelligent data structures
- **TypeScript Support**: Full TypeScript definitions included
- **Cross-Platform**: Pre-built binaries for Windows, macOS, and Linux

## 📦 Installation

```bash
npm install @postrust/lookup
```

## 🔧 Usage

### Basic Lookup

```javascript
const { lookup } = require('@postrust/lookup');

const result = lookup('1012AB', 1);
if (result) {
  console.log(`${result.straat} ${result.huisnummer}, ${result.woonplaats}`);
  // Output: Nieuwezijds Voorburgwal 1, Amsterdam
}
```

### TypeScript

```typescript
import { lookup, LookupResult } from '@postrust/lookup';

const result: LookupResult | null = lookup('1012AB', 1);
if (result) {
  console.log(`Found: ${result.postcode} - ${result.straat} ${result.huisnummer}, ${result.woonplaats}`);
}
```

### Batch Lookups

```javascript
const { lookupBatch } = require('@postrust/lookup');

const queries = [
  ['1012AB', 1],
  ['2000AA', 10],
  ['3000BB', 5]
];

const results = lookupBatch(queries);
results.forEach((result, i) => {
  if (result) {
    console.log(`${queries[i][0]} → ${result.straat} ${result.huisnummer}, ${result.woonplaats}`);
  } else {
    console.log(`${queries[i][0]} → Not found`);
  }
});
```

### Package Information

```javascript
const { getInfo } = require('@postrust/lookup');

console.log(getInfo());
// Output:
// postRUST NPM Package
// Memory usage: 14.75 MB  
// Compressed data size: 3.94 MB
```

## 📋 API Reference

### `lookup(postcode: string, huisnummer: number): LookupResult | null`

Lookup a single postcode and house number combination.

**Parameters:**
- `postcode` - Dutch postcode (e.g., "1012AB")
- `huisnummer` - House number (e.g., 1)

**Returns:** `LookupResult` object or `null` if not found

### `lookupBatch(queries: Array<[string, number]>): Array<LookupResult | null>`

Lookup multiple postcode/house number combinations in a single call.

**Parameters:**
- `queries` - Array of `[postcode, huisnummer]` tuples

**Returns:** Array of `LookupResult` objects or `null` for each query

### `LookupResult`

```typescript
interface LookupResult {
  postcode: string;    // "1012AB"
  straat: string;      // "Nieuwezijds Voorburgwal"  
  huisnummer: number;  // 1
  woonplaats: string;  // "Amsterdam"
}
```

### `getInfo(): string`

Get information about the loaded dataset and memory usage.

### `init(): void`

Initialize the package (called automatically when the module is loaded).

## ⚡ Performance

- **Lookup Speed**: Sub-millisecond response times
- **Memory Usage**: ~15MB RAM (data decompressed in memory)
- **Binary Size**: ~5MB (includes all data and dependencies)
- **Data Compression**: 73% reduction using brotli compression
- **Cold Start**: Near-instant (data already embedded)

## 🏗️ Architecture

This package uses several advanced techniques for optimal performance:

1. **Finite State Transducers (FST)**: Extremely efficient data structure for string lookups
2. **Delta Compression**: House numbers are delta-compressed to save space
3. **Brotli Compression**: All data is compressed with maximum brotli settings
4. **Native Code**: Core logic implemented in Rust for maximum speed
5. **Embedded Data**: No file I/O during runtime - everything is in memory

## 🔧 Building from Source

```bash
# Clone the repository
git clone https://github.com/your-username/postrust.git
cd postrust/postrust-npm

# Install dependencies
npm install

# Build the native module
npm run build

# Test
node test.js
```

## 📊 Data Source

The postcode data is sourced from official Dutch postal databases and includes:
- All valid Dutch postcodes
- Street names and house numbers
- Municipality information
- Regular updates to ensure data accuracy

## ⚖️ License

MIT License - see LICENSE file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---

**Note**: This package is optimized for server-side Node.js applications. For browser usage, consider using the REST API version instead due to the large data size.
