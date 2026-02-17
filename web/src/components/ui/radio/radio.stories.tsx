import type { Meta, StoryObj } from '@storybook/react';
import { Radio } from './radio';

const meta: Meta<typeof Radio> = {
  title: 'UI/Radio',
  component: Radio,
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
type Story = StoryObj<typeof Radio>;

export const Default: Story = {
  args: {
    name: 'option',
    value: '1',
    children: 'Option 1',
  },
};

export const Checked: Story = {
  args: {
    name: 'option',
    value: '1',
    checked: true,
    children: 'Selected option',
  },
};

export const Disabled: Story = {
  args: {
    name: 'option',
    value: '1',
    disabled: true,
    children: 'Disabled option',
  },
};

export const DisabledChecked: Story = {
  args: {
    name: 'option',
    value: '1',
    disabled: true,
    checked: true,
    children: 'Disabled selected option',
  },
};

export const Group: Story = {
  render: () => (
    <div className="space-y-2">
      <Radio name="group" value="1" checked>
        First option
      </Radio>
      <Radio name="group" value="2">
        Second option
      </Radio>
      <Radio name="group" value="3">
        Third option
      </Radio>
    </div>
  ),
};
