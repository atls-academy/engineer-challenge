import { GraphQLClient } from 'graphql-request'

let clientInstance: GraphQLClient | null = null

export const getClient = (): GraphQLClient => {
  if (!clientInstance) {
    const url = globalThis.window === undefined
      ? process.env.NEXT_INTERNAL_GRAPHQL_URL!
      : process.env.NEXT_PUBLIC_GRAPHQL_URL!

    clientInstance = new GraphQLClient(url, {
      credentials: 'include',
    })
  }
  return clientInstance
}
