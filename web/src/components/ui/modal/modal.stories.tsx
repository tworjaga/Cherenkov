import type { Meta, StoryObj } from '@storybook/react';
import { useState } from 'react';
import { Modal } from './modal';
import { Button } from '../button';

const meta: Meta<typeof Modal> = {
  title: 'UI/Modal',
  component: Modal,
  tags: ['autodocs'],
  argTypes: {
    open: {
      control: 'boolean',
    },
  },
};


export default meta;
type Story = StoryObj<typeof Modal>;

export const Default: Story = {
  render: () => {
    const [open, setOpen] = useState(false);
    return (
      <>
        <Button onClick={() => setOpen(true)}>Open Modal</Button>
        <Modal open={open} onOpenChange={setOpen} title="Modal Title">
          <p className="text-[#a0a0b0]">Modal content goes here.</p>
        </Modal>
      </>
    );
  },
};


export const WithFooter: Story = {
  render: () => {
    const [open, setOpen] = useState(false);
    return (
      <>
        <Button onClick={() => setOpen(true)}>Open Modal with Footer</Button>
        <Modal
          open={open}
          onOpenChange={setOpen}
          title="Confirm Action"
          footer={
            <div className="flex justify-end gap-2">
              <Button variant="ghost" onClick={() => setOpen(false)}>
                Cancel
              </Button>
              <Button onClick={() => setOpen(false)}>Confirm</Button>
            </div>
          }
        >
          <p className="text-[#a0a0b0]">Are you sure you want to proceed?</p>
        </Modal>
      </>
    );
  },
};


export const Large: Story = {
  render: () => {
    const [open, setOpen] = useState(false);
    return (
      <>
        <Button onClick={() => setOpen(true)}>Open Large Modal</Button>
        <Modal
          open={open}
          onOpenChange={setOpen}
          title="Large Modal"
          size="lg"
        >
          <div className="space-y-4">
            <p className="text-[#a0a0b0]">This is a large modal with more content.</p>
            <p className="text-[#a0a0b0]">Additional content can go here.</p>
          </div>
        </Modal>
      </>
    );
  },
};
