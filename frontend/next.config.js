/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'standalone',
  experimental: {
    outputFileTracingRoot: process.cwd(),
  },
  // Disable static generation to avoid runtime errors during build
  trailingSlash: true,
}

module.exports = nextConfig
