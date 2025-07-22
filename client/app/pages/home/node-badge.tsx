import { memo, useState } from "react";
import { Input } from "~/components/ui/input";
import { client} from "~/hooks/queries";
import { useNodesQuery, useNodesUpdateMutation } from "~/hooks/queries/nodes-queries";
const NodeBadge = memo(function ({
  nodeId,
}: {
  nodeId: number;
}) {
  const [editMode, setEditMode] = useState(false);
  const { getNode } = useNodesQuery();
  const [label, setLabel] = useState(getNode(nodeId)?.label ?? "");
  const mutation = useNodesUpdateMutation();
  async function save(new_label: string) {
    await mutation.mutateAsync({
      nodeId, label: new_label
    });
  }
  const handleDoubleClick = () => {
    setEditMode(true);
    setLabel(label);
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setLabel(e.target.value);
    save(e.target.value)

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
      className="min-w-10 inline-block text-center align-middle border p-1 rounded-md text-xs"
      onClick={handleDoubleClick}
      style={{ cursor: "pointer" }}
    >
      {getNode(nodeId)?.label?.trim() || <span className="text-gray-500">Vazio</span>}

    </span >)
  );
});

export default NodeBadge;
