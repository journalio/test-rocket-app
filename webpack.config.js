const { CleanWebpackPlugin } = require('clean-webpack-plugin')
const { resolve } = require('path')
const VueLoaderPlugin = require('vue-loader/lib/plugin')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const MiniCssExtractPlugin = require('mini-css-extract-plugin')

const getFinalCssLoader = mode => {
    if (mode === 'production') {
        return {
            loader: MiniCssExtractPlugin.loader,
            options: {
                esModule: true,
            },
        }
    }
    return 'vue-style-loader'
}

const getPostCssPlugins = mode => {
    const plugins = [require('tailwindcss')]
    if (mode === 'production') {
        plugins.push(
            require('@fullhuman/postcss-purgecss')({
                content: ['app/**/*.html', 'app/**/*.vue'],
                defaultExtractor: content =>
                    content.match(/[\w-/:]+(?<!:)/g) || [],
            }),
        )
    }
    return plugins
}

module.exports = (env, { mode }) => {
    const config = {
        entry: resolve(__dirname, 'app', 'index.js'),
        context: resolve(__dirname, 'app'),
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
                        presets: [
                            [
                                '@babel/preset-env',
                                {
                                    modules: false,
                                    targets: {
                                        browsers: [
                                            '> 1%',
                                            'last 2 versions',
                                            'not ie <= 8',
                                        ],
                                    },
                                },
                            ],
                        ],
                        plugins: ['@babel/plugin-transform-runtime'],
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
                                plugins: getPostCssPlugins(mode),
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
                inject: true,
                template: 'index.html',
            }),
        ],
    }

    if (mode === 'production') {
        config.plugins.push(new MiniCssExtractPlugin())
    }

    return config
}
