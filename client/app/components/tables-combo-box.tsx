import { ComboBox } from "~/components/combo-box";
import { useTablesQuery } from "~/hooks/queries";

type Props = {
  value: number | null,
  onChange: (value: number | null) => void,
  id?: string
}

export default function TablesComboBox( { value, id, onChange}: Props ) {
  const { data } = useTablesQuery();
  function handleChange(idStr: string) {
    if (idStr.trim() == "")
      return onChange(null);
    onChange(Number(idStr));
  }

  const ids = data?.map(e => e.id.toString()) || [];

  return (<ComboBox
    value={value?.toString() ?? ""}
    valueToView={(v) => {
      const id = Number(v);
      const table = data?.find(p => p.id === id);
      return table != undefined  ? table.def.label : "Erro!";
    }}
    onChange={handleChange}
    values={ids}
    disabled={ids.length == 0}
    placeholder="Qualquer Tabela"
    id={id}
    className="w-min"
  />)
}
