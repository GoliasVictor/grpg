"use client"

import * as React from "react"
import type {
  Cell,
  CellContext,
  ColumnDef,
  ColumnFiltersState,
  HeaderContext,
  Row,
  SortingState,
  VisibilityState
} from "@tanstack/react-table"
import {
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  useReactTable,
} from "@tanstack/react-table"
import { ArrowUpDown, ChevronDown, MoreHorizontal, Plus } from "lucide-react"

import { Button } from "~/components/ui/button"
import { Checkbox } from "~/components/ui/checkbox"
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "~/components/ui/dropdown-menu"
import { Input } from "~/components/ui/input"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "~/components/ui/table"
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "~/components/ui/dialog"
import { Label } from "~/components/ui/label"

import { useNodesCreateMutation, useNodesDeleteMutation, useNodesQuery, usePredicateQuery, useTripleCreateMutation, useTripleDeleteMutation } from "~/hooks/queries"
import DataTableColumnHeader  from "./data-table-column-header"
import NodeBadge from "./node-badge"
import NodeBadgeAdd from "./node-badge-add"
import NodeBadgeTriple from "./node-badge-triple"
import PredicatesComboBox from "../../components/predicates-combo-box"
import { NewColumnDialog } from "./new-column-dialog"
import TableFilterHead from "./table-filter-head"
import TablesComboBox from "~/components/tables-combo-box"

// Extend TableMeta to include nodeDeleteMutation
declare module '@tanstack/react-table' {
  interface TableMeta<TData extends unknown> {
    nodeDeleteMutation?: ReturnType<typeof useNodesDeleteMutation>;
    getPredicate: ReturnType<typeof usePredicateQuery>["getPredicate"];
  }
}


export type Payment = {
  node_id: number;
  columns: {
    id: number;
    values: number[];
  }[];
}

type Column = {
  id: number;
  filter: {
    direction: "in" | "out" | null;
    predicate_id: number | null;
  };
};
export type NodesTableProps = {
  data: Payment[];
  columnsDef: Column[];
  onChangeColumn: (
    id: number,
    newPid: number | null,
    newInOut: "in" | "out" | "any" | null
  ) => void;

  onNewColumn: (
    newPid: number,
    newInOut: "in" | "out" | "any"
  ) => void;
  onDeleteColumn: (id: number) => void;
  filter: any;
  setFilter: (filter: any) => void;
};


function NodeTableCell({ cell }: { cell: Cell<Payment, unknown> }) {
  return <TableCell>
    {flexRender(
      cell.column.columnDef.cell,
      cell.getContext()
    )}
  </TableCell>;
}
function NodeTableRow({ row } : { row: Row<Payment>}) {
  return (
  <TableRow
      data-state={row.getIsSelected() && "selected"}
    >
      {row.getVisibleCells().map((cell) => (
        <NodeTableCell key={cell.id} cell={cell} />
      ))}
    </TableRow>
  )
}

