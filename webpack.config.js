const { CleanWebpackPlugin } = require('clean-webpack-plugin')
const { resolve } = require('path')
const VueLoaderPlugin = require('vue-loader/lib/plugin')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const MiniCssExtractPlugin = require('mini-css-extract-plugin')


const getFinalCssLoader = mode => {
    if (mode === 'production') {
        return MiniCssExtractPlugin.loader
    }
    return 'vue-style-loader'
}

module.exports = (env, { mode }) => {
    const config = ({
        entry: './app/index.js',
        context: __dirname,
        output: {
            path: resolve(__dirname, 'public'),
        },
        module: {
            rules: [
                {
                    test: /\.vue$/,
                    loader: 'vue-loader',
                },

                {
                    test: /\.js$/,
                    loader: 'babel-loader',
                    exclude: /node_modules/,
                    options: {
                        presets: [['@babel/preset-env', {
                            modules: false,
                            'targets': {
                                'browsers': ['> 1%', 'last 2 versions', 'not ie <= 8'],
                            },
                        }]],
                        plugins: [
                            '@babel/plugin-transform-runtime',
                        ],
                    },
                },
                {
                    test: /\.css$/,
                    use: [
                        getFinalCssLoader(mode),
                        {
                            loader: 'css-loader',
                            options: {
                                importLoaders: 1,
                            },
                        },
                        {
                            loader: 'postcss-loader',
                            options: {
                                plugins: [
                                    require('tailwindcss'),
                                ],
                            },
                        },
                    ],
                },
            ],
        },
        resolve: {
            extensions: ['.vue', '.css', '.wasm', '.mjs', '.js', '.json'],
        },
        plugins: [
            new CleanWebpackPlugin(),
            new VueLoaderPlugin(),
            new HtmlWebpackPlugin({
                title: 'Test rocket app',
            }),
        ],
    })

    if (mode === 'production') {
        config.plugins.push(new MiniCssExtractPlugin({}))
    }

    return config
}