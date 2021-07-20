#!/usr/bin/env node
const { compileRust } = require('@tdos/js-sdk-v2/dist')
const fs = require('fs')
const path = require('path')
const abi = compileRust(fs.readFileSync(path.join(__dirname, '../src/lib.rs'), 'utf-8'))
fs.writeFileSync(path.join(__dirname, '../build/foo.abi.json'), JSON.stringify(abi, null, 2))
