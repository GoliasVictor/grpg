import { Button } from "~/components/ui/button"
import {
  QueryClient,
  QueryClientProvider,
  useQuery,
} from '@tanstack/react-query'
import createClient from "openapi-fetch";
import type { paths } from "~/lib/api/specs.d.ts"; 

import type { Route } from "./+types/home"
import { Fragment, useState } from "react";
import type { D } from "node_modules/react-router/dist/development/route-data-D7Xbr_Ww.mjs";
import { Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow } from "~/components/ui/table";
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
export function meta({}: Route.MetaArgs) {
  return [
    { title: "New React Router App" },
    { name: "description", content: "Welcome to React Router!" },
  ]
}

export default function Home(this: any) {
  const { isPending, error, data } = useQuery({
    queryKey: ['repoData'],
    queryFn: async () => (await client.GET("/node"))?.data,
    //refetchInterval: 1500,
  })
  const [isIn, setIsIn] = useState(false);
  const [nodeId , setNodeId] = useState(1);
  const { predicates, getPredicate, ...predicatesQuery } = usePredicate();
  const tableQuery = useQuery({
      queryKey: ['table', {isIn : isIn, nodeId: nodeId}],
      queryFn: async () => (await client.POST("/table", {
        body: {
          node_id: nodeId,
          predicate: null,
          direction: isIn ? "in" : "out",
        }
      }))?.data,
      //refetchInterval: 1500,
  }) 
  if (predicatesQuery.error) return 'An error has occurred: ' + predicatesQuery.error
  if (predicatesQuery.isLoading) return 'Loading...';
  if (!predicatesQuery.data) return 'No data found';

  if (tableQuery.error) return 'An error has occurred: ' + tableQuery.error
  if (tableQuery.isLoading) return 'Loading...';
  if (!tableQuery.data) return 'No data found';

  if (isPending) return 'Loading...'
  if (!data) return 'No data found'
  if (error) return 'An error has occurred: ' + error.message
  const tableData = tableQuery.data;
  return (
    <div className="flex h-screen w-screen items-center justify-center content-center">
      <Button onClick={() => setIsIn(!isIn)}>
        {isIn ? "To In" : "To Out"}
      </Button>
      <span>
        {nodeId}
      </span>
      <div className="w-min flex-row flex" >
        <Table className="w-min">
          <TableHeader>
            <TableRow>
              <TableHead>Id</TableHead>
              <TableHead>Label</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {data.map((d) => (
              <TableRow key={d.node_id}>
                    <TableCell key={d.node_id} className="font-medium">
                      <Button variant="link" className="p-0" onClick={() => {
                        setNodeId(d.node_id);
                        setIsIn(false);
                  }}>    
                    #{d.node_id}
                  </Button>
                    </TableCell>
                    <TableCell>{d.label}</TableCell>
                </TableRow>
              ))}
          </TableBody>
        </Table>
        <Table className="w-min">
          <TableHeader>
            <TableRow>
              <TableHead>Id</TableHead>
              <TableHead>Label</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
              { tableData.map((d) => (
                <TableRow key={d.node_id.toString() + d.pid!.toString()}>
                    <TableCell key={d.node_id} className="font-medium">
                      #{d.node_id}
                  </TableCell>
                  <TableCell>
                    {getPredicate(d.pid!)?.label ?? 0}
                  </TableCell>
                    <TableCell>{d.label}</TableCell>
                </TableRow>
              ))}
          </TableBody>
        </Table>
      </div>
        
    </div>
  )
}