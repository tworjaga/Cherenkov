/**
 * Date utility functions
 */

/**
 * Formats a date to ISO string without milliseconds
 */
export function toISOString(date: Date): string {
  return date.toISOString().replace(/\.\d{3}Z$/, 'Z');
}

/**
 * Parses an ISO string to Date object
 */
export function fromISOString(isoString: string): Date {
  return new Date(isoString);
}

/**
 * Formats date to display string
 */
export function formatDate(
  date: Date | string | number,
  options: Intl.DateTimeFormatOptions = {}
): string {
  const d = typeof date === 'string' ? new Date(date) : 
            typeof date === 'number' ? new Date(date) : date;
  
  return d.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    ...options,
  });
}

/**
 * Formats time to display string
 */
export function formatTime(
  date: Date | string | number,
  options: Intl.DateTimeFormatOptions = {}
): string {
  const d = typeof date === 'string' ? new Date(date) : 
            typeof date === 'number' ? new Date(date) : date;
  
  return d.toLocaleTimeString('en-US', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false,
    ...options,
  });
}

/**
 * Formats date and time together
 */
export function formatDateTime(
  date: Date | string | number,
  options: Intl.DateTimeFormatOptions = {}
): string {
  const d = typeof date === 'string' ? new Date(date) : 
            typeof date === 'number' ? new Date(date) : date;
  
  return d.toLocaleString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false,
    ...options,
  });
}

/**
 * Gets relative time string (e.g., "2 hours ago")
 */
export function getRelativeTime(date: Date | string | number): string {
  const d = typeof date === 'string' ? new Date(date) : 
            typeof date === 'number' ? new Date(date) : date;
  
  const now = new Date();
  const diffMs = now.getTime() - d.getTime();
  const diffSec = Math.floor(diffMs / 1000);
  const diffMin = Math.floor(diffSec / 60);
  const diffHour = Math.floor(diffMin / 60);
  const diffDay = Math.floor(diffHour / 24);

  if (diffSec < 60) {
    return 'just now';
  } else if (diffMin < 60) {
    return `${diffMin}m ago`;
  } else if (diffHour < 24) {
    return `${diffHour}h ago`;
  } else if (diffDay < 7) {
    return `${diffDay}d ago`;
  } else {
    return formatDate(d);
  }
}

/**
 * Adds time to a date using UTC methods for consistency
 */
export function addTime(
  date: Date,
  amount: number,
  unit: 'ms' | 's' | 'm' | 'h' | 'd' | 'w' | 'M' | 'y'
): Date {
  const result = new Date(date);
  
  switch (unit) {
    case 'ms':
      result.setUTCMilliseconds(result.getUTCMilliseconds() + amount);
      break;
    case 's':
      result.setUTCSeconds(result.getUTCSeconds() + amount);
      break;
    case 'm':
      result.setUTCMinutes(result.getUTCMinutes() + amount);
      break;
    case 'h':
      result.setUTCHours(result.getUTCHours() + amount);
      break;
    case 'd':
      result.setUTCDate(result.getUTCDate() + amount);
      break;
    case 'w':
      result.setUTCDate(result.getUTCDate() + amount * 7);
      break;
    case 'M':
      result.setUTCMonth(result.getUTCMonth() + amount);
      break;
    case 'y':
      result.setUTCFullYear(result.getUTCFullYear() + amount);
      break;
  }
  
  return result;
}

/**
 * Gets start of day
 */
export function startOfDay(date: Date): Date {
  const result = new Date(date);
  result.setHours(0, 0, 0, 0);
  return result;
}

/**
 * Gets end of day
 */
export function endOfDay(date: Date): Date {
  const result = new Date(date);
  result.setHours(23, 59, 59, 999);
  return result;
}

/**
 * Gets start of week (Sunday)
 */
export function startOfWeek(date: Date): Date {
  const result = new Date(date);
  const day = result.getDay();
  result.setDate(result.getDate() - day);
  result.setHours(0, 0, 0, 0);
  return result;
}

/**
 * Gets start of month
 */
export function startOfMonth(date: Date): Date {
  const result = new Date(date);
  result.setDate(1);
  result.setHours(0, 0, 0, 0);
  return result;
}

/**
 * Checks if two dates are the same day
 */
export function isSameDay(a: Date, b: Date): boolean {
  return (
    a.getFullYear() === b.getFullYear() &&
    a.getMonth() === b.getMonth() &&
    a.getDate() === b.getDate()
  );
}

/**
 * Checks if date is today
 */
export function isToday(date: Date): boolean {
  return isSameDay(date, new Date());
}

/**
 * Checks if date is in the past
 */
export function isPast(date: Date): boolean {
  return date.getTime() < Date.now();
}

/**
 * Checks if date is in the future
 */
export function isFuture(date: Date): boolean {
  return date.getTime() > Date.now();
}

/**
 * Gets duration between two dates in milliseconds
 */
export function getDurationMs(start: Date, end: Date): number {
  return end.getTime() - start.getTime();
}

/**
 * Gets duration between two dates as a formatted string
 */
export function getDurationString(start: Date, end: Date): string {
  const ms = getDurationMs(start, end);
  const seconds = Math.floor(ms / 1000);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);

  if (days > 0) {
    return `${days}d ${hours % 24}h`;
  } else if (hours > 0) {
    return `${hours}h ${minutes % 60}m`;
  } else if (minutes > 0) {
    return `${minutes}m ${seconds % 60}s`;
  } else {
    return `${seconds}s`;
  }
}

/**
 * Clamps a date between min and max
 */
export function clampDate(date: Date, min: Date, max: Date): Date {
  const time = date.getTime();
  const minTime = min.getTime();
  const maxTime = max.getTime();
  
  if (time < minTime) return new Date(minTime);
  if (time > maxTime) return new Date(maxTime);
  return new Date(time);
}

/**
 * Creates a date range array
 */
export function createDateRange(start: Date, end: Date, stepMs: number): Date[] {
  const dates: Date[] = [];
  let current = new Date(start);
  
  while (current.getTime() <= end.getTime()) {
    dates.push(new Date(current));
    current = new Date(current.getTime() + stepMs);
  }
  
  return dates;
}

/**
 * Formats timestamp for charts
 */
export function formatChartTimestamp(timestamp: number | Date): string {
  const date = typeof timestamp === 'number' ? new Date(timestamp) : timestamp;
  return date.toLocaleTimeString('en-US', {
    hour: '2-digit',
    minute: '2-digit',
    hour12: false,
  });
}

/**
 * Gets UTC timestamp
 */
export function getUTCTimestamp(): number {
  return Date.now();
}

/**
 * Converts local date to UTC
 */
export function toUTC(date: Date): Date {
  return new Date(date.toISOString());
}

/**
 * Converts UTC to local date
 */
export function fromUTC(date: Date): Date {
  return new Date(date.getTime() - date.getTimezoneOffset() * 60000);
}
