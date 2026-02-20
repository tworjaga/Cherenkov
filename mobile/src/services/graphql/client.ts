import { ApolloClient, InMemoryCache, createHttpLink, split } from '@apollo/client';
import { GraphQLWsLink } from '@apollo/client/link/subscriptions';
import { getMainDefinition } from '@apollo/client/utilities';
import { createClient } from 'graphql-ws';
import { setContext } from '@apollo/client/link/context';
import { useAuthStore } from '@stores';

const httpLink = createHttpLink({
  uri: process.env.API_URL || 'https://api.cherenkov.io/graphql',
});

const wsLink = new GraphQLWsLink(createClient({
  url: process.env.WS_URL || 'wss://api.cherenkov.io/graphql',
  connectionParams: () => {
    const token = useAuthStore.getState().token;
    return {
      authorization: token ? `Bearer ${token}` : '',
    };
  },
}));

const authLink = setContext((_, { headers }) => {
  const token = useAuthStore.getState().token;
  return {
    headers: {
      ...headers,
      authorization: token ? `Bearer ${token}` : '',
    },
  };
});

const splitLink = split(
  ({ query }) => {
    const definition = getMainDefinition(query);
    return (
      definition.kind === 'OperationDefinition' &&
      definition.operation === 'subscription'
    );
  },
  wsLink,
  authLink.concat(httpLink)
);

export const apolloClient = new ApolloClient({
  link: splitLink,
  cache: new InMemoryCache({
    typePolicies: {
      Query: {
        fields: {
          sensors: {
            merge(existing = [], incoming) {
              return incoming;
            },
          },
          alerts: {
            merge(existing = [], incoming) {
              return incoming;
            },
          },
        },
      },
    },
  }),
  defaultOptions: {
    watchQuery: {
      fetchPolicy: 'cache-and-network',
    },
  },
});
