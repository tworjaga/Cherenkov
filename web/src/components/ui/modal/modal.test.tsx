import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { Modal, ModalHeader, ModalTitle, ModalDescription, ModalContent, ModalFooter } from './modal';

describe('Modal', () => {
  it('renders modal when open', () => {
    render(
      <Modal open={true} onOpenChange={vi.fn()}>
        <ModalContent>
          <ModalHeader>
            <ModalTitle>Modal Title</ModalTitle>
            <ModalDescription>Modal Description</ModalDescription>
          </ModalHeader>
          <p>Modal content</p>
          <ModalFooter>
            <button>Action</button>
          </ModalFooter>
        </ModalContent>
      </Modal>
    );

    expect(screen.getByText('Modal Title')).toBeInTheDocument();
    expect(screen.getByText('Modal Description')).toBeInTheDocument();
    expect(screen.getByText('Modal content')).toBeInTheDocument();
    expect(screen.getByText('Action')).toBeInTheDocument();
  });

  it('does not render when closed', () => {
    const { container } = render(
      <Modal open={false} onOpenChange={vi.fn()}>
        <ModalContent>
          <p>Hidden content</p>
        </ModalContent>
      </Modal>
    );

    expect(container.textContent).not.toContain('Hidden content');
  });

  it('calls onOpenChange when close button clicked', () => {
    const handleOpenChange = vi.fn();
    render(
      <Modal open={true} onOpenChange={handleOpenChange}>
        <ModalContent>
          <p>Content</p>
        </ModalContent>
      </Modal>
    );

    const closeButton = screen.getByRole('button', { name: /close/i });
    fireEvent.click(closeButton);
    expect(handleOpenChange).toHaveBeenCalledWith(false);
  });

});
