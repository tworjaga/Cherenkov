import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { Alert } from './alert';

describe('Alert', () => {
  it('renders with default variant', () => {
    render(<Alert>Test message</Alert>);
    expect(screen.getByText('Test message')).toBeInTheDocument();
  });

  it('renders with title', () => {
    render(<Alert title="Alert Title">Test message</Alert>);
    expect(screen.getByText('Alert Title')).toBeInTheDocument();
    expect(screen.getByText('Test message')).toBeInTheDocument();
  });

  it('renders all variants', () => {
    const variants = ['info', 'success', 'warning', 'error'] as const;
    
    variants.forEach(variant => {
      const { container } = render(<Alert variant={variant}>Message</Alert>);
      expect(container.firstChild).toBeInTheDocument();
    });
  });

  it('calls onDismiss when dismiss button clicked', () => {
    const onDismiss = vi.fn();
    render(
      <Alert dismissible onDismiss={onDismiss}>
        Dismissible alert
      </Alert>
    );
    
    const dismissButton = screen.getByRole('button');
    fireEvent.click(dismissButton);
    expect(onDismiss).toHaveBeenCalled();
  });

  it('does not show dismiss button when not dismissible', () => {
    render(<Alert>Non-dismissible alert</Alert>);
    expect(screen.queryByRole('button')).not.toBeInTheDocument();
  });
});
