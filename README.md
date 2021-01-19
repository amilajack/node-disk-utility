# disk-utility

![CI](https://github.com/amilajack/node-disk-utility/workflows/CI/badge.svg)

Quickly calculate the size of a folder or disk on macOS and Linux

## Features

* Does not depend on system dependencies (like [`du`](https://en.wikipedia.org/wiki/Du_(Unix)))
* Built using [Neon](https://github.com/neon-bindings/neon)!

## Installation

```bash
npm install disk-utility
```

## Usage

```js
import diskUtility from 'disk-utility';

// Calculate the size of a directory
diskUtility.dirSize(__dirname);

// Pass optional core count
diskUtility.dirSize(__dirname, 4);
```

## Local Setup

```bash
git clone https://github.com/amilajack/disk-utility
cd disk-utility
npm install
npm test
```
