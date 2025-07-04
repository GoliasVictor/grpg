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
const client = createClient<paths>({ baseUrl: "http://127.0.0.1:8000/" });


export function meta({}: Route.MetaArgs) {
  return [
    { title: "New React Router App" },
    { name: "description", content: "Welcome to React Router!" },
  ]
}

export default function Home(this: any) {
  const { isPending, error, data } = useQuery({
    queryKey: ['repoData'],
    queryFn: async () => (await client.GET("/node"))?.data
  })
  
  if (isPending) return 'Loading...'
  if (!data) return 'No data found'
  if (error) return 'An error has occurred: ' + error.message
  
  return (
    <div className="flex h-screen w-screen items-center justify-center">
      <div className="h-min w-sm grid grid-cols-2 gap-4 p-4">
        {data.map((d) => <Fragment key={d.node_id}>
          <span> #{(d.node_id * 10).toString(16).toUpperCase()} </span>
          <span>{d.label}</span>
        </Fragment>)}
        <Button>Click me { }</Button>
      </div>
    </div>
  )
}