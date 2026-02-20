'use client';

import React from 'react';

export function SkipLink(): JSX.Element {
  return (
    <a
      href="#main-content"
      className="
        sr-only focus:not-sr-only
        fixed top-4 left-4 z-[100]
        px-4 py-2
        bg-accent-primary text-white
        rounded-md
        font-medium text-sm
        focus:outline-none focus:ring-2 focus:ring-accent-primary focus:ring-offset-2 focus:ring-offset-bg-primary
        transition-all duration-200
      "
      aria-label="Skip to main content"
    >
      Skip to main content
    </a>
  );
}
