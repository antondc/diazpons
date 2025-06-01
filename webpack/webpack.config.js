const path = require("path");
const CompressionPlugin = require("compression-webpack-plugin");
const HtmlWebPackPlugin = require("html-webpack-plugin");
const NodePolyfillPlugin = require("node-polyfill-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const CaseSensitivePathsPlugin = require("case-sensitive-paths-webpack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");

const WEBPACK_BUILD = path.resolve(process.cwd(), "webpack");
const WEBPACK_DIST = path.resolve(process.cwd(), "dist");
const WEBPACK_SRC = path.resolve(process.cwd(), "src");
const WEBPACK_PRESENTATION = path.resolve(WEBPACK_SRC, "presentation");
const WEBPACK_SRC_CLIENT = path.resolve(WEBPACK_BUILD, "index.js");
const WEBPACK_ASSETS = path.resolve(WEBPACK_PRESENTATION, "assets");

module.exports = {
  entry: WEBPACK_SRC_CLIENT,
  mode: "development",
  output: {
    filename: "client-[fullhash:4].js",
    path: WEBPACK_DIST,
    publicPath: "/",
  },
  target: "web",
  module: {
    rules: [
      {
        test: /\.(less|css)$/i,
        use: [MiniCssExtractPlugin.loader, "css-loader", "less-loader"],
        exclude: /node_modules/,
      },
      {
        test: /\.(js|jsx|ts|tsx)$/,
        exclude: /node_modules/,
        use: {
          loader: "babel-loader",
          options: {
            presets: ["@babel/preset-env", "@babel/preset-typescript"],
          },
        },
      },
      {
        test: /\.svg$/,
        use: ["@svgr/webpack"],
      },
      {
        test: /\.(woff|woff2|eot|ttf)$/,
        type: "asset/resource",
        generator: {
          filename: "fonts/[name].[ext]",
        },
      },
    ],
  },
  plugins: [
    new CleanWebpackPlugin(),
    new HtmlWebPackPlugin({
      baseUrl: "",
      publicPath: "/static/",
      inject: true,
      template: path.join(WEBPACK_PRESENTATION, "indexBase.html"),
      filename: path.join(WEBPACK_PRESENTATION, "index.html"),
    }),
    new CompressionPlugin({
      algorithm: "gzip",
      test: /\.svg$|\.woff$|\.woff2$|\.ttf$|\.eot$|\.otf$|\.js$|\.css$|\.html$/,
    }),
    new NodePolyfillPlugin(),
    new MiniCssExtractPlugin({
      filename: "[name]-[fullhash:4].css",
      chunkFilename: "[name]-[fullhash:4].css",
    }),
    new CaseSensitivePathsPlugin(),
    new CopyWebpackPlugin([
      {
        from: path.join(WEBPACK_ASSETS, "img"),
        to: path.join(WEBPACK_DIST, "img"),
      },
      {
        from: path.join(WEBPACK_ASSETS, "svg"),
        to: path.join(WEBPACK_DIST, "svg"),
      },
      {
        from: path.join(WEBPACK_ASSETS, "favicon"),
        to: path.join(WEBPACK_DIST, "favicon"),
      },
    ]),
  ],
};
