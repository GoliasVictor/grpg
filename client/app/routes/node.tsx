import { Button } from "~/components/ui/button"
import {
  QueryClient,
  QueryClientProvider,
  useQuery,
} from '@tanstack/react-query'
import createClient from "openapi-fetch";
import type { paths } from "~/lib/api/specs.d.ts";
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "~/components/ui/card"
import type { Route } from "./+types/node"
import { Fragment, useState } from "react";
import { Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow } from "~/components/ui/table";
import { NavLink } from "react-router";
const client = createClient<paths>({ baseUrl: "http://127.0.0.1:8000/" });

function usePredicate() {
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


export function meta({ }: Route.MetaArgs) {
  return [
    { title: "New React Router App" },
    { name: "description", content: "Welcome to React Router!" },
  ]
}
function GTable({ data, getPredicate, isIn }: { data: any[], getPredicate: (id: number) => any | undefined, isIn: boolean }) {
  
  const predicateCell = (d: any) => (
    <TableCell>
      {getPredicate(d.pid!)?.label ?? 0}
    </TableCell>
  )
  const labelCell = (d: any) => (
    <TableCell>
      <NavLink to={`/nodes/${d.node_id}`} >
        <Button variant="link" className="p-0" >
          {d.label}
        </Button>
      </NavLink>
    </TableCell>
  )
  let leftCell, rightCell;
  if (isIn) {
    leftCell = labelCell;
    rightCell = predicateCell;

  }
  else {
    leftCell = predicateCell;
    rightCell = labelCell;
  }
  return (<Table className="w-min">
    <TableHeader>
      <TableRow>
        <TableHead>Id</TableHead>
        <TableHead>Label</TableHead>
      </TableRow>
    </TableHeader>
    <TableBody>
      {data.map((d) => (
        <TableRow key={d.node_id.toString() + d.pid!.toString()}>
          {leftCell(d)}
          {rightCell(d)}

        </TableRow>
      ))}
    </TableBody>
  </Table>)
}
export default function Home(this: any, {
  params
}: Route.ComponentProps) {
  const { isPending, error, data } = useQuery({
    queryKey: ['repoData'],
    queryFn: async () => (await client.GET("/node"))?.data,
    //refetchInterval: 1500,
  })
  console.log("props", params.id);
  const [isIn, setIsIn] = useState(false);
  const nodeId = parseInt(params.id);
  console.log("nodeId", nodeId);
  const { predicates, getPredicate, ...predicatesQuery } = usePredicate();
  const inTableQuery = useQuery({
    queryKey: ['table', { isIn: "in", nodeId: nodeId }],
    queryFn: async () => (await client.POST("/table", {
      body: {
        node_id: nodeId,
        predicate: null,
        direction: "in",
      }
    }))?.data,
    //refetchInterval: 1500,
  })
  const outTableQuery = useQuery({
    queryKey: ['table', { isIn: "out", nodeId: nodeId }],
    queryFn: async () => (await client.POST("/table", {
      body: {
        node_id: nodeId,
        predicate: null,
        direction: "out",
      }
    }))?.data,
    //refetchInterval: 1500,
  })
  if (predicatesQuery.error) return 'An error has occurred: ' + predicatesQuery.error
  if (predicatesQuery.isLoading) return 'Loading...';
  if (!predicatesQuery.data) return 'No data found';

  if (inTableQuery.error) return 'An error has occurred: ' + inTableQuery.error
  if (inTableQuery.isLoading) return 'Loading...';
  if (!inTableQuery.data) return 'No data found';
  
  if (outTableQuery.error) return 'An error has occurred: ' + outTableQuery.error
  if (outTableQuery.isLoading) return 'Loading...';
  if (!outTableQuery.data) return 'No data found';

  if (isPending) return 'Loading...'
  if (!data) return 'No data found'
  if (error) return 'An error has occurred: ' + error.message
  const inTableData = inTableQuery.data;
  const outTableData = outTableQuery.data;
  return (
    <div className="flex h-screen w-screen items-center justify-center content-center">
      <div className="w-min flex-row flex gap-4" >
        <Card className="w-2xs">
          <CardHeader>
            <CardTitle>Out</CardTitle>
          </CardHeader>
          <CardContent>
            <GTable data={outTableData} isIn={false} getPredicate={getPredicate} />
          </CardContent>
        </Card>
        <Card className="w-2xs">
          <CardHeader>
            <CardTitle>In</CardTitle>
          </CardHeader>
          <CardContent>
            <GTable data={inTableData} isIn={true} getPredicate={getPredicate} />
          </CardContent>
        </Card>

      </div>
    </div>
  )
}