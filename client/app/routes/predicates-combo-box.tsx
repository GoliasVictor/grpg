import { ComboBox } from "~/components/combo-box";
import { usePredicateQuery } from "~/hooks/queries";

export default function PredicatesComboBox(props: {
  value: number | null,
  onChange: (value: number | null) => void,
  id?: string
}) {
  const { data } = usePredicateQuery();
  function handleChange(idStr: string) {
    if (idStr.trim() == "")
      return props.onChange(null);
    props.onChange(Number(idStr));
  }

  const ids = data?.map(e => e.id.toString()) || [];

  return (<ComboBox
    value={props.value?.toString() ?? ""}
    valueToView={(v) => {
      const id = Number(v);
      const predicate = data?.find(p => p.id === id);
      return predicate != undefined  ? predicate.label : "Erro!";
    }}
    onChange={handleChange}
    values={ids}
    disabled={ids.length == 0}
    placeholder="Selecione um predicado"
    id={props.id}
  />)
} 