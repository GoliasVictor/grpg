import { useQuery } from "@tanstack/react-query";
import createClient from "openapi-fetch";
import type { paths } from "~/lib/api/specs";

export const client = createClient<paths>({ baseUrl: "http://127.0.0.1:8000/" });

export function usePredicateQuery() {
  const predicatesQuery = useQuery({
    queryKey: ['predicates'],
    queryFn: async () => (await client.GET("/predicates"))?.data,
    //refetchInterval: 1500,
  })

  return {
    ...predicatesQuery,
    predicates: predicatesQuery.data || [],
    getPredicate: (id: number) => {
      return predicatesQuery.data?.find((p) => p.id === id);
    }
  };
}

export function useNodesQuery() {
  const query = useQuery({
    queryKey: ['nodes'],
    queryFn: async () => (await client.GET("/node"))?.data,
    //refetchInterval: 1500,
  })
  return {
    ...query,
    nodes: query.data || [],
    getNode: (id: number) => {
      return query.data?.find((n) => n.node_id === id);
    }
  }
}

export function useTableInOutQuery(isIn: boolean, nodeId: number) {
  const query = useQuery({
    queryKey: ['table', { isIn: "in", nodeId: nodeId }],
    queryFn: async () => (await client.POST("/table", {
      body: {
        node_id: nodeId,
        predicate: null,
        direction: isIn ? "in": "out",
      }
    }))?.data,
    //refetchInterval: 1500,
  })
  return query;
}