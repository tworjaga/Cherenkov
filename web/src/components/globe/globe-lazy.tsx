'use client';

import { Suspense, lazy, ComponentProps } from 'react';
import { Skeleton } from '@/components/ui/skeleton';

// Lazy load the Globe component to reduce initial bundle size
const GlobeComponent = lazy(() => import('./globe').then(mod => ({ default: mod.Globe })));

type GlobeProps = ComponentProps<typeof import('./globe').Globe>;


export const GlobeLazy = (props: GlobeProps) => {

  return (
    <Suspense 
      fallback={
        <div className="relative w-full h-full bg-bg-primary flex items-center justify-center">
          <div className="flex flex-col items-center gap-4">
            <Skeleton className="w-16 h-16 rounded-full" />
            <Skeleton className="w-32 h-4" />
            <span className="text-text-secondary text-body-sm">Loading Globe...</span>
          </div>
        </div>
      }
    >
      <GlobeComponent {...props} />
    </Suspense>
  );
};

GlobeLazy.displayName = 'GlobeLazy';
