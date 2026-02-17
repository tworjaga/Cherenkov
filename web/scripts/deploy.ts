#!/usr/bin/env ts-node
import { execSync } from 'child_process';
import { existsSync } from 'fs';
import { join } from 'path';

const DEPLOYMENT_TARGETS = ['staging', 'production'] as const;
type DeploymentTarget = (typeof DEPLOYMENT_TARGETS)[number];

function validateTarget(target: string): target is DeploymentTarget {
  return DEPLOYMENT_TARGETS.includes(target as DeploymentTarget);
}

function buildProject(): void {
  console.log('Building project...');
  execSync('npm run build', { stdio: 'inherit', cwd: process.cwd() });
}

function runTests(): void {
  console.log('Running tests...');
  execSync('npm test', { stdio: 'inherit', cwd: process.cwd() });
}

function deployToTarget(target: DeploymentTarget): void {
  console.log(`Deploying to ${target}...`);
  const deployScript = join(process.cwd(), 'scripts', 'deploy.sh');
  
  if (!existsSync(deployScript)) {
    throw new Error(`Deploy script not found: ${deployScript}`);
  }
  
  execSync(`bash ${deployScript} ${target}`, { stdio: 'inherit' });
}

async function main(): Promise<void> {
  const target = process.argv[2];
  
  if (!target || !validateTarget(target)) {
    console.error(`Usage: ts-node deploy.ts <${DEPLOYMENT_TARGETS.join('|')}>`);
    process.exit(1);
  }
  
  try {
    runTests();
    buildProject();
    deployToTarget(target);
    console.log(`Successfully deployed to ${target}`);
  } catch (error) {
    console.error('Deployment failed:', error);
    process.exit(1);
  }
}

main();
