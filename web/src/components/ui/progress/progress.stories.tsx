import type { Meta, StoryObj } from '@storybook/react';
import { Progress } from './progress';

const meta: Meta<typeof Progress> = {
  title: 'UI/Progress',
  component: Progress,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
};

export default meta;
type Story = StoryObj<typeof Progress>;

export const Default: Story = {
  args: {
    value: 50,
  },
};

export const Low: Story = {
  args: {
    value: 25,
  },
};

export const High: Story = {
  args: {
    value: 75,
  },
};

export const Complete: Story = {
  args: {
    value: 100,
  },
};

export const Indeterminate: Story = {
  args: {
    indeterminate: true,
  },
};

export const WithValue: Story = {
  args: {
    value: 65,
  },
};
