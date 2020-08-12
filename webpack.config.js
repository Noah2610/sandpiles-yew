const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");

const PORT = 8080;

const distPath = path.resolve(__dirname, "dist");
module.exports = (env, argv) => ({
    devServer: {
        contentBase: distPath,
        compress: argv.mode === "production",
        port: PORT,
    },
    entry: "./bootstrap.js",
    output: {
        path: distPath,
        filename: "bundle.js",
        webassemblyModuleFilename: "bundle.wasm",
    },
    module: {
        rules: [
            {
                test: /\.css$/i,
                use: [
                    "style-loader",
                    { loader: "css-loader", options: { importLoaders: 1 } },
                    "postcss-loader",
                ],
            },
        ],
    },
    plugins: [
        new CopyWebpackPlugin([{ from: "./public", to: distPath }]),
        new WasmPackPlugin({
            crateDirectory: ".",
            extraArgs: "--no-typescript",
        }),
    ],
    watch: argv.mode !== "production",
});
