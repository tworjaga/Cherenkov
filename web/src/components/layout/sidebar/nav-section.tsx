'use client';

import { ReactNode } from 'react';

interface NavSectionProps {
  title?: string;
  children: ReactNode;
  collapsed?: boolean;
}

export const NavSection = ({ title, children, collapsed = false }: NavSectionProps) => {
  return (
    <div className="mb-4">
      {!collapsed && title && (
        <div className="px-4 py-2 text-heading-xs text-text-tertiary">
          {title}
        </div>
      )}
      <div className="space-y-1">
        {children}
      </div>
    </div>
  );
};
