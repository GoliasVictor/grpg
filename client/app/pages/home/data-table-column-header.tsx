import { type Column } from "@tanstack/react-table"
import { ArrowDown, ArrowLeft, ArrowRight, ArrowUp, ChevronsUpDown, EyeOff, Minus, X } from "lucide-react"

import { cn } from "~/lib/utils"
import { Button } from "~/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "~/components/ui/dropdown-menu"
import { memo } from "react"

interface DataTableColumnHeaderProps<TData, TValue>
  extends React.HTMLAttributes<HTMLDivElement> {
  column: Column<TData, TValue>
  title: string,
  isIn: boolean | null
  onChangeDirection: (direction: "in" | "out" | "any") => void
  onDeleteColumn: () => void
}

function DataTableColumnHeaderPrimitive<TData, TValue>({
  column,
  title,
  className,
  isIn,
  onChangeDirection,
  onDeleteColumn
}: DataTableColumnHeaderProps<TData, TValue>) {
  //   if (!column.getCanSort()) {
  //     return <div className={cn(className)}>{title}</div>
  //   }

  return (
    <div className={cn("flex items-center", className)}>
      <Button variant="ghost">{title}</Button>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <div>
            <Button
              variant="ghost"
              size="sm"
              className="data-[state=open]:bg-accent  h-8"
            >

              {isIn === null ? (
                <Minus />
              ) : isIn ? (
                <ArrowLeft />
              ) : (
                <ArrowRight />

              )}
            </Button>
          </div>

        </DropdownMenuTrigger>
        <DropdownMenuContent align="start">
          <DropdownMenuItem onClick={() => onChangeDirection("in")}>
            <ArrowLeft />
            In
          </DropdownMenuItem>
          <DropdownMenuItem onClick={() => onChangeDirection("out")}>
            <ArrowRight />
            Out
          </DropdownMenuItem>
          <DropdownMenuItem onClick={() => onChangeDirection("any")}>
            <Minus />
            Any
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
      <Button variant="ghost" size="icon" className="transition-colors duration-200 hover:text-white cursor-pointer
      hover:bg-destructive focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60 h-8" onClick={onDeleteColumn}>
        <X />
      </Button>
    </div>
  )
};

export default memo(DataTableColumnHeaderPrimitive) as typeof DataTableColumnHeaderPrimitive;
