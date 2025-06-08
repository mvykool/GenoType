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

