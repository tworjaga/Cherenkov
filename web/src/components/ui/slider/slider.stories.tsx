import type { Meta, StoryObj } from '@storybook/react';
import { Slider } from './slider';

const meta: Meta<typeof Slider> = {
  title: 'UI/Slider',
  component: Slider,
  tags: ['autodocs'],
  argTypes: {
    defaultValue: {
      control: 'object',
    },
    max: {
      control: 'number',
    },
    min: {
      control: 'number',
    },
    step: {
      control: 'number',
    },
    disabled: {
      control: 'boolean',
    },
  },
};

export default meta;
type Story = StoryObj<typeof Slider>;

export const Default: Story = {
  args: {
    defaultValue: [50],
    max: 100,
    step: 1,
  },
};

export const WithLabel: Story = {
  render: () => (
    <div className="w-[300px] space-y-4">
      <div className="flex justify-between">
        <span className="text-sm text-[#a0a0b0]">Volume</span>
        <span className="text-sm text-[#a0a0b0]">50%</span>
      </div>
      <Slider defaultValue={[50]} max={100} step={1} />
    </div>
  ),
};

export const Range: Story = {
  args: {
    defaultValue: [25, 75],
    max: 100,
    step: 1,
  },
};

export const Disabled: Story = {
  args: {
    defaultValue: [50],
    max: 100,
    disabled: true,
  },
};

export const SmallSteps: Story = {
  args: {
    defaultValue: [0.5],
    max: 1,
    step: 0.1,
  },
};

export const Vertical: Story = {
  render: () => (
    <div className="h-[200px]">
      <Slider
        defaultValue={[50]}
        max={100}
        step={1}
        orientation="vertical"
        className="h-full"
      />
    </div>
  ),
};
