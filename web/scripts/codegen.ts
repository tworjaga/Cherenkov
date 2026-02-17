#!/usr/bin/env ts-node
import { generate } from '@graphql-codegen/cli';
import config from '../codegen';


async function main() {
  try {
    await generate(config, true);
    console.log('GraphQL code generation completed successfully');
  } catch (error) {
    console.error('Code generation failed:', error);
    process.exit(1);
  }
}

main();
