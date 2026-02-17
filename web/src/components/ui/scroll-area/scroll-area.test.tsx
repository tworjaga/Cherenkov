import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { ScrollArea } from './scroll-area';

describe('ScrollArea', () => {
  it('renders scroll area with content', () => {
    render(
      <ScrollArea className="h-32">
        <div data-testid="content">Scrollable content</div>
      </ScrollArea>
    );
    expect(screen.getByTestId('content')).toBeInTheDocument();
  });

  it('applies custom className', () => {
    render(
      <ScrollArea className="custom-class h-32">
        <div>Content</div>
      </ScrollArea>
    );
    const scrollArea = document.querySelector('.custom-class');
    expect(scrollArea).toBeInTheDocument();
  });

  it('renders with long content', () => {
    render(
      <ScrollArea className="h-32 w-32">
        <div style={{ height: '400px' }}>Long content</div>
      </ScrollArea>
    );
    expect(screen.getByText('Long content')).toBeInTheDocument();
  });

  it('renders with wide content', () => {
    render(
      <ScrollArea className="h-32 w-32">
        <div style={{ width: '400px' }}>Wide content</div>
      </ScrollArea>
    );
    expect(screen.getByText('Wide content')).toBeInTheDocument();
  });

});
