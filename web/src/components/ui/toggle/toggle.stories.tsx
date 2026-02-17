import type { Meta, StoryObj } from '@storybook/react';
import { Toggle } from './toggle';

const meta: Meta<typeof Toggle> = {
  title: 'UI/Toggle',
  component: Toggle,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
};

export default meta;
type Story = StoryObj<typeof Toggle>;

export const Default: Story = {
  args: {
    children: 'Toggle',
  },
};

export const Pressed: Story = {
  args: {
    pressed: true,
    children: 'Pressed',
  },
};

export const Disabled: Story = {
  args: {
    disabled: true,
    children: 'Disabled',
  },
};

export const WithIcon: Story = {
  args: {
    children: (
      <>
        <span className="mr-2">★</span>
        Star
      </>
    ),
  },
};
