import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import createClient from "openapi-fetch";
import type { paths } from "~/lib/api/specs";

export const client = createClient<paths>({ baseUrl: "http://127.0.0.1:8000/" });

export function usePredicateQuery() {
  const predicatesQuery = useQuery({
    queryKey: ['predicates'],
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

export function useNodesQuery() {
  const query = useQuery({
    queryKey: ['nodes'],
    queryFn: async () => (await client.GET("/node"))?.data,
    staleTime: Infinity,
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
        direction: isIn ? "in" : "out",
      }
    }))?.data,
    //refetchInterval: 1500,
  })
  return query;
}

type Node = {
  node_id: number;
  label: string;
}
export function useNodesUpdateMutation() {
  const queryClient = useQueryClient();
  const mutatation = useMutation({
    mutationFn: async (data: { nodeId: number, label: string }) => {
      queryClient.cancelQueries({ queryKey: ['nodes'] });
      await client.PUT("/node/{node_id}", {
        params: {
          path: { node_id: data.nodeId },
          query: { label: data.label }
        }
      })
      console.log(queryClient.getQueriesData({ queryKey: [] }));
    },
    onSuccess: () => {

      queryClient.invalidateQueries({ queryKey: ['nodes'] });
    },
  })
  return mutatation
}

export function useNodesCreateMutation() {
  const queryClient = useQueryClient();
  const mutatation = useMutation({
    mutationFn: async (data: { label: string }) => {
      queryClient.cancelQueries({ queryKey: ['nodes'] });
      await client.POST("/node", {
        params: {
          query: data
        }
      })
    },
    onSuccess: () => {
      queryClient.refetchQueries({ queryKey: ['nodes'] });
      queryClient.invalidateQueries({ queryKey: ['home-table'] });

    },
  })
  return mutatation
}

export function useNodesDeleteMutation() {
  const queryClient = useQueryClient();
  const mutatation = useMutation({
    mutationFn: async (data: { nodeId: number }) => {
      queryClient.cancelQueries({ queryKey: ['nodes'] });
      await client.DELETE("/node/{node_id}", {
        params: {
          path: { node_id: data.nodeId }
        }
      })
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['nodes'] });
      queryClient.invalidateQueries({ queryKey: ['home-table'] });
    }
  })
  return mutatation
}

export function useTripleCreateMutation() {
  const queryClient = useQueryClient();
  const mutatation = useMutation({
    mutationFn: async (data: { objectId: number, predicateId: number, subjectId: number }) => {
      queryClient.cancelQueries({ queryKey: ['home-table'] });
      await client.POST("/triple", {
        body: {
          object_id: data.objectId,
          predicate_id: data.predicateId,
          subject_id: data.subjectId
        }
      })
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['home-table'] });
    },
  })
  return mutatation;
}

export function useTripleDeleteMutation() {
  const queryClient = useQueryClient();
  const mutatation = useMutation({
    mutationFn: async (data: { objectId: number, predicateId: number, subjectId: number }) => {
      queryClient.cancelQueries({ queryKey: ['home-table'] });
      await client.DELETE("/triple", {
        body: {
          object_id: data.objectId,
          predicate_id: data.predicateId,
          subject_id: data.subjectId
        }
      })
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['home-table'] });
    },
  })
  return mutatation;
}
