import { Button } from "~/components/ui/button"
import {
  keepPreviousData,
  useQuery,
  useQueryClient,
} from '@tanstack/react-query'
import { Maximize2, Minimize2 } from "lucide-react";
import type { Route } from "./+types/home"
import { useCallback, useState } from "react";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "~/components/ui/table";
import { client, useNodesQuery, usePredicateQuery } from "~/hooks/queries";
import { NodesTable } from "../pages/home/nodes-table";


export function meta({ }: Route.MetaArgs) {
  return [
    { title: "New React Router App" },
    { name: "description", content: "Welcome to React Router!" },
  ]
}
type Filter = {
  predicate: number | null;
  direction: "in" | "out" | "any";
  anotherNode: number | null;
}

export async function clientLoader({ }: Route.LoaderArgs) {
  return {
    predicates: (await client.GET("/predicates"))?.data || []
  }
}

export default function Home(this: any, { loaderData }: Route.ComponentProps) {
  const { nodes } = useNodesQuery();
  const [filter, setFilter] = useState<Filter>({
    direction: "any",
    anotherNode: null,
    predicate: null,
  });
  const [collumns, setCollumns] = useState<{
    id: number,
    filter: {
      direction: "in" | "out" | null;
      predicate_id: number
    }
  }[]>(loaderData.predicates.map((p) =>
  ({
    id: p.id,
    filter: {
      predicate_id: p.id,
      direction: null,
    }
  })));
  const filterdNodes = useQuery({
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

  let values;
  if (filter.anotherNode == null && filter.direction == "any" && filter.predicate == null) {
    values = nodes.map(n => n.node_id);
  } else {
    values = [...filterdNodes.data || []];
  }
  const tableQuery = useQuery({
    queryKey: ['home-table', { values: values, columns: collumns }],
    queryFn: async () => (
      (await client.POST("/full-table", {
          body: {
            nodes_id: values,
            columns: collumns
          }
        }
      ))?.data
    ),
    placeholderData: keepPreviousData,
    //refetchInterval: 1500,
  })
  const [lastData, setLastData] = useState<typeof tableQuery.data | null>(null);
  const handleNewColumn = useCallback((newPid: number, newInOut: "in" | "out" | "any") => {
    setCollumns([
      ...collumns,
      {
        id: collumns.length + 1,
        filter: {
          predicate_id: newPid ?? 1,
          direction: (newInOut == "any" ? null : newInOut)
        }
      }
    ])
  }, [collumns]);
  const handleChangeColumn = useCallback((id: number, newPid: number | null, newInOut: "in" | "out" | "any" | null) => {
    setCollumns(
      collumns.map((c) => {
        if (c.id === id) {

          return {
            ...c,
            filter: {
              ...c.filter,
              predicate_id: newPid || c.filter.predicate_id,
              direction: (newInOut == null ? c.filter.direction : (newInOut == "any" ? null : newInOut))
            }
          }
        }
        return c;
      })
    )
  }, [collumns]);

  const handleDeleteColumn = useCallback((id: number) => {
      setCollumns(collumns.filter(c => c.id !== id));
  }, [collumns]);
  if (tableQuery.error) return 'An error has occurred: ' + tableQuery.error
  if (tableQuery.isLoading && lastData == null) return 'Loading...';
  const tableData = (tableQuery.data ?? lastData) ?? [];
  if (!Object.is(tableQuery.data, lastData) && !tableQuery.isPending  ) {
    setLastData(tableQuery.data);
  }
  console.log(tableQuery.data)
  return (
    <div className="flex flex-col h-screen w-screen items-center">
      <div className="w-min flex-row flex" >
        <div>
          <NodesTable
            data={tableQuery.data || []}
            columnsDef={collumns}
            filter={filter}
            setFilter={setFilter}
            onNewColumn={handleNewColumn}
            onChangeColumn={handleChangeColumn}
            onDeleteColumn={handleDeleteColumn}
          />
        </div>
      </div>

    </div>
  )
}
