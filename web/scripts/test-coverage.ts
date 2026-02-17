#!/usr/bin/env ts-node
import { execSync } from 'child_process';
import { existsSync, readFileSync } from 'fs';
import { join } from 'path';

const COVERAGE_THRESHOLD = 80;

interface CoverageSummary {
  total: {
    lines: { pct: number };
    statements: { pct: number };
    functions: { pct: number };
    branches: { pct: number };
  };
}

function runTestsWithCoverage(): void {
  console.log('Running tests with coverage...');
  execSync('npm run test:coverage', { stdio: 'inherit', cwd: process.cwd() });
}

function readCoverageReport(): CoverageSummary | null {
  const coveragePath = join(process.cwd(), 'coverage', 'coverage-summary.json');
  
  if (!existsSync(coveragePath)) {
    console.error('Coverage report not found');
    return null;
  }
  
  try {
    const content = readFileSync(coveragePath, 'utf-8');
    return JSON.parse(content) as CoverageSummary;
  } catch {
    console.error('Failed to parse coverage report');
    return null;
  }
}

function checkThresholds(summary: CoverageSummary): boolean {
  const metrics = ['lines', 'statements', 'functions', 'branches'] as const;
  let passed = true;
  
  for (const metric of metrics) {
    const pct = summary.total[metric].pct;
    if (pct < COVERAGE_THRESHOLD) {
      console.error(`${metric} coverage ${pct}% below threshold ${COVERAGE_THRESHOLD}%`);
      passed = false;
    } else {
      console.log(`${metric} coverage: ${pct}%`);
    }
  }
  
  return passed;
}

async function main(): Promise<void> {
  try {
    runTestsWithCoverage();
    const summary = readCoverageReport();
    
    if (!summary) {
      process.exit(1);
    }
    
    const passed = checkThresholds(summary);
    
    if (passed) {
      console.log('All coverage thresholds met');
      process.exit(0);
    } else {
      console.error('Coverage thresholds not met');
      process.exit(1);
    }
  } catch (error) {
    console.error('Test coverage failed:', error);
    process.exit(1);
  }
}

main();
