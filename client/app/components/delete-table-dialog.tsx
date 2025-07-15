import type { PropsWithChildren } from "react"
import { Button } from "~/components/ui/button"
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
import { useTableDeleteMutation } from "~/hooks/queries";

export default function DeleteTableDialog(props: PropsWithChildren<{
  tableId: number;
}>) {
  const deleteTableMutation = useTableDeleteMutation();

  const deleteTable = () => {
    deleteTableMutation.mutate({
      tableId: props.tableId
    });
  };
  return (
    <Dialog >
      <form>
        <DialogTrigger asChild>
          {props.children }

        </DialogTrigger>
        <DialogContent className="sm:max-w-[425px]">
          <DialogHeader>
            <DialogTitle>Confirmar apagar</DialogTitle>
            <DialogDescription>
              Voce tem certeza que deseja apagar essa tabela? Essa ação não pode ser desfeita.
            </DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <DialogClose asChild>
              <Button variant="outline">Cancelar</Button>
            </DialogClose>
            <Button type="submit" variant="destructive" onClick={deleteTable}>Apagar</Button>
          </DialogFooter>
        </DialogContent>
      </form>
    </Dialog>
  )
}
