const path = require('path');

const CopyPlugin = require("copy-webpack-plugin");
const CssMinimizerPlugin = require('css-minimizer-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = {
    entry: {
        index: './src/index.ts',
    },
    resolve: {
        extensions: ['.ts', '.tsx', '.js', '.jsx', '.mjs', '.json', '.wasm'],
    },
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: '[name]/[name].[contenthash].js',
        publicPath: 'auto',
        clean: false,
    },

    // See: https://webpack.js.org/guides/caching/
    optimization: {
        moduleIds: 'deterministic',
        runtimeChunk: 'single',
        splitChunks: {
            cacheGroups: {
                vendor: {
                    test: /[\\/]node_modules[\\/]/,
                    name: 'vendors',
                    chunks: 'all',
                }
            }
        },
        minimize: true,
        minimizer: [
            `...`,
            new CssMinimizerPlugin(),
        ],
    },
    module: {
        rules: [
            {
                test: /\.(mjs|js|jsx|ts|tsx)$/,
                exclude: /node_modules/,
                use: {
                    loader: 'babel-loader',
                    options: {
                        presets: [
                            '@babel/preset-env',
                        ],
                        plugins: [
                            ["@babel/transform-runtime", {
                                "regenerator": true,
                            }],
                        ]
                    }
                }
            },
            {
                test: /\.css$/,
                use: [MiniCssExtractPlugin.loader, 'css-loader'],
            },
            {
                test: /\.(png|svg|jpg|jpeg|gif)$/i,
                type: 'asset/resource',
            },
            {
                test: /\.(woff|woff2|eot|ttf|otf)$/i,
                type: 'asset/resource',
            },
            {
                test: /\.wasm$/,
                type: 'webassembly/sync',
            }
        ]
    },
    experiments: {
        syncWebAssembly: true,
    },
    plugins: [
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, '..', 'rust-wasm-bindgen'),
            watchDirectories: [
                path.resolve(__dirname, '..', 'rust'),
                path.resolve(__dirname, '..', 'rust-wasm-bindgen'),
            ],
            outDir: path.resolve(__dirname, 'pkg_rust'),
            outName: "rust",
            forceMode: "production",
        }),
        new MiniCssExtractPlugin({
            filename: '[name].[contenthash].css',
            chunkFilename: '[id].[contenthash].css',
        }),
        new CopyPlugin({
            patterns: [
                path.resolve(__dirname, 'src', 'favicon.ico'),
                path.resolve(__dirname, 'src', '*.png'),
            ],
        }),
        new HtmlWebpackPlugin({
            title: 'SheepText',
            template: path.resolve(__dirname, 'src', 'index.html'),
            filename: 'index.html',
            chunks: ['index'],
        })
    ]
};
