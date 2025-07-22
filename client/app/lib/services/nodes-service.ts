import { keepPreviousData, useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import createClient from "openapi-fetch";
import type { components, paths } from "~/lib/api/specs";

export const client = createClient<paths>({ baseUrl: import.meta.env.VITE_API_URL });

class NodesService {
  async getNodes(setting_id : number) {
    return (await client.GET("/settings/{setting_id}/node",
      {params:{path: {setting_id}}}
    )).data
  }
  async createNode(setting_id: number, data: components["schemas"]["NewNode"]) {
    await client.POST("/settings/{setting_id}/node", {
      params: {
        path: {
          setting_id
        }
      },
      body: data
    })
  }
  async updateNode(setting_id: number, def: components["schemas"]["Node"]) {
    await client.PUT("/settings/{setting_id}/node/{node_id}", {
      params: {
        path: {
          setting_id: setting_id,
          node_id: def.node_id
        },
        query: { label: def.label }
      }
    })
  }

  async deleteNode(setting_id: number, nodeId: number) {
    await client.DELETE("/settings/{setting_id}/node/{node_id}", {
      params: {
        path: {
          setting_id: setting_id,
          node_id: nodeId
        }
      }
    })
  }
}
export default NodesService;
