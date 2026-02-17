import { create } from '@storybook/theming/create';

export default create({
  base: 'dark',
  brandTitle: 'Cherenkov UI',
  brandUrl: 'https://cherenkov.io',
  brandImage: '/images/logo-dark.svg',
  brandTarget: '_self',
  
  colorPrimary: '#3b82f6',
  colorSecondary: '#10b981',
  
  appBg: '#0f172a',
  appContentBg: '#1e293b',
  appBorderColor: '#334155',
  appBorderRadius: 8,
  
  textColor: '#f8fafc',
  textInverseColor: '#0f172a',
  
  barTextColor: '#94a3b8',
  barSelectedColor: '#3b82f6',
  barBg: '#0f172a',
  
  inputBg: '#1e293b',
  inputBorder: '#334155',
  inputTextColor: '#f8fafc',
  inputBorderRadius: 6,
});
