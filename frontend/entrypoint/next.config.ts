import { NextConfig } from 'next'
import path from 'path'

const root = path.resolve(__dirname, '..')
const pkg = (p: string) => path.resolve(root, 'packages', p, 'src')

const nextConfig: NextConfig = {
  reactStrictMode: true,
  output: 'standalone',
  basePath: '',
  images: { unoptimized: true },
  turbopack: {},
  webpack: (config, { dev }) => {
    config.resolve.alias = {
      ...config.resolve.alias,
      '@ui/buttons': pkg('ui/buttons'),
      '@ui/placeholder': pkg('ui/placeholder'),
      '@pages/login': pkg('pages/login'),
      '@pages/registration': pkg('pages/registration'),
      '@pages/recovery': pkg('pages/recovery'),
      '@pages/settings-success': pkg('settings-success'),
      '@pages/placeholder-title': pkg('ui/placeholder-title'),
      '@widgets/login': pkg('widgets/login'),
      '@widgets/recovery': pkg('widgets/recovery'),
      '@ui/auth-footer': pkg('ui/auth-footer'),
      '@ui/auth-desc-text': pkg('ui/auth-desc-text'),
    }
    if (dev) {
      config.watchOptions = {
        poll: 1000,
        aggregateTimeout: 300,
        ignored: /node_modules\/(?!(@ui|@pages|@shared))/,
      }
    }
    return config
  },
  transpilePackages: ['@ui/buttons'],
}

export default nextConfig
