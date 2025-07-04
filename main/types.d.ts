type HashSet<T extends number | string> = Record<T, undefined>;
type HashMap<T extends number | string, U> = Record<T, U>;
type Vec<T> = Array<T>;
type Option<T> = T | undefined;
type Result<T, U> = T | U;
export type NumberAlias = number;
export type Colour =
  | { t: "Red"; c: number }
  | { t: "Green"; c: number }
  | { t: "Blue"; c: number };
export interface Person {
  name: string;
  age: number;
  enjoys_coffe: boolean;
}
export interface ComplexType {
  colour_map: HashMap<string, Colour>;
  list_of_names: Vec<string>;
  optional_person: Option<Person>;
}

