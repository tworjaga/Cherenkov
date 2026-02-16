import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { ToastItem } from './Toast';

describe('ToastItem', () => {

  const mockOnClose = vi.fn();

  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
    mockOnClose.mockClear();
  });

  it('renders with message', () => {
    render(
      <ToastItem
        id="1"
        title="Test"
        message="Test message"
        severity="info"
        onClose={mockOnClose}
      />
    );
    expect(screen.getByText('Test message')).toBeInTheDocument();
  });

  it('renders with correct severity colors - critical', () => {
    render(
      <ToastItem
        id="1"
        title="Test"
        message="Critical alert"
        severity="critical"
        onClose={mockOnClose}
      />
    );
    const toast = screen.getByRole('alert');
    expect(toast).toHaveClass('border-alert-critical');
  });

  it('renders with correct severity colors - high', () => {
    render(
      <ToastItem
        id="1"
        title="Test"
        message="High alert"
        severity="high"
        onClose={mockOnClose}
      />
    );
    const toast = screen.getByRole('alert');
    expect(toast).toHaveClass('border-alert-high');
  });

  it('renders with correct severity colors - medium', () => {
    render(
      <ToastItem
        id="1"
        title="Test"
        message="Medium alert"
        severity="medium"
        onClose={mockOnClose}
      />
    );
    const toast = screen.getByRole('alert');
    expect(toast).toHaveClass('border-alert-medium');
  });

  it('renders with correct severity colors - low', () => {
    render(
      <ToastItem
        id="1"
        title="Test"
        message="Low alert"
        severity="low"
        onClose={mockOnClose}
      />
    );
    const toast = screen.getByRole('alert');
    expect(toast).toHaveClass('border-alert-low');
  });

  it('renders with correct severity colors - info', () => {
    render(
      <ToastItem
        id="1"
        title="Test"
        message="Info message"
        severity="info"
        onClose={mockOnClose}
      />
    );
    const toast = screen.getByRole('alert');
    expect(toast).toHaveClass('border-text-secondary');
  });

  it('calls onClose when close button clicked', () => {
    render(
      <ToastItem
        id="1"
        title="Test"
        message="Test message"
        severity="info"
        onClose={mockOnClose}
      />
    );
    const closeButton = screen.getByLabelText('Close notification');
    fireEvent.click(closeButton);
    expect(mockOnClose).toHaveBeenCalledWith('1');
  });

  it('auto-dismisses after duration', async () => {
    vi.useRealTimers();
    render(
      <ToastItem
        id="1"
        title="Test"
        message="Test message"
        severity="info"
        duration={50}
        onClose={mockOnClose}
      />
    );
    
    await waitFor(() => {
      expect(mockOnClose).toHaveBeenCalledWith('1');
    }, { timeout: 200 });
  }, 1000);



  it('does not auto-dismiss when duration is 0', () => {
    render(
      <ToastItem
        id="1"
        title="Test"
        message="Test message"
        severity="info"
        duration={0}
        onClose={mockOnClose}
      />
    );
    
    vi.advanceTimersByTime(10000);
    
    expect(mockOnClose).not.toHaveBeenCalled();
  });

  it('clears timeout on unmount', () => {
    const { unmount } = render(
      <ToastItem
        id="1"
        title="Test"
        message="Test message"
        severity="info"
        duration={3000}
        onClose={mockOnClose}
      />
    );
    
    unmount();
    vi.advanceTimersByTime(3000);
    
    expect(mockOnClose).not.toHaveBeenCalled();
  });

  it('has correct ARIA attributes', () => {
    render(
      <ToastItem
        id="1"
        title="Test"
        message="Test message"
        severity="critical"
        onClose={mockOnClose}
      />
    );
    const toast = screen.getByRole('alert');
    expect(toast).toHaveAttribute('aria-live', 'assertive');
  });

  it('renders with title when provided', () => {
    render(
      <ToastItem
        id="1"
        title="Notification Title"
        message="Test message"
        severity="info"
        onClose={mockOnClose}
      />
    );
    expect(screen.getByText('Notification Title')).toBeInTheDocument();
    expect(screen.getByText('Test message')).toBeInTheDocument();
  });
});
