# pdf-extract-node

This is a node.js wrapper for the [pdf-extract](https://github.com/jrmuizel/pdf-extract) library. It is a simple way to extract text from PDFs.

## Installation

```bash
npm install @effishai/pdf-extract-node
```

## Usage

```javascript
import { extractText } from "@effishai/pdf-extract-node";

// Read PDF file as Buffer
const pdfBuffer = fs.readFileSync('path/to/pdf');

// Extract text from the pdf
const text = extractText(pdfBuffer)

// Extract text from the pdf by pages
const textPages = extractTextByPages(pdfBuffer)

```