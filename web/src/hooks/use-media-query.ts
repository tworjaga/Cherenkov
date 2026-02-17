'use client';

import { useState, useEffect } from 'react';

export const useMediaQuery = (query: string): boolean => {
  const [matches, setMatches] = useState(false);

  useEffect(() => {
    const media = window.matchMedia(query);
    
    const updateMatch = () => {
      setMatches(media.matches);
    };
    
    updateMatch();
    
    media.addEventListener('change', updateMatch);
    
    return () => {
      media.removeEventListener('change', updateMatch);
    };
  }, [query]);

  return matches;
};

export const useBreakpoint = () => {
  const isXs = useMediaQuery('(max-width: 767px)');
  const isSm = useMediaQuery('(min-width: 768px) and (max-width: 1023px)');
  const isMd = useMediaQuery('(min-width: 1024px) and (max-width: 1439px)');
  const isLg = useMediaQuery('(min-width: 1440px) and (max-width: 1919px)');
  const isXl = useMediaQuery('(min-width: 1920px)');

  return {
    isXs,
    isSm,
    isMd,
    isLg,
    isXl,
    isMobile: isXs || isSm,
    isDesktop: isMd || isLg || isXl,
  };
};
