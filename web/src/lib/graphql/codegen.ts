import type { CodegenConfig } from '@graphql-codegen/cli';

const config: CodegenConfig = {
  schema: './src/lib/graphql/schema.graphql',
  documents: ['./src/lib/graphql/**/*.ts'],
  generates: {
    './src/lib/graphql/generated/': {
      preset: 'client',
      plugins: [],
      config: {
        scalars: {
          Timestamp: 'number',
          UUID: 'string',
          JSON: 'unknown',
        },
      },
    },
  },
  hooks: {
    afterAllFileWrite: ['prettier --write'],
  },
};

export default config;
