import { type ModType } from "../bindings";

export const getLabelByTipo = (tipo: ModType) => {
  return tipo.split(/(?=[A-Z])/)[0]
}