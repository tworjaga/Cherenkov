import type { Meta, StoryObj } from '@storybook/react';
import { Separator } from './separator';

const meta: Meta<typeof Separator> = {
  title: 'UI/Separator',
  component: Separator,
  tags: ['autodocs'],
  argTypes: {
    orientation: {
      control: 'select',
      options: ['horizontal', 'vertical'],
    },
  },
};

export default meta;
type Story = StoryObj<typeof Separator>;

export const Default: Story = {
  args: {},
};

export const Horizontal: Story = {
  args: {
    orientation: 'horizontal',
  },
};

export const Vertical: Story = {
  args: {
    orientation: 'vertical',
    className: 'h-20',
  },
};

export const WithContent: Story = {
  render: () => (
    <div className="space-y-4">
      <div className="text-sm">Content above</div>
      <Separator />
      <div className="text-sm">Content below</div>
    </div>
  ),
};

export const VerticalWithContent: Story = {
  render: () => (
    <div className="flex h-20 items-center space-x-4">
      <div className="text-sm">Left</div>
      <Separator orientation="vertical" />
      <div className="text-sm">Right</div>
    </div>
  ),
};

export const CustomStyling: Story = {
  args: {
    className: 'bg-[#00d4ff]',
  },
};