const DynamicColumnCell = ({ cellData, deleteTripleMutation, tripleMutation, column, getNode, values }:  {
  tripleMutation: ReturnType<typeof useTripleCreateMutation>,
  deleteTripleMutation: ReturnType<typeof useTripleDeleteMutation>,
  values : number[],
  cellData: Payment,
  column: any,
  getNode: ReturnType<typeof useNodesQuery>["getNode"]
}) => {
  const handleChoice = React.useCallback(
    (anotherId: number) => {
      if (column.filter.direction == "in") {
        tripleMutation.mutate({
          subjectId: anotherId,
          predicateId: column.filter.predicate_id,
          objectId: cellData.node_id
        });
      }
      else if (column.filter.direction == "out") {
        tripleMutation.mutate({
          subjectId: cellData.node_id,
          predicateId: column.filter.predicate_id,
          objectId: anotherId,
        });

      }
    }, [tripleMutation.mutate]
  )
  return (
    <div className="max-w-100 min-w-40 flex flex-wrap gap-1 overflow-x-scroll">
      {values.map((a) => <NodeBadgeTriple
        key={a} nodeId={a}
        getNode={getNode}
        onDelete={
          () => {
            deleteTripleMutation.mutate({
              subjectId: column.filter.direction == "in" ? a : cellData.node_id,
              predicateId: column.filter.predicate_id,
              objectId: column.filter.direction == "in" ? cellData.node_id : a
            });
          }
        }
      />)}
      { column.filter.direction != null && <NodeBadgeAdd onChoice={handleChoice}></NodeBadgeAdd>}

    </div>
  )
};
const NodesTable = React.memo(function NodesTable({ data, columnsDef, onNewColumn, onChangeColumn , onDeleteColumn, filter, setFilter}: NodesTableProps ) {
  const {getNode } = useNodesQuery();
  const {getPredicate} = usePredicateQuery();
  const nodeDeleteMutation = useNodesDeleteMutation();
  const tripleCreateMutation = useTripleCreateMutation();
  const novoMutation = useNodesCreateMutation();
  const deleteTripleMutation = useTripleDeleteMutation();
  const [rows, setRows] = React.useState<{rowData: any, component: any}>();

  const [columnVisibility, setColumnVisibility] =
    React.useState<VisibilityState>({})
  const [rowSelection, setRowSelection] = React.useState({})
  const columns = React.useMemo<ColumnDef<Payment>[]>(() =>[
    {
      id: "select",
      header: ({ table }) => (
        <div className="flex items-center">
          <Checkbox
            checked={
              table.getIsAllPageRowsSelected() ||
              (table.getIsSomePageRowsSelected() && "indeterminate")
            }
            onCheckedChange={(value) => table.toggleAllPageRowsSelected(!!value)}
            aria-label="Select all"
          /></div>
      ),
      cell: ({ row }) => (
        <div className="flex items-center">
          <Checkbox
            checked={row.getIsSelected()}
            onCheckedChange={(value) => row.toggleSelected(!!value)}
            aria-label="Select row"
          /></div>
      ),
      enableSorting: false,
      enableHiding: false,
    },
    {
      accessorKey: "node_id",
      header: "Node",
      cell: ({ row }) => (<div className="flex gap-2">
        <NodeBadge key={row.original.node_id} nodeId={row.original.node_id} />
      </div>

      ),
    },
    ...columnsDef.map((c) => ({
      id: `d-columns.${c.id}`,
      header: ({ column, table }: HeaderContext<Payment, unknown>) => {
        const getPredicate = table.options.meta?.getPredicate!
        const handleDeleteColumn = React.useCallback(
            () => {
              onDeleteColumn(c.id)
          },
          [c, onDeleteColumn]
        )
        const handleChangeDirection = React.useCallback(
          (direction: "in" | "out" | "any") => {
            onChangeColumn(c.id, null, direction)
          },
          [c, onChangeColumn]
        )
        return (
          <DataTableColumnHeader
            column={column}
            title={c.filter.predicate_id != null ? getPredicate(c.filter.predicate_id)?.label || "" :  "undefined"}
            isIn={c.filter.direction == null ? null : c.filter.direction == "in"}
            onDeleteColumn={handleDeleteColumn}
            onChangeDirection={handleChangeDirection}
          />)
      },
      enableHiding: false,
      cell: ({ row }: { row: Row<Payment> }) => {

        const values = row.original.columns.filter(d => d.id == c.id)[0].values as number[]
        return <DynamicColumnCell
          values={values}
          cellData={row.original}
          tripleMutation={tripleCreateMutation}
          deleteTripleMutation={deleteTripleMutation}
          column={c}
          getNode={getNode} />
      },
    })),
    {
      id: "actions",
      enableHiding: false,
      header: () => {
        const handleAddColumn = React.useCallback((predicate : number | null, direction : "in" | "out" | "any") => {
          onNewColumn(predicate || 0, direction);
        }, [onNewColumn]);

        return (<NewColumnDialog onAddColumn={handleAddColumn} />)
      },
      cell: ({ row, table }) => {
        const payment = row.original
        const nodeDeleteMutation = table.options.meta?.nodeDeleteMutation!;

        return (
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="ghost" className="h-8 w-8 p-0">
                <span className="sr-only">Open menu</span>
                <MoreHorizontal />
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuLabel>Actions</DropdownMenuLabel>
              <DropdownMenuItem
                onClick={() => {
                  nodeDeleteMutation.mutate({ nodeId: payment.node_id })
                }}
              >
                Apagar
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        )
      },
    },
  ], [columnsDef]);
  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    onRowSelectionChange: setRowSelection,
    state: {
      columnVisibility,
      rowSelection,
    },
    meta: {
      nodeDeleteMutation,
      getPredicate
    }
  })
  return (
    <div className="w-full">
      <div className="flex items-center py-4 gap-20 justify-between">
        <TableFilterHead filter={filter} onChangeFilter={setFilter} />
      </div>
      <div className="rounded-md border">
        <Table>
          <TableHeader>
            {table.getHeaderGroups().map((headerGroup) => (
              <TableRow key={headerGroup.id}>
                {headerGroup.headers.map((header) => {
                  return (
                    <TableHead key={header.id}>
                      {header.isPlaceholder
                        ? null
                        : flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                    </TableHead>
                  )
                })}
              </TableRow>
            ))}
          </TableHeader>
          <TableBody>
            {table.getRowModel().rows?.length ? (
              table.getRowModel().rows.map((row) => <NodeTableRow row={row} key={row.id}/>)
            ) : (
              <TableRow>
                <TableCell
                  colSpan={columns.length}
                  className="h-24 text-center"
                >
                  No results.
                </TableCell>
              </TableRow>
            )}
            <TableRow>
                <TableCell colSpan={columns.length} className="text-center">
                  <Button className="w-full h-full text-left left" onClick={() => novoMutation.mutate({label: " "})} variant="ghost">
                  <Plus/>
                    Novo no
                  </Button>
                </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </div>
      <div className="flex items-center justify-end space-x-2 py-4">
        <div className="text-muted-foreground flex-1 text-sm">
          {table.getFilteredSelectedRowModel().rows.length} of{" "}
          {table.getFilteredRowModel().rows.length} row(s) selected.
        </div>
        <div className="space-x-2">
          <Button
            variant="outline"
            size="sm"
            onClick={() => table.previousPage()}
            disabled={!table.getCanPreviousPage()}
          >
            Previous
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={() => table.nextPage()}
            disabled={!table.getCanNextPage()}
          >
            Next
          </Button>
        </div>
      </div>
    </div>
  )
})

export { NodesTable };
