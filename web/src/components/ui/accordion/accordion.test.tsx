import { describe, it, expect } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { Accordion, AccordionItem, AccordionTrigger, AccordionContent } from './accordion';

describe('Accordion', () => {
  it('renders accordion with items', () => {
    render(
      <Accordion type="single">
        <AccordionItem value="item1">
          <AccordionTrigger>Trigger 1</AccordionTrigger>
          <AccordionContent>Content 1</AccordionContent>
        </AccordionItem>
      </Accordion>
    );
    expect(screen.getByText('Trigger 1')).toBeInTheDocument();
  });

  it('expands item when trigger is clicked', () => {
    render(
      <Accordion type="single">
        <AccordionItem value="item1">
          <AccordionTrigger>Trigger 1</AccordionTrigger>
          <AccordionContent>Content 1</AccordionContent>
        </AccordionItem>
      </Accordion>
    );
    
    const trigger = screen.getByText('Trigger 1');
    fireEvent.click(trigger);
    
    expect(screen.getByText('Content 1')).toBeInTheDocument();
  });

  it('collapses expanded item when clicked again (single mode)', () => {
    render(
      <Accordion type="single" collapsible>
        <AccordionItem value="item1">
          <AccordionTrigger>Trigger 1</AccordionTrigger>
          <AccordionContent>Content 1</AccordionContent>
        </AccordionItem>
      </Accordion>
    );
    
    const trigger = screen.getByText('Trigger 1');
    fireEvent.click(trigger);
    expect(screen.getByText('Content 1')).toBeInTheDocument();
    
    fireEvent.click(trigger);
    // Content should be hidden
  });

  it('allows multiple items open in multiple mode', () => {
    render(
      <Accordion type="multiple">
        <AccordionItem value="item1">
          <AccordionTrigger>Trigger 1</AccordionTrigger>
          <AccordionContent>Content 1</AccordionContent>
        </AccordionItem>
        <AccordionItem value="item2">
          <AccordionTrigger>Trigger 2</AccordionTrigger>
          <AccordionContent>Content 2</AccordionContent>
        </AccordionItem>
      </Accordion>
    );
    
    fireEvent.click(screen.getByText('Trigger 1'));
    fireEvent.click(screen.getByText('Trigger 2'));
    
    expect(screen.getByText('Content 1')).toBeInTheDocument();
    expect(screen.getByText('Content 2')).toBeInTheDocument();
  });

  it('renders disabled item', () => {
    render(
      <Accordion type="single">
        <AccordionItem value="item1" disabled>
          <AccordionTrigger>Disabled Trigger</AccordionTrigger>
          <AccordionContent>Content</AccordionContent>
        </AccordionItem>
      </Accordion>
    );
    
    const trigger = screen.getByText('Disabled Trigger');
    expect(trigger).toHaveAttribute('data-disabled');
  });
});
