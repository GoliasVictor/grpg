import { Button } from "~/components/ui/button"
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "~/components/ui/card"
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "~/components/ui/table";
import { NavLink } from "react-router";
import { useNodesQuery } from "~/hooks/queries";

function NTable({ data}: { data: any[]}) {
  
  return (<Table className="w-min">
    <TableHeader>
      <TableRow>
        <TableHead>Id</TableHead>
        <TableHead>Label</TableHead>
      </TableRow>
    </TableHeader>
    <TableBody>
      {data.map((d) => (
        <TableRow key={d.node_id.toString()}>
          <TableCell>
            {d.node_id}
          </TableCell>
          <TableCell>
            <NavLink to={`/nodes/${d.node_id}`} >
              <Button variant="link" className="p-0" >
                {d.label}
              </Button>
            </NavLink>
          </TableCell>

        </TableRow>
      ))}
    </TableBody>
  </Table>)
}
export default function Home() {
  const nodesQuery = useNodesQuery();

  if (nodesQuery.isPending) return 'Loading...'
  if (!nodesQuery.data) return 'No data found'
  if (nodesQuery.error) return 'An error has occurred: ' + nodesQuery.error.message

  const inTableData = nodesQuery.data;
  return (
    <div className="flex h-screen w-screen items-center justify-center content-center">
      <div className="w-min flex-row flex gap-4" >
        <Card className="w-2xs">
          <CardHeader>
            <CardTitle>All</CardTitle>
          </CardHeader>
          <CardContent>
            <NTable data={inTableData}/>
          </CardContent>
        </Card>

      </div>
    </div>
  )
}