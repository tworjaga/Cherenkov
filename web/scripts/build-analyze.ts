#!/usr/bin/env ts-node
import { bundleAnalyzer } from 'next-bundle-analyzer';

async function main() {
  try {
    await bundleAnalyzer({
      enabled: true,
      openAnalyzer: true,
      analyzerMode: 'static',
      reportFilename: 'bundle-report.html',
    });
    console.log('Bundle analysis completed');
  } catch (error) {
    console.error('Bundle analysis failed:', error);
    process.exit(1);
  }
}

main();
