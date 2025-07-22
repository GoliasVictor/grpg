import { ComboBox } from "~/components/combo-box";
import { useNodesQuery } from "~/hooks/queries/nodes-queries";
type Props = {
  value: number | null,
  onChange: (value: number | null) => void,
  id?: string
}

export default function NodesComboBox( { value, id, onChange}: Props ) {
  const { data } = useNodesQuery();
  function handleChange(idStr: string) {
    if (idStr.trim() == "")
      return onChange(null);
    onChange(Number(idStr));
  }

  const ids = [...data?.map(e => e.node_id.toString()) || [], ""];

  return (<ComboBox
    value={value?.toString() ?? ""}
    valueToView={(v) => {
      if (v.trim() == "")
        return "Qualquer conceito";
      const id = Number(v);
      const element = data?.find(p => p.node_id === id);
      return element != undefined  ? element.label : "Erro!";
    }}
    onChange={handleChange}
    values={ids}
    disabled={ids.length == 0}
    placeholder="Qualquer conceito"
    id={id}
    className="w-min"
  />)
}
