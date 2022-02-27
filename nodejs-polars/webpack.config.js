import NodePolyfillPlugin from "node-polyfill-webpack-plugin"
import {dirname, resolve} from 'path';

const __dirname = dirname(fileURLToPath(import.meta.url));

import {createRequire} from 'module';
import {fileURLToPath} from 'url';
const require = createRequire(import.meta.url);
export default {
  devtool: 'source-map',
  entry: './bin/index.js',
  module: {
    rules: [
      {
        test: /\.ts?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
      {
        test: /\.wasm$/,
        type: "webassembly/async"
      }
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
    fallback: {
      "fs": false,
      util: false,
      module: require.resolve("module"),
      buffer: require.resolve('buffer'),
      console: require.resolve('console-browserify'),
      path: require.resolve('path-browserify'),
      stream: require.resolve('stream-browserify'),
      url: require.resolve('url'),
    }
  },
  experiments: {
    asyncWebAssembly: true,
    futureDefaults: true,
    outputModule: true
  },
  output: {
    library: {
      type: 'module',
    },
    filename: "polars.js",
    path: resolve(__dirname, 'browser/dist/polars'), // directory of where the bundle will be created at
  },
  plugins: [
    new NodePolyfillPlugin()
  ]
};