import { ComboBox } from "~/components/combo-box";
import { usePredicateQuery } from "~/hooks/queries";

type Props = {
  value: number | null,
  onChange: (value: number | null) => void,
  id?: string
}

export default function PredicatesComboBox( { value, id, onChange}: Props ) {
  const { data } = usePredicateQuery();
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
      const predicate = data?.find(p => p.id === id);
      return predicate != undefined  ? predicate.label : "Erro!";
    }}
    onChange={handleChange}
    values={ids}
    disabled={ids.length == 0}
    placeholder="Qualquer relação"
    id={id}
    className="w-min"
  />)
}
