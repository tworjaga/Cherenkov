import type { Meta, StoryObj } from '@storybook/react';
import { Checkbox } from './checkbox';

const meta: Meta<typeof Checkbox> = {
  title: 'UI/Checkbox',
  component: Checkbox,
  tags: ['autodocs'],
  argTypes: {
    checked: {
      control: 'boolean',
    },
    disabled: {
      control: 'boolean',
    },
  },
};

export default meta;
type Story = StoryObj<typeof Checkbox>;

export const Default: Story = {
  args: {
    children: 'Accept terms and conditions',
  },
};

export const Checked: Story = {
  args: {
    checked: true,
    children: 'Option selected',
  },
};

export const Disabled: Story = {
  args: {
    disabled: true,
    children: 'Disabled option',
  },
};

export const DisabledChecked: Story = {
  args: {
    disabled: true,
    checked: true,
    children: 'Disabled checked option',
  },
};

export const WithoutLabel: Story = {
  args: {},
};

export const Group: Story = {
  render: () => (
    <div className="space-y-2">
      <Checkbox checked>Option 1</Checkbox>
      <Checkbox>Option 2</Checkbox>
      <Checkbox>Option 3</Checkbox>
    </div>
  ),
};
