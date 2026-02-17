import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export * from './formatters';
export * from './validators';
export * from './calculations';
export * from './arrays';
export * from './dates';


export * from './validators';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}
