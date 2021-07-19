#!/usr/bin/env node -r ts-node/register --experimental-wasm-bigint
import { inline } from '@tdos/js-sdk-v2/dist'
import fs = require('fs')
import path = require('path')
import { providers, Wallet } from 'ethers'

const abi = fs.readFileSync(path.join(__dirname, '../build/foo.abi.json'), 'ascii')

const entry = `http://${process.env.HOST || 'localhost'}:${process.env.PORT || '7010'}`
const provider = new providers.JsonRpcProvider(entry)

const wallet = new Wallet(<string> process.env.PRIVATE_KEY, provider)

async function deploy() {
    const code = fs.readFileSync(path.join(__dirname, '../build/foo.wasm'))
    const inlined = inline(code, abi, [])
    let r = await wallet.sendTransaction({data : inlined})
    console.log(await r.wait())
}

deploy()
