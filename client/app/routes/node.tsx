import { Button } from "~/components/ui/button"
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "~/components/ui/card"
import type { Route } from "./+types/node"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "~/components/ui/table";
import { NavLink } from "react-router";
import { usePredicateQuery, useTableInOutQuery } from "~/hooks/queries";



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
  const { getPredicate } = usePredicateQuery();
  const nodeId = parseInt(params.id);
  const inTableQuery = useTableInOutQuery(true, nodeId);
  const outTableQuery = useTableInOutQuery(false, nodeId);

  if (inTableQuery.error) return 'An error has occurred: ' + inTableQuery.error
  if (inTableQuery.isLoading) return 'Loading...';
  if (!inTableQuery.data) return 'No data found';
  
  if (outTableQuery.error) return 'An error has occurred: ' + outTableQuery.error
  if (outTableQuery.isLoading) return 'Loading...';
  if (!outTableQuery.data) return 'No data found';


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