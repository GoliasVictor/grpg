"use client"

import * as React from "react"
import { CheckIcon, ChevronsUpDownIcon, Plus } from "lucide-react"

import { cn } from "~/lib/utils"
import { Button } from "~/components/ui/button"
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "~/components/ui/command"
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "~/components/ui/popover"
import { useNodesQuery } from "~/hooks/queries"
import { memo, useMemo } from "react"



const NodeBadgeAdd = memo(function NodeBadgeAdd({onChoice}:{onChoice : (nodeId : number) => void}) {
  const [open, setOpen] = React.useState(false)
  const { nodes, getNode } = useNodesQuery({
    subscribed: false,
  });

  const frameworks = useMemo(() => {
     return nodes.map((n) => ({
      value: n.node_id.toString(),
      label: n.label || "No label",
    }))
  }, [nodes])

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
      <Button
            variant="outline"
            className=" w-min h-min gap-0 has-[>svg]:px-1 m-0 inline-block  text-center align-middle border p-1 rounded-md text-xs"
            style={{ cursor: "pointer" }}
          >
            <Plus className="w-4 h-4"/>
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[200px] p-0">
        <Command filter={
          (v, search, _keywords) => {
            return getNode(parseInt(v))?.label.toLocaleLowerCase().includes(search.toLowerCase())  ? 1 : 0
          }
        }>
          <CommandInput placeholder="Search framework..." />
          <CommandList>
            <CommandEmpty>No framework found.</CommandEmpty>
            <CommandGroup>
              {frameworks.map((framework) => (
                <CommandItem
                  key={framework.value}
                  value={framework.value}
                  onSelect={(currentValue) => {
                    onChoice(parseInt(currentValue))
                    setOpen(false)
                  }}
                >
                  {framework.label}
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  )
})

export default NodeBadgeAdd
