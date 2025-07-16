import * as React from "react"
import { CheckIcon, ChevronsUpDownIcon } from "lucide-react"

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

type ComboBoxProps = {
  id?: string,
  value: string,
  onChange: (value: string) => void,
  valueToView: (value: string) => string,
  values: string[],
  disabled: boolean,
  placeholder: string
} & Omit<React.ComponentProps<typeof Button>, "onChange">

export function ComboBox({
    id,
    value,
    onChange,
    valueToView,
    values,
    disabled,
    placeholder,
    className,
    ...props
  }: ComboBoxProps) {
  const [open, setOpen] = React.useState(false)

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild disabled={disabled} className="text-left">
        <Button
          disabled={disabled}
          variant="outline"
          role="combobox"
          type="button"
          aria-expanded={open}
          className={cn("w-full justify-between ", className)}
          id={id}

        >
          {value
            ? valueToView(value)
            : placeholder}
          <ChevronsUpDownIcon className="ml-2 h-4 w-4 shrink-0 opacity-50" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-3xs h-40 p-0">
        <Command filter={
          (v, search, _keywords) => {
            return valueToView(v).toLocaleLowerCase().includes(search.toLowerCase()) ? 1 : 0
          }
        }>
          <CommandInput placeholder={placeholder} />
          <CommandList>
            <CommandEmpty>No framework found.</CommandEmpty>
            <CommandGroup>
              {values.map((v) => (
                <CommandItem
                  key={v}
                  value={v}

                  onSelect={(currentValue) => {
                    onChange(currentValue === value ? "" : currentValue)
                    setOpen(false)
                  }}
                >
                  {valueToView(v)}

                  <CheckIcon
                    className={cn(
                      "mr-2 h-4 w-4",
                      value === v ? "opacity-100" : "opacity-0"
                    )}
                  />
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  )
}
