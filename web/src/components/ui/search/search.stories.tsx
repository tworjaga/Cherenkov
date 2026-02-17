import type { Meta, StoryObj } from '@storybook/react';
import { Search } from './search';

const meta: Meta<typeof Search> = {
  title: 'UI/Search',
  component: Search,
  tags: ['autodocs'],
  argTypes: {
    placeholder: {
      control: 'text',
    },
    autoFocus: {
      control: 'boolean',
    },
  },
};


export default meta;
type Story = StoryObj<typeof Search>;

export const Default: Story = {
  args: {
    placeholder: 'Search...',
  },
};

export const WithValue: Story = {
  args: {
    value: 'Search query',
    placeholder: 'Search...',
  },
};

export const WithAutoFocus: Story = {
  args: {
    placeholder: 'Auto focused search',
    autoFocus: true,
  },
};

export const WithCustomPlaceholder: Story = {

  args: {
    placeholder: 'Search sensors, facilities...',
  },
};
