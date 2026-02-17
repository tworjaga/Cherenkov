import type { Meta, StoryObj } from '@storybook/react';
import { ScrollArea } from './scroll-area';

const meta: Meta<typeof ScrollArea> = {
  title: 'UI/ScrollArea',
  component: ScrollArea,
  tags: ['autodocs'],
};

export default meta;
type Story = StoryObj<typeof ScrollArea>;

const longContent = Array.from({ length: 50 }, (_, i) => (
  <p key={i} className="py-2 text-sm text-text-secondary">
    Item {i + 1}: Lorem ipsum dolor sit amet, consectetur adipiscing elit.
  </p>
));

export const Default: Story = {
  render: () => (
    <ScrollArea className="h-[200px] w-[350px] rounded-md border border-border-subtle p-4">
      {longContent}
    </ScrollArea>
  ),
};

export const Horizontal: Story = {
  render: () => (
    <ScrollArea className="h-[100px] w-[350px] whitespace-nowrap rounded-md border border-border-subtle p-4">
      <div className="flex gap-4">
        {Array.from({ length: 20 }, (_, i) => (
          <div
            key={i}
            className="h-16 w-32 flex-shrink-0 rounded bg-bg-tertiary flex items-center justify-center text-text-secondary"
          >
            Item {i + 1}
          </div>
        ))}
      </div>
    </ScrollArea>
  ),
};

export const Both: Story = {
  render: () => (
    <ScrollArea className="h-[200px] w-[350px] rounded-md border border-border-subtle p-4">
      <div className="w-[600px]">
        {longContent}
      </div>
    </ScrollArea>
  ),
};
