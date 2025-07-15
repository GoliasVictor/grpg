import type { Route } from "./+types/home"
import { useCallback, useState } from "react";
import { client, useTableQuery, useTableUpdateMutation } from "~/hooks/queries";
import { NodesTable } from "../pages/home/nodes-table";
import TablesComboBox from "~/components/tables-combo-box";
import type { components } from "~/lib/api/specs";
import { useNavigate, useParams } from "react-router";


export function meta({ }: Route.MetaArgs) {
  return [
    { title: "New React Router App" },
    { name: "description", content: "Welcome to React Router!" },
  ]
}

export default function Home(this: any) {
  return (
    <div className="flex flex-col h-screen w-full items-center">
      bem vindo ao GRPG :D

    </div>
  )
}
