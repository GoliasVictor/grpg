import { keepPreviousData, useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import createClient from "openapi-fetch";
import type { components, paths } from "~/lib/api/specs";
import NodesService from "~/lib/services/nodes-service";

export const client = createClient<paths>({ baseUrl: import.meta.env.VITE_API_URL });


type Filter = {
  predicate: number | null;
  direction: "in" | "out" | "any";
  anotherNode: number | null;
}
export const setting_id = 1;
export const QueriesKeys = {
  nodes: ['nodes'],
  predicates: ['predicates'],
  table: (tableId: number) => ['table', { tableId }],
  anyTable: ['table'],
  homeTable: ['home-table'],
  homeNodes: (filter: Filter) => ['home-nodes', { filter }],
  AnyfullTable: ['table'],
  fullTable: (tableId: number) => (
    ['table', { tableId }]
  ),
}
export function usePredicateQuery() {
  const predicatesQuery = useQuery({
    queryKey: QueriesKeys.predicates,
    queryFn: async () => (await client.GET("/settings/{setting_id}/predicates", {
      params: {
        path: {
           setting_id
        }
      }
    }))?.data,
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

// export function useTableInOutQuery(isIn: boolean, nodeId: number) {
//   const query = useQuery({
//     queryKey: QueriesKeys.table(isIn, nodeId),
//     queryFn: async () => (await client.POST("/table", {
//       body: {
//         node_id: nodeId,
//         predicate: null,
//         direction: isIn ? "in" : "out",
//       }
//     }))?.data,
//     //refetchInterval: 1500,
//   })
//   return query;
// }

export function useTableQuery(tableId: number) {
  return useQuery({
    queryKey: QueriesKeys.fullTable(tableId),
    queryFn: async () => (
      (await client.GET("/settings/{setting_id}/table/{id}", {
        params: {
          path: {
            setting_id: setting_id,
            id: tableId
          }
        }
      }
      ))?.data
    ),
    placeholderData: keepPreviousData,
    // refetchInterval: 1500,
  })
}

export function useTripleCreateMutation() {
  const queryClient = useQueryClient();
  const mutatation = useMutation({
    mutationFn: async (data: { objectId: number, predicateId: number, subjectId: number }) => {
      queryClient.cancelQueries({ queryKey: QueriesKeys.homeTable });
      queryClient.cancelQueries({ queryKey: QueriesKeys.AnyfullTable });
      await client.POST("/settings/{setting_id}/triple", {
        params: {
          path: {
            setting_id
          }
        },
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
      await client.DELETE("/settings/{setting_id}/triple", {
        params: {
          path: {
            setting_id
          }
        },
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


export function useTablesQuery() {
  return useQuery({
    queryKey: QueriesKeys.anyTable,
    queryFn: async () => (await client.GET("/settings/{setting_id}/tables", {
      params: {
        path: {
          setting_id
        }
      },
    }))?.data,
  })
}


export function useTable(tableId: number) {
  return useQuery({
    queryKey: QueriesKeys.anyTable,
    queryFn: async () => (await client.GET("/settings/{setting_id}/tables", {
      params: {
        path: {
          setting_id
        }
      },
    }))?.data,
    select: (data) => {
      return data?.find((t) => t.id === tableId);
    },
  })
}

export function useTableUpdateMutation() {
  const queryClient = useQueryClient();
  const mutatation = useMutation({
    mutationFn: async (data: { tableId : number,def: components["schemas"]["TableDefinition"] }) => {
      queryClient.cancelQueries({ queryKey: QueriesKeys.homeTable });
      await client.PUT("/settings/{setting_id}/table/{id}", {
        params: {
          path: {
            setting_id: setting_id,
            id: data.tableId
          }
        },
        body: data.def
      })
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: QueriesKeys.anyTable });
      queryClient.invalidateQueries({ queryKey: QueriesKeys.AnyfullTable });
    },
  })
  return mutatation
}

export function useTableCreateMutation() {
  const queryClient = useQueryClient();
  const mutatation = useMutation({
    mutationFn: async (data: { def: components["schemas"]["TableDefinition"] }) => {
      queryClient.cancelQueries({ queryKey: QueriesKeys.anyTable });
      await client.POST("/settings/{setting_id}/table", {
        params: {
          path: {
            setting_id
          }
        },
        body: data.def
      })
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: QueriesKeys.anyTable });
      queryClient.invalidateQueries({ queryKey: QueriesKeys.AnyfullTable });
    },
  })
  return mutatation;
}

export function useTableDeleteMutation() {
  const queryClient = useQueryClient();
  const mutation = useMutation({
    mutationFn: async (data: { tableId: number }) => {
      queryClient.cancelQueries({ queryKey: QueriesKeys.anyTable });
      await client.DELETE("/settings/{setting_id}/tables/{id}", {
        params: {
          path: {
            setting_id: setting_id,
            id: data.tableId
          }
        }
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: QueriesKeys.anyTable });
      queryClient.invalidateQueries({ queryKey: QueriesKeys.AnyfullTable });
    },
  });
  return mutation;
}
