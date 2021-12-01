const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = {

  webpack: {
    plugins: {
      add: [
        new WasmPackPlugin({
          crateDirectory: path.resolve(__dirname, './src/rust'),
          outDir: path.resolve(__dirname, "./src/rust/pkg")

        })
      ]
    },
    configure: (webpackPlugin) => {
      webpackPlugin.resolve.extensions.push('.wasm');

      webpackPlugin.module.rules.forEach(rule => {
        (rule.oneOf || []).forEach(oneOf => {
          if (oneOf.loader && oneOf.loader.indexOf("file-loader") >= 0) {
            // Make file-loader ignore WASM files
            oneOf.exclude.push(/\.wasm$/);
          }
        });
      });
      return webpackPlugin;
    }
  }
};