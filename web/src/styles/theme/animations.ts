export const easing = {
  default: [0.4, 0, 0.2, 1],
  bounce: [0.34, 1.56, 0.64, 1],
  smooth: [0.45, 0, 0.55, 1],
  snap: [0, 0, 0.2, 1],
} as const;

export const duration = {
  fast: 0.15,
  normal: 0.3,
  slow: 0.5,
  slower: 0.8,
} as const;

export const animations = {
  slideIn: {
    initial: { x: '100%', opacity: 0 },
    animate: { x: 0, opacity: 1 },
    transition: { duration: duration.normal, ease: easing.default },
  },
  slideOut: {
    initial: { x: 0, opacity: 1 },
    animate: { x: '100%', opacity: 0 },
    transition: { duration: duration.normal, ease: easing.default },
  },
  fadeIn: {
    initial: { opacity: 0 },
    animate: { opacity: 1 },
    transition: { duration: duration.fast, ease: easing.default },
  },
  pulse: {
    animate: {
      opacity: [1, 0.6, 1],
      scale: [1, 1.02, 1],
    },
    transition: {
      duration: 2,
      repeat: Infinity,
      ease: 'easeInOut',
    },
  },
  pulseFast: {
    animate: {
      opacity: [1, 0.4, 1],
      scale: [1, 1.05, 1],
    },
    transition: {
      duration: 1,
      repeat: Infinity,
      ease: 'easeInOut',
    },
  },
  countUp: {
    transition: { duration: duration.fast, ease: easing.snap },
  },
  flyTo: {
    transition: { duration: duration.slower, ease: easing.smooth },
  },
  hover: {
    scale: 1.02,
    transition: { duration: duration.fast },
  },
  stagger: {
    transition: { staggerChildren: 0.05 },
  },
  glow: {
    animate: {
      boxShadow: [
        '0 0 20px rgba(0, 212, 255, 0.15)',
        '0 0 40px rgba(0, 212, 255, 0.3)',
        '0 0 20px rgba(0, 212, 255, 0.15)',
      ],
    },
    transition: {
      duration: 2,
      repeat: Infinity,
      ease: 'easeInOut',
    },
  },
} as const;

export type Animations = typeof animations;
