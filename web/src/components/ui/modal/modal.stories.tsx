import type { Meta, StoryObj } from '@storybook/react';
import { useState } from 'react';
import {
  Modal,
  ModalContent,
  ModalHeader,
  ModalTitle,
  ModalDescription,
  ModalFooter,
} from './modal';
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

const DefaultModalExample = () => {
  const [open, setOpen] = useState(false);
  return (
    <Modal open={open} onOpenChange={setOpen}>
      <Button onClick={() => setOpen(true)}>Open Modal</Button>
      <ModalContent>
        <ModalHeader>
          <ModalTitle>Modal Title</ModalTitle>
          <ModalDescription>
            Modal content goes here.
          </ModalDescription>
        </ModalHeader>
      </ModalContent>
    </Modal>
  );
};

export const Default: Story = {
  render: () => <DefaultModalExample />,
};

const WithFooterModalExample = () => {
  const [open, setOpen] = useState(false);
  return (
    <Modal open={open} onOpenChange={setOpen}>
      <Button onClick={() => setOpen(true)}>Open Modal with Footer</Button>
      <ModalContent>
        <ModalHeader>
          <ModalTitle>Confirm Action</ModalTitle>
          <ModalDescription>
            Are you sure you want to proceed?
          </ModalDescription>
        </ModalHeader>
        <ModalFooter>
          <Button variant="ghost" onClick={() => setOpen(false)}>
            Cancel
          </Button>
          <Button onClick={() => setOpen(false)}>Confirm</Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
};

export const WithFooter: Story = {
  render: () => <WithFooterModalExample />,
};

const LargeModalExample = () => {
  const [open, setOpen] = useState(false);
  return (
    <Modal open={open} onOpenChange={setOpen}>
      <Button onClick={() => setOpen(true)}>Open Large Modal</Button>
      <ModalContent className="max-w-2xl">
        <ModalHeader>
          <ModalTitle>Large Modal</ModalTitle>
          <ModalDescription>
            This is a large modal with more content.
            Additional content can go here.
          </ModalDescription>
        </ModalHeader>
      </ModalContent>
    </Modal>
  );
};

export const Large: Story = {
  render: () => <LargeModalExample />,
};
