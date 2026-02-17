'use client';

import * as React from 'react';
import { format } from 'date-fns';
import { Calendar as CalendarIcon } from 'lucide-react';
import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';
import { Modal, ModalContent, ModalHeader, ModalTitle, ModalTrigger } from '@/components/ui/modal';

interface DatePickerProps {
  date?: Date;
  onSelect?: (date: Date) => void;
  placeholder?: string;
  className?: string;
}

export const DatePicker = ({
  date,
  onSelect,
  placeholder = 'Pick a date',
  className,
}: DatePickerProps) => {
  const [selectedDate, setSelectedDate] = React.useState<Date | undefined>(date);

  const handleSelect = (date: Date) => {
    setSelectedDate(date);
    onSelect?.(date);
  };

  const days = React.useMemo(() => {
    const today = new Date();
    const year = today.getFullYear();
    const month = today.getMonth();
    const firstDay = new Date(year, month, 1);
    const lastDay = new Date(year, month + 1, 0);
    const daysInMonth = lastDay.getDate();
    const startDayOfWeek = firstDay.getDay();

    const days: (number | null)[] = [];
    for (let i = 0; i < startDayOfWeek; i++) {
      days.push(null);
    }
    for (let i = 1; i <= daysInMonth; i++) {
      days.push(i);
    }
    return days;
  }, []);

  const weekDays = ['Su', 'Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa'];

  return (
    <Modal>
      <ModalTrigger asChild>
        <Button
          variant="outline"
          className={cn(
            'w-[280px] justify-start text-left font-normal',
            !selectedDate && 'text-text-tertiary',
            className
          )}
        >
          <CalendarIcon className="mr-2 h-4 w-4" />
          {selectedDate ? format(selectedDate, 'PPP') : placeholder}
        </Button>
      </ModalTrigger>
      <ModalContent className="w-auto p-0">
        <ModalHeader className="border-b border-border-subtle px-4 py-3">
          <ModalTitle>Select Date</ModalTitle>
        </ModalHeader>
        <div className="p-4">
          <div className="grid grid-cols-7 gap-1 text-center text-xs text-text-tertiary mb-2">
            {weekDays.map((day) => (
              <div key={day} className="w-8 h-8 flex items-center justify-center">
                {day}
              </div>
            ))}
          </div>
          <div className="grid grid-cols-7 gap-1">
            {days.map((day, index) => (
              <button
                key={index}
                disabled={!day}
                onClick={() => day && handleSelect(new Date(new Date().getFullYear(), new Date().getMonth(), day))}
                className={cn(
                  'w-8 h-8 rounded-md text-sm flex items-center justify-center transition-colors',
                  !day && 'invisible',
                  day && 'hover:bg-bg-hover',
                  selectedDate?.getDate() === day && 'bg-accent-primary text-bg-primary'
                )}
              >
                {day}
              </button>
            ))}
          </div>
        </div>
      </ModalContent>
    </Modal>
  );
};

export default DatePicker;
