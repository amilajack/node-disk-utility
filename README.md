# disk-utility

[![Build Status](https://travis-ci.org/amilajack/disk-utility.svg?branch=master)](https://travis-ci.org/amilajack/disk-utility)

## Installation

```bash
npm install disk-utility
```

## Usage

```js
import diskUtility from 'disk-utility';

// calculate the size of a disk in parallel
diskUtility.dirSize(__dirname);

// Pass optional core count
diskUtility.dirSize(__dirname, 4);
```
