import { GraphQLClient } from 'graphql-request'

let clientInstance: GraphQLClient | null = null

export const getClient = (): GraphQLClient => {
  if (!clientInstance) {
    clientInstance = new GraphQLClient(process.env.NEXT_PUBLIC_GRAPHQL_URL!, {
      credentials: 'include',
    })
  }
  return clientInstance
}
