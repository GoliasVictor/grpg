import { Dialog, DialogTrigger, DialogContent, DialogTitle, DialogClose } from "~/components/ui/dialog";
import { Label } from "~/components/ui/label";
import { Plus } from "lucide-react";
import { Button } from "~/components/ui/button";
import { DialogHeader, DialogFooter } from "~/components/ui/dialog";
import { Input } from "~/components/ui/input";
import PredicatesComboBox from "./predicates-combo-box";
import { useState } from "react";

import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "~/components/ui/select"


export function NewColumnDialog({
  onAddColumn
}: {
  onAddColumn: (predicate: number | null, direction: "in"| "out" | "any") => void;
}) {
  const [predicate, setPredicate] = useState<number | null>(null);
  const [direction, setDirection] = useState<string | "">("");
  return (<>
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="ghost" size="icon" className="text-right"><Plus /></Button >
      </DialogTrigger>
      <DialogContent className="w-sm sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Adicionar Coluna</DialogTitle>
        </DialogHeader>
        <div className="grid gap-4">
          <div className="grid gap-3">
            <Label htmlFor="predicate">Name</Label>
            <PredicatesComboBox value={predicate} onChange={setPredicate} />
          </div>
          <div className="grid gap-3">
            <Label htmlFor="username-1">Username</Label>
            <Select onValueChange={setDirection} defaultValue={direction}>
              <SelectTrigger>
                <SelectValue placeholder="Selecione a direcao" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="in">in</SelectItem>
                <SelectItem value="out">out</SelectItem>
                <SelectItem value="any">any</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </div>
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">Cencelar</Button>
          </DialogClose>
          <Button onClick={() => onAddColumn(predicate, (direction as "in" | "out" | "any" | "")|| "any")}>Adicinar</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </>
  )
}