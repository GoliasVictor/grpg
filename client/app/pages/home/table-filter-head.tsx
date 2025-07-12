import { DropdownMenu } from "@radix-ui/react-dropdown-menu";
import { ArrowLeft, ArrowRight, Minus } from "lucide-react";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "~/components/ui/select"
import * as SelectPrimitive from "@radix-ui/react-select"
import React from "react";
import PredicatesComboBox from "../../components/predicates-combo-box";
import NodesComboBox from "../../components/nodes-combo-box";

type Filter = {
  predicate: number | null;
  direction: "in" | "out" | "any";
  anotherNode: number | null;
}

type Props = {
  filter: Filter,
  onChangeFilter: (filter: Filter) => void
}
export default function TableFilterHead({ filter, onChangeFilter }: Props) {
  const handleChangeDirection = (direction: "in" | "out" | "any") => {
    onChangeFilter({ ...filter, direction });
  };
  const handleChangePredicate = (predicate: number | null) => {
    onChangeFilter({ ...filter, predicate });
  };
  const handleChangeNode = (anotherNode: number | null) => {
    onChangeFilter({ ...filter, anotherNode });
  };
  return <>
    <div className="flex items-center gap-3">
      <span className="border-input flex items-center justify-between gap-2 rounded-md border bg-transparent px-3 py-2 text-sm whitespace-nowrap shadow-xs data-[size=default]:h-9 data-[size=sm]:h-8">
        Self
      </span>
      <PredicatesComboBox value={filter.predicate} onChange={handleChangePredicate}/>
      <Select
        value={filter.direction}
        onValueChange={handleChangeDirection}>
        <SelectTrigger noIcon={true}>
          <SelectValue placeholder="Select a fruit" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="out">
            <ArrowRight />
          </SelectItem>
          <SelectItem value="in">
            <ArrowLeft />
          </SelectItem>
          <SelectItem value="any">
            <Minus />
          </SelectItem>
        </SelectContent>
      </Select>
      <NodesComboBox value={filter.anotherNode} onChange={handleChangeNode}/>
    </div>
  </>

}
