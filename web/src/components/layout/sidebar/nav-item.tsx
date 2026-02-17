'use client';

import { motion } from 'framer-motion';
import { LucideIcon } from 'lucide-react';
import { cn } from '@/lib/utils';

interface NavItemProps {
  icon: LucideIcon;
  label: string;
  href: string;
  isActive?: boolean;
  isCollapsed?: boolean;
  onClick?: () => void;
}

export function NavItem({
  icon: Icon,
  label,
  isActive = false,
  isCollapsed = false,
  onClick,
}: NavItemProps) {
  return (
    <motion.button
      onClick={onClick}
      className={cn(
        'group relative flex items-center gap-3 rounded-lg px-3 py-2.5 transition-all duration-200',
        isActive
          ? 'bg-[#1a1a25] text-[#00d4ff]'
          : 'text-[#a0a0b0] hover:bg-[#1a1a25] hover:text-white'
      )}
      whileHover={{ scale: 1.02 }}
      whileTap={{ scale: 0.98 }}
    >
      {/* Active indicator */}
      {isActive && (
        <motion.div
          layoutId="activeNav"
          className="absolute left-0 top-1/2 h-6 w-0.5 -translate-y-1/2 rounded-full bg-[#00d4ff]"
          initial={false}
          transition={{ type: 'spring', stiffness: 500, damping: 30 }}
        />
      )}

      <Icon className="h-5 w-5 shrink-0" strokeWidth={isActive ? 2 : 1.5} />

      {!isCollapsed && (
        <span className="text-sm font-medium">{label}</span>
      )}

      {/* Tooltip for collapsed state */}
      {isCollapsed && (
        <div className="absolute left-full ml-2 hidden rounded-md bg-[#12121a] px-2 py-1 text-xs text-white opacity-0 shadow-lg transition-opacity group-hover:block group-hover:opacity-100">
          {label}
        </div>
      )}
    </motion.button>
  );
}
