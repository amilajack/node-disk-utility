# disk-utility

[![Build Status](https://travis-ci.org/amilajack/disk-utility.svg?branch=master)](https://travis-ci.org/amilajack/disk-utility)

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
yarn
yarn test
```
