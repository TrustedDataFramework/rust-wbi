# sm-crypto

## webpack

1. allow .wasm module by experiments.asyncWebAssembly = true

```js
const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')

module.exports = {
  mode: 'development',
  devServer: {
    contentBase: path.join(__dirname, 'dist'),
    compress: true,
  },  
  entry: './src/index.js', // input file of the JS bundle
  output: {
    filename: 'bundle.js', // output filename
    path: path.resolve(__dirname, 'dist'), // directory of where the bundle will be created at
  },
  experiments: {
    asyncWebAssembly: true
  },  
  plugins: [
    new HtmlWebpackPlugin()
  ],
}
```

1. require this module async in your .js file

```js
async function main() {
    const libsm = await require('@tdos/sm-crypto/sm_crypto')
    // generate private key 
    // 通过 sm3 生成随机私钥, 输入值是随机的 0x 开头的十六进制串，
    const privateKey = libsm.sm3('0x00')
    console.log(`privateKey = ${privateKey}`)

    // 将私钥转成公钥匙并且压缩
    const publicKey = libsm.sm2_pk_from_sk(privateKey, true)
    console.log(`publicKey = ${publicKey}`)

    // sign
    // 对消息内容进行签名，消息内容必须是 0x 开头的十六进制串
    const msg = '0xffff'
    // 签名需要生成随机种子
    const seed = 128n
    const sig = libsm.sm2_sign(seed, privateKey, msg)
    console.log(`sig = ${sig}`)

    // verify
    // 验证签名, 验证签名也需要随机种子
    const verified = libsm.sm2_verify(seed + 2n, msg, publicKey, sig)
    console.log(`verify result = ${verified}`)
}


main()
```
