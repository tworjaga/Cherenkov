import type { Meta, StoryObj } from '@storybook/react';
import { DatePicker } from './date-picker';

const meta: Meta<typeof DatePicker> = {
  title: 'UI/DatePicker',
  component: DatePicker,
  tags: ['autodocs'],
  argTypes: {
    date: {
      control: 'date',
    },
  },
};



export default meta;
type Story = StoryObj<typeof DatePicker>;

export const Default: Story = {
  args: {
    date: new Date(),
  },
};


export const WithPlaceholder: Story = {
  args: {
    date: new Date(),
    placeholder: 'Select a date',
  },
};
