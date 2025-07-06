import { Button } from "~/components/ui/button"
import {
  useQuery,
} from '@tanstack/react-query'
import { Maximize2, Minimize2 } from "lucide-react";
import type { Route } from "./+types/home"
import { useState } from "react";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "~/components/ui/table";
import { client, useNodesQuery, usePredicateQuery } from "~/hooks/queries";
import { NodesTable } from "./nodes-table";


export function meta({ }: Route.MetaArgs) {
  return [
    { title: "New React Router App" },
    { name: "description", content: "Welcome to React Router!" },
  ]
}

export default function Home(this: any) {
  const [isIn, setIsIn] = useState(false);
  const [nodeId, setNodeId] = useState(1);
  const { getPredicate } = usePredicateQuery();
  const {nodes} = useNodesQuery();
  const collumns : {
    id: number,
    filter: {
      direction: "in" | "out" | null;
      predicate_id: number
    }
  }[] = [
    {
      id: 1,
      filter: {
        predicate_id: 1,
        direction: isIn ? "in" : "out",
      }
    },
    {
      id: 2,
      filter: {
        predicate_id: 2,
        direction: isIn ? "in" : "out",
      }
    }
  ];
  const values = nodes.map((n) => n.node_id);
  const tableQuery = useQuery({
    queryKey: ['table', { isIn: isIn, values: values }],
    queryFn: async () => (
      await Promise.all(
        values.map(async (c) => ({
            node_id: c, row: (
              (await client.POST("/row", {
                body: {
                  node_id: c,
                  columns: collumns
                }
              }
              ))?.data
            )
          })
        )
      )
    )
    //refetchInterval: 1500,
  })


  if (tableQuery.error) return 'An error has occurred: ' + tableQuery.error
  if (tableQuery.isLoading) return 'Loading...';
  if (!tableQuery.data) return 'No data found';
  const tableData = tableQuery.data;
  console.log("tableData", tableData.map((r) => r.row?.columns));
  return (
    <div className="flex flex-col h-screen w-screen items-center justify-center">
      <div>
        <Button variant="secondary" size="icon" onClick={() => setIsIn(!isIn)}>
          {isIn ? <Minimize2 /> : <Maximize2 />}
        </Button>
        <span>
          {nodeId}
        </span>
      </div>
      <div className="w-min flex-row flex" >
        <div>
          <NodesTable data={tableData} columnsDef={collumns} />
        </div>
      </div>

    </div>
  )
}