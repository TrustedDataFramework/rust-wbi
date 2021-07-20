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
  // module: {
  //   rules: [
  //     {
  //       test: /\.wasm$/,
  //       type: 'webassembly/sync',
  //     }
  //   ]
  // },
  experiments: {
    asyncWebAssembly: true
  },  
  plugins: [
    new HtmlWebpackPlugin()
  ],
}