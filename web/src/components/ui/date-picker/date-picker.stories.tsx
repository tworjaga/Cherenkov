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
    disabled: {
      control: 'boolean',
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
    placeholder: 'Select a date',
  },
};

export const Disabled: Story = {
  args: {
    date: new Date(),
    disabled: true,
  },
};


export const WithMinMax: Story = {
  args: {
    date: new Date(),
    minDate: new Date(Date.now() - 7 * 24 * 60 * 60 * 1000),
    maxDate: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000),
  },
};
