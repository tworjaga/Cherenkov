import { describe, it, expect } from 'vitest';
import {
  groupBy,
  unique,
  uniqueBy,
  sortBy,
  chunk,
  flatten,
  intersection,
  difference,
  partition,
  last,
  first,
  sum,
  average,
  max,
  min,
  moveItem,
  insertAt,
  removeAt,
  toggleItem,
} from './arrays';

describe('arrays', () => {
  describe('groupBy', () => {
    it('groups items by key function', () => {
      const items = [
        { type: 'a', value: 1 },
        { type: 'b', value: 2 },
        { type: 'a', value: 3 },
      ];
      const result = groupBy(items, (item) => item.type);
      expect(result).toEqual({
        a: [
          { type: 'a', value: 1 },
          { type: 'a', value: 3 },
        ],
        b: [{ type: 'b', value: 2 }],
      });
    });
  });

  describe('unique', () => {
    it('removes duplicate values', () => {
      expect(unique([1, 2, 2, 3, 3, 3])).toEqual([1, 2, 3]);
    });
  });

  describe('uniqueBy', () => {
    it('removes duplicates by key function', () => {
      const items = [
        { id: 1, name: 'a' },
        { id: 2, name: 'b' },
        { id: 1, name: 'c' },
      ];
      expect(uniqueBy(items, (item) => item.id)).toEqual([
        { id: 1, name: 'a' },
        { id: 2, name: 'b' },
      ]);
    });
  });

  describe('sortBy', () => {
    it('sorts by key in ascending order', () => {
      const items = [{ value: 3 }, { value: 1 }, { value: 2 }];
      expect(sortBy(items, (item) => item.value)).toEqual([
        { value: 1 },
        { value: 2 },
        { value: 3 },
      ]);
    });

    it('sorts by key in descending order', () => {
      const items = [{ value: 3 }, { value: 1 }, { value: 2 }];
      expect(sortBy(items, (item) => item.value, 'desc')).toEqual([
        { value: 3 },
        { value: 2 },
        { value: 1 },
      ]);
    });
  });

  describe('chunk', () => {
    it('splits array into chunks', () => {
      expect(chunk([1, 2, 3, 4, 5], 2)).toEqual([[1, 2], [3, 4], [5]]);
    });
  });

  describe('flatten', () => {
    it('flattens nested arrays', () => {
      expect(flatten([1, [2, 3], 4])).toEqual([1, 2, 3, 4]);
    });
  });

  describe('intersection', () => {
    it('finds common elements', () => {
      expect(intersection([1, 2, 3], [2, 3, 4])).toEqual([2, 3]);
    });
  });

  describe('difference', () => {
    it('finds elements in first array not in second', () => {
      expect(difference([1, 2, 3], [2, 3, 4])).toEqual([1]);
    });
  });

  describe('partition', () => {
    it('splits array by predicate', () => {
      const [evens, odds] = partition([1, 2, 3, 4], (n) => n % 2 === 0);
      expect(evens).toEqual([2, 4]);
      expect(odds).toEqual([1, 3]);
    });
  });

  describe('last', () => {
    it('returns last element', () => {
      expect(last([1, 2, 3])).toBe(3);
    });

    it('returns undefined for empty array', () => {
      expect(last([])).toBeUndefined();
    });
  });

  describe('first', () => {
    it('returns first element', () => {
      expect(first([1, 2, 3])).toBe(1);
    });

    it('returns undefined for empty array', () => {
      expect(first([])).toBeUndefined();
    });
  });

  describe('sum', () => {
    it('calculates sum of numbers', () => {
      expect(sum([1, 2, 3, 4])).toBe(10);
    });

    it('returns 0 for empty array', () => {
      expect(sum([])).toBe(0);
    });
  });

  describe('average', () => {
    it('calculates average of numbers', () => {
      expect(average([1, 2, 3, 4])).toBe(2.5);
    });

    it('returns 0 for empty array', () => {
      expect(average([])).toBe(0);
    });
  });

  describe('max', () => {
    it('finds maximum value', () => {
      expect(max([1, 3, 2])).toBe(3);
    });

    it('returns undefined for empty array', () => {
      expect(max([])).toBeUndefined();
    });
  });

  describe('min', () => {
    it('finds minimum value', () => {
      expect(min([3, 1, 2])).toBe(1);
    });

    it('returns undefined for empty array', () => {
      expect(min([])).toBeUndefined();
    });
  });

  describe('moveItem', () => {
    it('moves item from one index to another', () => {
      expect(moveItem([1, 2, 3, 4], 0, 2)).toEqual([2, 3, 1, 4]);
    });
  });

  describe('insertAt', () => {
    it('inserts item at specified index', () => {
      expect(insertAt([1, 2, 3], 1, 99)).toEqual([1, 99, 2, 3]);
    });
  });

  describe('removeAt', () => {
    it('removes item at specified index', () => {
      expect(removeAt([1, 2, 3], 1)).toEqual([1, 3]);
    });
  });

  describe('toggleItem', () => {
    it('adds item if not present', () => {
      expect(toggleItem([1, 2], 3)).toEqual([1, 2, 3]);
    });

    it('removes item if present', () => {
      expect(toggleItem([1, 2, 3], 2)).toEqual([1, 3]);
    });
  });
});
