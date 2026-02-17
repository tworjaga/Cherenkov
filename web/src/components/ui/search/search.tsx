'use client';

import * as React from 'react';
import { Search as SearchIcon, X } from 'lucide-react';
import { cn } from '@/lib/utils';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';

interface SearchProps {
  value?: string;
  onChange?: (value: string) => void;
  onSearch?: (value: string) => void;
  placeholder?: string;
  className?: string;
  autoFocus?: boolean;
}

export const Search = ({
  value,
  onChange,
  onSearch,
  placeholder = 'Search...',
  className,
  autoFocus = false,
}: SearchProps) => {
  const [internalValue, setInternalValue] = React.useState(value || '');
  const inputRef = React.useRef<HTMLInputElement>(null);

  const currentValue = value !== undefined ? value : internalValue;

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newValue = e.target.value;
    if (value === undefined) {
      setInternalValue(newValue);
    }
    onChange?.(newValue);
  };

  const handleClear = () => {
    if (value === undefined) {
      setInternalValue('');
    }
    onChange?.('');
    inputRef.current?.focus();
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      onSearch?.(currentValue);
    }
  };

  return (
    <div className={cn('relative flex items-center', className)}>
      <SearchIcon className="absolute left-3 h-4 w-4 text-text-tertiary pointer-events-none" />
      <Input
        ref={inputRef}
        type="text"
        value={currentValue}
        onChange={handleChange}
        onKeyDown={handleKeyDown}
        placeholder={placeholder}
        autoFocus={autoFocus}
        className="pl-9 pr-9 bg-bg-secondary border-border-default focus:border-accent-primary"
      />
      {currentValue && (
        <Button
          variant="ghost"
          size="sm"
          onClick={handleClear}
          className="absolute right-1 h-6 w-6 p-0 text-text-tertiary hover:text-text-primary"
        >
          <X className="h-4 w-4" />
        </Button>
      )}
    </div>
  );
};

export default Search;
