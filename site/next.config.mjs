// Auto-generate navigation
// TODO Move to server component
import './scripts/generateNavigation.mjs';

import path from 'node:path';
import url from 'node:url';
import nextMDX from '@next/mdx';
import { config } from './src/mdx/mdx.mjs';

const withMDX = nextMDX(config);

/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'export',
  reactStrictMode: true,
  transpilePackages: ['@rivet-gg/components', '@rivet-gg/icons'],
  typescript: {
    ignoreBuildErrors: true
  },
  pageExtensions: ['js', 'jsx', 'ts', 'tsx', 'mdx', 'md'],
  images: {
    // For static output
    unoptimized: true
  },
  experimental: {
    scrollRestoration: true
  },
  webpack: config => {
    const __dirname = path.dirname(url.fileURLToPath(import.meta.url));
    return {
      ...config,
      resolve: {
        ...config.resolve,
        fallback: {
          'react/jsx-dev-runtime': path.resolve(__dirname, '../node_modules/react/jsx-dev-runtime.js'),
          react: path.resolve(__dirname, '../node_modules/react')
        }
      }
    };
  }
};

export default async function () {
  return withMDX(nextConfig);
}
