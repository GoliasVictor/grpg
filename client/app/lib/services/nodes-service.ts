import { keepPreviousData, useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import createClient from "openapi-fetch";
import type { components, paths } from "~/lib/api/specs";

export const client = createClient<paths>({ baseUrl: import.meta.env.VITE_API_URL });

class NodesService {
  async getNodes(workspace_id : number) {
    return (await client.GET("/workspaces/{workspace_id}/node",
      {params:{path: {workspace_id}}}
    )).data
  }
  async createNode(workspace_id: number, data: components["schemas"]["NewNode"]) {
    await client.POST("/workspaces/{workspace_id}/node", {
      params: {
        path: {
          workspace_id
        }
      },
      body: data
    })
  }
  async updateNode(workspace_id: number, def: components["schemas"]["Node"]) {
    await client.PUT("/workspaces/{workspace_id}/node/{node_id}", {
      params: {
        path: {
          workspace_id: workspace_id,
          node_id: def.node_id
        },
        query: { label: def.label }
      }
    })
  }

  async deleteNode(workspace_id: number, nodeId: number) {
    await client.DELETE("/workspaces/{workspace_id}/node/{node_id}", {
      params: {
        path: {
          workspace_id: workspace_id,
          node_id: nodeId
        }
      }
    })
  }
}
export default NodesService;
