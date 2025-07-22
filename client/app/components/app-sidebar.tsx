import { Calendar, Home, Inbox, MoreHorizontal, Plus, Search, Settings } from "lucide-react"

import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupAction,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuAction,
  SidebarMenuButton,
  SidebarMenuItem,
} from "~/components/ui/sidebar"
import { client, workspace_id, useTableCreateMutation, useTableDeleteMutation, useTablesQuery, useTableUpdateMutation } from "~/hooks/queries";
import { Link, useLocation } from "react-router";
import { DropdownMenu } from "@radix-ui/react-dropdown-menu";
import { DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "./ui/dropdown-menu";
import { memo, useRef, useState } from "react";
import { Input } from "./ui/input";
import DeleteTableDialog  from "./delete-table-dialog";
const SideBarItem = memo(function SideBarItem({ label, id, isActive }: {
  label: string;
  id: number;
  isActive: boolean;
}) {
  const url = `/table/${id}`;
  const [editMode, setEditMode] = useState(false);
  const [value, setValue] = useState(label);
  const tableUpdateMutation = useTableUpdateMutation();

  const close = async () => {
    tableUpdateMutation.mutate({
      tableId: id,
      def: {
        ...(await client.GET("/workspaces/{workspace_id}/table/{id}", {
          params: {
            path: {
              workspace_id: workspace_id,
              id: id
            }
          }
        }
        ))?.data!.def,
        label: value
      }
    });
    setEditMode(false);
  }

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      close();
    }
  };
  const inputRef = useRef<HTMLInputElement>(null);
  return (
    <SidebarMenuItem>
      <SidebarMenuButton asChild isActive={isActive}>
        {editMode ?
          <Input
            value={value}
            onChange={(e) => setValue(e.target.value)}
            onBlur={close}
            onKeyDown={handleKeyDown}
          />
          :
          <Link to={url}>
            <Inbox />
            <span>{label}</span>
          </Link>
        }

      </SidebarMenuButton>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <SidebarMenuAction>
            <MoreHorizontal />
          </SidebarMenuAction>
        </DropdownMenuTrigger>
        <DropdownMenuContent side="right" align="start">
          <DropdownMenuItem onClick={() => {
            setEditMode(true)
            inputRef.current?.focus();
          }}>
            <span>Renomear</span>
          </DropdownMenuItem>
          <DeleteTableDialog
            tableId={id}
            >
            <DropdownMenuItem onSelect={(e) => e.preventDefault()}>
              <span>Apagar</span>
            </DropdownMenuItem>
          </DeleteTableDialog>


        </DropdownMenuContent>
      </DropdownMenu>
    </SidebarMenuItem>
  );
});
export function AppSidebar() {
  const createTableMutation = useTableCreateMutation();
  const tablesQuery = useTablesQuery();
  const items = tablesQuery.data?.sort(d=> d.id).map((table) => ({
    label: table.def.label,
    id: table.id,
    isActive: location.pathname  == `/nodes/${table.id}`
  })) || [];
  const handleCreateTable = () => {
    createTableMutation.mutate({
      def: {
        label: "New Table",
        columns: [],
        filter: {
          predicate: null,
          direction: null,
          node_id: null,
        },
      },
    });
  };
  return (
    <Sidebar>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel>Tabelas</SidebarGroupLabel>
          <SidebarGroupAction title="Adicionar Tabela" onClick={handleCreateTable}>
            <Plus /><span className="sr-only">Adicionar Tabela</span>
          </SidebarGroupAction>
          <SidebarGroupContent>
            <SidebarMenu>
              {items.map((item) => (
                <SideBarItem
                  id={item.id}
                  key={item.id}
                  label={item.label}
                  isActive={item.isActive}
                />
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
    </Sidebar>
  )
}
