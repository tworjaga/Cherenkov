import type { Meta, StoryObj } from '@storybook/react';
import { Alert } from './alert';

const meta: Meta<typeof Alert> = {
  title: 'UI/Alert',
  component: Alert,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
};

export default meta;
type Story = StoryObj<typeof Alert>;

export const Info: Story = {
  args: {
    variant: 'info',
    children: 'This is an info alert message.',
  },
};

export const Success: Story = {
  args: {
    variant: 'success',
    children: 'Operation completed successfully.',
  },
};

export const Warning: Story = {
  args: {
    variant: 'warning',
    children: 'Please review your input.',
  },
};

export const Error: Story = {
  args: {
    variant: 'error',
    children: 'Something went wrong.',
  },
};

export const Dismissible: Story = {
  args: {
    variant: 'info',
    dismissible: true,
    onDismiss: () => {},
    children: 'Click the X to dismiss this alert.',
  },
};
