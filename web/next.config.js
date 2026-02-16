/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  experimental: {
    appDir: true,
  },
  images: {
    domains: [],
  },
  webpack: (config) => {
    config.externals.push({
      'mapbox-gl': 'mapboxgl',
    });
    return config;
  },
};

module.exports = nextConfig;
