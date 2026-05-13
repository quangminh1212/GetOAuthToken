import { kiroProvider } from './kiro';

export const tokenProviders = [
  kiroProvider,
];

export const defaultProvider = tokenProviders[0];

export function findProvider(providerId) {
  return tokenProviders.find((provider) => provider.id === providerId) || defaultProvider;
}
