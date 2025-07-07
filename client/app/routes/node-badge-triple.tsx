import { useState } from "react";
import { Button } from "~/components/ui/button";
import { Input } from "~/components/ui/input";
import { client, useNodesUpdateMutation } from "~/hooks/queries";

export default function NodeBadgeTriple({
  nodeId,
  getNode,
  onDelete
}: {
  nodeId: number;
  getNode: (id: number) => { label: string | null } | undefined;
  onDelete: () => void;
}) {

  return (
    <Button
      variant="outline"
      className="min-w-10 inline-block text-center align-middle border p-1 rounded-md text-xs transition-colors duration-200 hover:text-white cursor-pointer h-min
      
      hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60
    
      "
      onClick={onDelete}
    >
      {getNode(nodeId)?.label?.trim() || <span className="text-gray-500">Vazio</span>}
    </Button>
  );
}