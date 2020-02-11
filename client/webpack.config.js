'use strict';
const webpack = require('webpack');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const path = require('path');



module.exports = (env, args) => {
  const isProductionMode = (args.mode === 'production');

  return {
    entry: './src/Main.ts',
    output: {
      path: path.resolve(__dirname, 'dist'),
      filename: isProductionMode ? '[name].[contenthash].js' : '[name].[hash].js',
    },
    devServer: {
      host: '192.168.1.150',
      disableHostCheck: true
    },
    plugins: [
      new HtmlWebpackPlugin({
        template: 'index.html'
      }),
      new webpack.ProvidePlugin({
        TextDecoder: ['text-encoding', 'TextDecoder'],
        TextEncoder: ['text-encoding', 'TextEncoder']
      }),
      new CopyWebpackPlugin([
        { from: 'assets' }
      ]),
    ],
    module: {
      rules: [
        {
          test: /\.tsx?$/,
          loader: 'ts-loader'
        }
      ]
    },
    resolve: {
      extensions: ['.ts', '.tsx', '.js']
    }
  };
}