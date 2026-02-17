import type { Meta, StoryObj } from '@storybook/react';
import { Textarea } from './textarea';

const meta: Meta<typeof Textarea> = {
  title: 'UI/Textarea',
  component: Textarea,
  tags: ['autodocs'],
  argTypes: {
    disabled: {
      control: 'boolean',
    },
    placeholder: {
      control: 'text',
    },
  },
};

export default meta;
type Story = StoryObj<typeof Textarea>;

export const Default: Story = {
  args: {
    placeholder: 'Enter your message...',
  },
};

export const WithValue: Story = {
  args: {
    defaultValue: 'This is some text content in the textarea.',
    placeholder: 'Enter your message...',
  },
};

export const Disabled: Story = {
  args: {
    disabled: true,
    defaultValue: 'This textarea is disabled',
    placeholder: 'Enter your message...',
  },
};

export const WithLabel: Story = {
  render: () => (
    <div className="space-y-2">
      <label className="text-sm font-medium text-[#a0a0b0]">Description</label>
      <Textarea placeholder="Enter description..." />
    </div>
  ),
};

export const Resizable: Story = {
  args: {
    className: 'resize-y',
    placeholder: 'This textarea can be resized vertically...',
  },
};

export const AutoResize: Story = {
  render: () => (
    <Textarea
      className="min-h-[80px]"
      placeholder="This textarea has minimum height..."
    />
  ),
};
