import type { Preview } from '@storybook/react';
import '../src/app/globals.css';

const preview: Preview = {
  parameters: {
    actions: { argTypesRegex: '^on[A-Z].*' },
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/,
      },
    },
    backgrounds: {
      default: 'dark',
      values: [
        {
          name: 'dark',
          value: '#050508',
        },
        {
          name: 'light',
          value: '#ffffff',
        },
      ],
    },
    layout: 'fullscreen',
  },
  decorators: [
    (Story) => (
      <div className="min-h-screen bg-bg-primary p-4">
        <Story />
      </div>
    ),
  ],
};

export default preview;
