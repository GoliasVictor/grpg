import { keepPreviousData, useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import createClient from "openapi-fetch";
import type { components, paths } from "~/lib/api/specs";
import NodesService from "~/lib/services/nodes-service";
import { workspace_id, QueriesKeys } from "../queries";

export function useNodesQuery({ subscribed }: { subscribed?: boolean } = {}) {
  const nodesService = new NodesService();
  const query = useQuery({
    queryKey:  QueriesKeys.nodes,
    queryFn: async () => await nodesService.getNodes(workspace_id),
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

export function useOneNodeQuery(id: number) {
  const nodesService = new NodesService();
  return useQuery({
    queryKey: QueriesKeys.nodes,
    queryFn: async () => await nodesService.getNodes(workspace_id),
    staleTime: Infinity,
    select: (data) => {
      return data?.find((n: { node_id: number }) => n.node_id === id);
    }
  })
}

export function useNodesUpdateMutation() {
  const queryClient = useQueryClient();
  const nodesService = new NodesService();
  const mutatation = useMutation({
    mutationFn: async (data: { nodeId: number, label: string }) => {
      queryClient.cancelQueries({ queryKey: QueriesKeys.homeTable });
      await nodesService.updateNode(workspace_id, {
        node_id: data.nodeId,
        label: data.label
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: QueriesKeys.nodes });
    },
  })
  return mutatation
}

export function useNodesCreateMutation() {
  const queryClient = useQueryClient();
  const nodesService = new NodesService();
  const mutatation = useMutation({
    mutationFn: async (data: { label: string }) => {
      queryClient.cancelQueries({ queryKey: QueriesKeys.nodes });
      await nodesService.createNode(workspace_id, {
        label: data.label
      });
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
  const nodesService = new NodesService();
  const mutatation = useMutation({
    mutationFn: async (data: { nodeId: number }) => {
      queryClient.cancelQueries({ queryKey: QueriesKeys.nodes });
      nodesService.deleteNode(workspace_id, data.nodeId);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: QueriesKeys.nodes });
      queryClient.invalidateQueries({ queryKey: QueriesKeys.homeTable });
      queryClient.invalidateQueries({ queryKey: QueriesKeys.AnyfullTable });
    }
  })
  return mutatation
}
