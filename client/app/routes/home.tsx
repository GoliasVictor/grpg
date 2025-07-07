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
  const [nodeId, setNodeId] = useState(1);
  const { getPredicate } = usePredicateQuery();
  const {nodes} = useNodesQuery();
  const [collumns, setCollumns] = useState<{
    id: number,
    filter: {
      direction: "in" | "out" | null;
      predicate_id: number
    }
  }[]> ([
    {
      id: 1,
      filter: {
        predicate_id: 1,
        direction:  "out",
      }
    },
    {
      id: 2,
      filter: {
        predicate_id: 2,
        direction: "in",
      }
    }
  ]);
  const values = nodes.map((n) => n.node_id);
  const tableQuery = useQuery({
    queryKey: ['home-table', {  values: values , columns: collumns}],
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
  return (
    <div className="flex flex-col h-screen w-screen items-center justify-center">
      <div className="w-min flex-row flex" >
        <div>
          <NodesTable data={tableData} columnsDef={collumns} onChangeColumn={(id, newPid, newInOut) => {
            setCollumns(
              collumns.map((c) => {
                if (c.id === id) {
                console.log(c.filter.direction)

                  return {
                    ...c,
                    filter: {
                      ...c.filter,
                      predicate_id: newPid || c.filter.predicate_id,
                      direction: (newInOut == null? c.filter.direction : (newInOut == "any" ? null : newInOut) ) 
                    }
                  }
                }
                return c;
              })   
            )
          } }/>
        </div>
      </div>

    </div>
  )
}