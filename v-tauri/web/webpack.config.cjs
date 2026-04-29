const path = require('path')
const webpack = require('webpack')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const MiniCssExtractPlugin = require('mini-css-extract-plugin')
const { VueLoaderPlugin } = require('vue-loader')

const rootDir = path.join(__dirname, '../../')
const srcDir = path.join(rootDir, 'src')
const rendererDir = path.join(srcDir, 'renderer')
const outDir = path.join(rootDir, 'v-tauri', 'frontend-dist')

const isDev = process.env.NODE_ENV === 'development'

module.exports = {
  target: 'web',
  mode: isDev ? 'development' : 'production',
  devtool: isDev ? 'eval-source-map' : 'source-map',
  entry: {
    renderer: [
      path.join(__dirname, './compat/polyfills.js'),
      path.join(rendererDir, 'main.ts'),
    ],
  },
  output: {
    filename: isDev ? '[name].js' : '[name].[contenthash:8].js',
    chunkFilename: isDev ? '[name].js' : '[name].[contenthash:8].js',
    path: outDir,
    publicPath: '',
    clean: true,
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js', '.json', '.vue'],
    modules: [
      path.join(rootDir, 'v-tauri/node_modules'),
      'node_modules',
    ],
    alias: {
      '@root': srcDir,
      '@main': path.join(srcDir, 'main'),
      '@renderer': path.join(srcDir, 'renderer'),
      '@lyric': path.join(srcDir, 'renderer-lyric'),
      '@static': path.join(srcDir, 'static'),
      '@common': path.join(srcDir, 'common'),
      electron$: path.join(__dirname, './compat/electron.js'),
      'electron-log/node$': path.join(__dirname, './compat/electron-log-node.js'),
      'electron-updater$': path.join(__dirname, './compat/node/empty.js'),
      crypto$: path.join(__dirname, './compat/node/crypto.js'),
      dns$: path.join(__dirname, './compat/node/dns.js'),
      fs$: path.join(__dirname, './compat/node/empty.js'),
      'fs/promises$': path.join(__dirname, './compat/node/empty.js'),
      os$: path.join(__dirname, './compat/node/os.js'),
      path$: path.join(__dirname, './compat/node/path.js'),
      'node:fs$': path.join(__dirname, './compat/node/empty.js'),
      'node:fs/promises$': path.join(__dirname, './compat/node/empty.js'),
      'node:os$': path.join(__dirname, './compat/node/os.js'),
      'node:path$': path.join(__dirname, './compat/node/path.js'),
      zlib$: path.join(__dirname, './compat/node/empty.js'),
      tunnel$: path.join(__dirname, './compat/node/empty.js'),
      'music-metadata$': path.join(__dirname, './compat/music-metadata.js'),
    },
    fallback: {
      buffer: require.resolve('buffer/'),
      fs: false,
      path: false,
      os: false,
      zlib: false,
      stream: false,
      crypto: false,
    },
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        exclude: /node_modules/,
        use: {
          loader: 'ts-loader',
          options: {
            transpileOnly: true,
            appendTsSuffixTo: [/\.vue$/],
            configFile: path.join(__dirname, 'tsconfig.web.json'),
          },
        },
      },
      {
        test: /\.vue$/,
        loader: 'vue-loader',
      },
      {
        test: /\.pug$/,
        loader: 'pug-plain-loader',
      },
      {
        resourceQuery: /lang=css/,
        use: [
          isDev ? 'style-loader' : MiniCssExtractPlugin.loader,
          'css-loader',
        ],
      },
      {
        resourceQuery: /lang=less/,
        use: [
          isDev ? 'style-loader' : MiniCssExtractPlugin.loader,
          'css-loader',
          {
            loader: 'less-loader',
            options: {
              sourceMap: true,
            },
          },
        ],
      },
      {
        test: /\.css$/,
        use: [
          isDev ? 'style-loader' : MiniCssExtractPlugin.loader,
          'css-loader',
        ],
      },
      {
        test: /\.less$/,
        use: [
          isDev ? 'style-loader' : MiniCssExtractPlugin.loader,
          'css-loader',
          {
            loader: 'less-loader',
            options: {
              sourceMap: true,
            },
          },
        ],
      },
      {
        test: /\.(png|jpe?g|gif|svg)(\?.*)?$/,
        type: 'asset',
        parser: {
          dataUrlCondition: {
            maxSize: 10000,
          },
        },
        generator: {
          filename: 'imgs/[name]-[contenthash:8][ext]',
        },
      },
      {
        test: /\.(mp4|webm|ogg|mp3|wav|flac|aac)$/,
        type: 'asset',
        parser: {
          dataUrlCondition: {
            maxSize: 10000,
          },
        },
        generator: {
          filename: 'media/[name]-[contenthash:8][ext]',
        },
      },
      {
        test: /\.(woff2?|eot|ttf|otf)(\?.*)?$/,
        type: 'asset',
        parser: {
          dataUrlCondition: {
            maxSize: 10000,
          },
        },
        generator: {
          filename: 'fonts/[name]-[contenthash:8][ext]',
        },
      },
    ],
  },
  plugins: [
    new webpack.DefinePlugin({
      'process.env': {
        NODE_ENV: JSON.stringify(isDev ? 'development' : 'production'),
        ELECTRON_DISABLE_SECURITY_WARNINGS: JSON.stringify(true),
      },
      __VUE_OPTIONS_API__: 'true',
      __VUE_PROD_DEVTOOLS__: 'false',
      __VUE_PROD_HYDRATION_MISMATCH_DETAILS__: 'false',
      COMMIT_ID: '""',
      COMMIT_DATE: '""',
    }),
    new webpack.ProvidePlugin({
      Buffer: ['buffer', 'Buffer'],
    }),
    new webpack.NormalModuleReplacementPlugin(/^node:/, (resource) => {
      resource.request = resource.request.replace(/^node:/, '')
    }),
    new webpack.NormalModuleReplacementPlugin(/src[\\/]renderer[\\/]utils[\\/]request\.js$/, path.join(__dirname, './compat/request.js')),
    new webpack.NormalModuleReplacementPlugin(/src[\\/]renderer[\\/]worker[\\/]index\.ts$/, path.join(__dirname, './compat/worker.js')),
    new webpack.NormalModuleReplacementPlugin(/src[\\/]renderer[\\/]worker[\\/]utils[\\/]index\.ts$/, path.join(__dirname, './compat/worker-utils.js')),
    new HtmlWebpackPlugin({
      filename: 'index.html',
      template: path.join(rendererDir, 'index.html'),
    }),
    new VueLoaderPlugin(),
    new MiniCssExtractPlugin({
      filename: isDev ? '[name].css' : '[name].[contenthash:8].css',
      chunkFilename: isDev ? '[name].css' : '[name].[contenthash:8].css',
    }),
  ],
  devServer: {
    port: 1420,
    hot: true,
    historyApiFallback: true,
    headers: {
      'Access-Control-Allow-Origin': '*',
    },
    static: {
      directory: outDir,
    },
  },
  performance: {
    hints: false,
  },
}
