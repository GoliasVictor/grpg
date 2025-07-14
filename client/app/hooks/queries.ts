import { keepPreviousData, useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import createClient from "openapi-fetch";
import type { components, paths } from "~/lib/api/specs";

export const client = createClient<paths>({ baseUrl: "http://127.0.0.1:8000/" });


type Filter = {
  predicate: number | null;
  direction: "in" | "out" | "any";
  anotherNode: number | null;
}
const QueriesKeys = {
  nodes: ['nodes'],
  predicates: ['predicates'],
  table: (isIn: boolean, nodeId: number) => ['table', { isIn, nodeId }],
  homeTable: ['home-table'],
  homeNodes: (filter: Filter) => ['home-nodes', { filter }],
  AnyfullTable: ['full-table'],
  fullTable: (filter: Filter, columns: components["schemas"]["ColumnDefinition"][]) => (
    ['full-table', { filter, columns }]
  ),
}
export function usePredicateQuery() {
  const predicatesQuery = useQuery({
    queryKey: QueriesKeys.predicates,
    queryFn: async () => (await client.GET("/predicates"))?.data,
    staleTime: Infinity
  })

  return {
    ...predicatesQuery,
    predicates: predicatesQuery.data || [],
    getPredicate: (id: number) => {
      return predicatesQuery.data?.find((p) => p.id === id);
    }
  };
}

export function useNodesQuery({subscribed} : {subscribed?: boolean} = { }) {
  const query = useQuery({
    queryKey:  QueriesKeys.nodes,
    queryFn: async () => (await client.GET("/node"))?.data,
    staleTime: Infinity,
    subscribed: subscribed,
  })
  return {
    ...query,
    nodes: query.data || [],
    getNode: (id: number) => {
      return query.data?.find((n) => n.node_id === id);
    }
  }
}


export function useOneNodeQuery(id : number) {
  return useQuery({
    queryKey: QueriesKeys.nodes,
    queryFn: async () => (await client.GET("/node"))?.data,
    staleTime: Infinity,
    select: (data) => {
      return data?.find((n: { node_id: number }) => n.node_id === id);
    }
  })


}
export function useTableInOutQuery(isIn: boolean, nodeId: number) {
  const query = useQuery({
    queryKey: QueriesKeys.table(isIn, nodeId),
    queryFn: async () => (await client.POST("/table", {
      body: {
        node_id: nodeId,
        predicate: null,
        direction: isIn ? "in" : "out",
      }
    }))?.data,
    //refetchInterval: 1500,
  })
  return query;
}
export function useFilterdNodesQuery(filter: Filter) {
  return useQuery({
    queryKey: ['home-nodes', { filter: filter }],
    queryFn: async () => {
      return (await client.POST("/table", {
        body: {
          direction: filter.direction === "any" ? null : filter.direction,
          predicate: filter.predicate ?? null,
          node_id: filter.anotherNode ?? null
        }
      }))?.data
    },
    //refetchInterval: 1000,
    select: (data) => new Set((data ?? []).map(d => d.node_id))
  });
}

export function useTableQuery(filter: Filter, columns : components["schemas"]["ColumnDefinition"][]) {
  return useQuery({
    queryKey: QueriesKeys.fullTable(filter, columns),
    queryFn: async () => (
      (await client.POST("/full-table", {
        body: {
          filter: {
            direction: filter.direction === "any" ? null : filter.direction,
            predicate: filter.predicate ?? null,
            node_id: filter.anotherNode ?? null
          },
          columns: columns
        }
      }
      ))?.data
    ),
    placeholderData: keepPreviousData,
    //refetchInterval: 1500,
  })
}
type Node = {
  node_id: number;
  label: string;
}
export function useNodesUpdateMutation() {
  const queryClient = useQueryClient();
  const mutatation = useMutation({
    mutationFn: async (data: { nodeId: number, label: string }) => {
      queryClient.cancelQueries({ queryKey: QueriesKeys.homeTable });
      await client.PUT("/node/{node_id}", {
        params: {
          path: { node_id: data.nodeId },
          query: { label: data.label }
        }
      })
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: QueriesKeys.nodes });
    },
  })
  return mutatation
}

export function useNodesCreateMutation() {
  const queryClient = useQueryClient();
  const mutatation = useMutation({
    mutationFn: async (data: { label: string }) => {
      queryClient.cancelQueries({ queryKey: QueriesKeys.nodes });
      await client.POST("/node", {
        params: {
          query: data
        }
      })
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: QueriesKeys.nodes });
      queryClient.invalidateQueries({ queryKey: QueriesKeys.homeTable });
      queryClient.invalidateQueries({ queryKey: QueriesKeys.AnyfullTable });
    },
  })
  return mutatation
}

export function useNodesDeleteMutation() {
  const queryClient = useQueryClient();
  const mutatation = useMutation({
    mutationFn: async (data: { nodeId: number }) => {
      queryClient.cancelQueries({ queryKey: QueriesKeys.nodes });
      await client.DELETE("/node/{node_id}", {
        params: {
          path: { node_id: data.nodeId }
        }
      })
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: QueriesKeys.nodes });
      queryClient.invalidateQueries({ queryKey: QueriesKeys.homeTable });
      queryClient.invalidateQueries({ queryKey: QueriesKeys.AnyfullTable });
    }
  })
  return mutatation
}

export function useTripleCreateMutation() {
  const queryClient = useQueryClient();
  const mutatation = useMutation({
    mutationFn: async (data: { objectId: number, predicateId: number, subjectId: number }) => {
      queryClient.cancelQueries({ queryKey: QueriesKeys.homeTable });
      queryClient.cancelQueries({ queryKey: QueriesKeys.AnyfullTable });
      await client.POST("/triple", {
        body: {
          object_id: data.objectId,
          predicate_id: data.predicateId,
          subject_id: data.subjectId
        }
      })
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: QueriesKeys.homeTable });
      queryClient.invalidateQueries({ queryKey: QueriesKeys.AnyfullTable });
    },
  })
  return mutatation;
}

export function useTripleDeleteMutation() {
  const queryClient = useQueryClient();
  const mutatation = useMutation({
    mutationFn: async (data: { objectId: number, predicateId: number, subjectId: number }) => {
      queryClient.cancelQueries({ queryKey: QueriesKeys.homeTable });
      await client.DELETE("/triple", {
        body: {
          object_id: data.objectId,
          predicate_id: data.predicateId,
          subject_id: data.subjectId
        }
      })
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: QueriesKeys.homeTable });
      queryClient.invalidateQueries({ queryKey: QueriesKeys.AnyfullTable });
    },
  })
  return mutatation;
}
