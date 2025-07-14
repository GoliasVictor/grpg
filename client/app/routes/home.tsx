import type { Route } from "./+types/home"
import { useCallback, useState } from "react";
import { client, useTableQuery, useTableUpdateMutation } from "~/hooks/queries";
import { NodesTable } from "../pages/home/nodes-table";
import TablesComboBox from "~/components/tables-combo-box";
import type { components } from "~/lib/api/specs";


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
  console.log("clientLoader");
  return {
    predicates: (await client.GET("/predicates"))?.data || []
  }
}

export default function Home(this: any) {
  const [filter, setFilter] = useState<Filter>({
    direction: "any",
    anotherNode: null,
    predicate: null,
  });
  const [tableId, setTableId] = useState(0);

  const tableUpdateMutation = useTableUpdateMutation();

  const tableQuery = useTableQuery(tableId);
  const columns = tableQuery?.data?.def.columns!;

  const handleNewColumn = useCallback((newPid: number, newInOut: "in" | "out" | "any") => {
    setCollumns([
      ...columns,
      {
        id: columns.length + 1,
        filter: {
          predicate_id: newPid ?? 1,
          direction: (newInOut == "any" ? null : newInOut)
        }
      }
    ])
  }, [columns]);
  const handleChangeColumn = useCallback((id: number, newPid: number | null, newInOut: "in" | "out" | "any" | null) => {
    setCollumns(
      columns.map((c) => {
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
  }, [columns]);

  const handleDeleteColumn = useCallback((id: number) => {
      setCollumns(columns.filter(c => c.id !== id));
  }, [columns]);

  const setCollumns = useCallback((newColumns : components["schemas"]["ColumnDefinition"][]) => {
    tableUpdateMutation.mutate({
      tableId,
      def: {
        ...tableQuery.data?.def!,
        columns: newColumns
      }
    });
  },[tableUpdateMutation.mutate, tableQuery.data]);

  if (tableQuery.error) return 'An error has occurred: ' + tableQuery.error
  if (tableQuery.isLoading) return 'Loading...';
  return (
    <div className="flex flex-col h-screen w-screen items-center">
      <div className="w-min flex-row flex py-10" >
        <div>
          <div>
            <TablesComboBox onChange={( d) => setTableId(d!)} value={tableId} />
          </div>
          <NodesTable
            data={tableQuery.data?.rows || []}
            columnsDef={columns}
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
