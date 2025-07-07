import { useState } from "react";
import { Input } from "~/components/ui/input";
import { client, useNodesUpdateMutation } from "~/hooks/queries";

export default function NodeBadge({
  nodeId,
  getNode,
}: {
  nodeId: number;
  getNode: (id: number) => { label: string | null } | undefined;
}) {
  const [editMode, setEditMode] = useState(false);
  const [label, setLabel] = useState(getNode(nodeId)?.label ?? "");
  const mutation = useNodesUpdateMutation();
  async function save(new_label: string) {
    console.log("Saving label:", new_label);  
    await mutation.mutateAsync({
      nodeId, label: new_label
    });
  }
  const handleDoubleClick = () => {
    setEditMode(true);
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setLabel(e.target.value);
  };
  
  const handleBlur = () => {
    save(label)
    setEditMode(false);
    // Optionally, you can call a prop to update the label in parent here
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      save(label)
      setEditMode(false);
      // Optionally, you can call a prop to update the label in parent here
    }
  };

  return (
    editMode ? (
        <Input
          type="text"
          value={label}
          onChange={handleChange}
          onBlur={handleBlur}
          onKeyDown={handleKeyDown}
          autoFocus
          className="border w-min rounded px-1 text-xs p-1 py-1 h-min"
        />
      ) : (<span
      className="text-center align-middle border p-1 rounded-md text-xs"
      onClick={handleDoubleClick}
      style={{ cursor: "pointer" }}
    >
        {getNode(nodeId)?.label}
      
    </span >)
  );
}