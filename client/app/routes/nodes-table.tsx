"use client"

import * as React from "react"
import type {
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
import { DataTableColumnHeader } from "./data-table-column-header"
import NodeBadge from "./node-badge"
import NodeBadgeAdd from "./node-badge-add"
import NodeBadgeTriple from "./node-badge-triple"
import PredicatesComboBox from "./predicates-combo-box"
import { NewColumnDialog } from "./new-column-dialog"

// Extend TableMeta to include nodeDeleteMutation
declare module '@tanstack/react-table' {
  interface TableMeta<TData extends unknown> {
    nodeDeleteMutation?: ReturnType<typeof useNodesDeleteMutation>;
  }
}


export type Payment = {
  node_id: number;
  row: {
    columns: {
      id: number;
      values: number[];
    }[];
  } | undefined;
}

type Column = {
  id: number;
  filter: {
    direction: "in" | "out" | null;
    predicate_id: number;
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
};

export function NodesTable({ data, columnsDef, onNewColumn, onChangeColumn , onDeleteColumn}: NodesTableProps ) {
  const {getNode } = useNodesQuery();
  const {getPredicate} = usePredicateQuery();
  const [sorting, setSorting] = React.useState<SortingState>([])
  const [columnFilters, setColumnFilters] = React.useState<ColumnFiltersState>(
    []
  )
  
  const nodeDeleteMutation = useNodesDeleteMutation();
  const tripleMutation = useTripleCreateMutation();
  const novoMutation = useNodesCreateMutation();
  const deleteTripleMutation = useTripleDeleteMutation();


  const [columnVisibility, setColumnVisibility] =
    React.useState<VisibilityState>({})
  const [rowSelection, setRowSelection] = React.useState({})
  const columns: ColumnDef<Payment>[] = [
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
      header: ({ column }: HeaderContext<Payment, unknown>) => (
        <DataTableColumnHeader
          column={column}
          title={getPredicate(c.filter.predicate_id)?.label || ""}
          isIn={c.filter.direction == null ? null : c.filter.direction == "in"}
          onDeleteColumn={() => {
            onDeleteColumn(c.id)
          }}
          onChangeDirection={(direction) => {
            onChangeColumn(c.id,null, direction)
          }}
        />
      ),
      enableHiding: false,
      cell: ({ row }: { row: Row<Payment> }) => {
        
        const values = row.original.row?.columns.filter(d => d.id == c.id)[0].values as number[]
        return (
          <div className="max-w-60 min-w-40 flex flex-wrap gap-1 overflow-x-scroll">
            {values.map((a) => <NodeBadgeTriple
              key={a} nodeId={a}
              getNode={getNode}
              onDelete={
                () => {
                  deleteTripleMutation.mutate({
                    subjectId: c.filter.direction == "in" ? a : row.original.node_id,
                    predicateId: c.filter.predicate_id,
                    objectId: c.filter.direction == "in" ? row.original.node_id : a
                  });
                }
              }
            />)}
            <NodeBadgeAdd onChoice={(anotherId) => {
              if (c.filter.direction == "in") {
                tripleMutation.mutate({
                  subjectId: anotherId,
                  predicateId: c.filter.predicate_id,
                  objectId: row.original.node_id
                });
              }
              else if (c.filter.direction == "out") {
                tripleMutation.mutate({
                  subjectId: row.original.node_id,
                  predicateId: c.filter.predicate_id,
                  objectId: anotherId,
                });

              }
            }}></NodeBadgeAdd>
          </div>
        )
      },
    })),
    {
      id: "actions",
      enableHiding: false,
      header: () => (<NewColumnDialog onAddColumn={(predicate, direction) => {
        onNewColumn(predicate || 0, direction);
      }}/>),
      cell: ({ row, table}) => {
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
  ]

  const table = useReactTable({
    data,
    columns,
    onSortingChange: setSorting,
    onColumnFiltersChange: setColumnFilters,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    onColumnVisibilityChange: setColumnVisibility,
    onRowSelectionChange: setRowSelection,
    state: {
      sorting,
      columnFilters,
      columnVisibility,
      rowSelection,
    },
    meta: {
      nodeDeleteMutation
    }
  })

  return (
    <div className="w-full">
      <div className="flex items-center py-4">
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="outline" className="ml-auto">
              Columns <ChevronDown />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            {table
              .getAllColumns()
              .filter((column) => column.getCanHide())
              .map((column) => {
                return (
                  <DropdownMenuCheckboxItem
                    key={column.id}
                    className="capitalize"
                    checked={column.getIsVisible()}
                    onCheckedChange={(value) =>
                      column.toggleVisibility(!!value)
                    }
                  >
                    {column.id}
                  </DropdownMenuCheckboxItem>
                )
              })}
          </DropdownMenuContent>
        </DropdownMenu>
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
              table.getRowModel().rows.map((row) => (
                <TableRow
                  key={row.id}
                  data-state={row.getIsSelected() && "selected"}
                >
                  {row.getVisibleCells().map((cell) => (
                    <TableCell key={cell.id}>
                      {flexRender(
                        cell.column.columnDef.cell,
                        cell.getContext()
                      )}
                    </TableCell>
                  ))}
                </TableRow>
              ))
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
}
