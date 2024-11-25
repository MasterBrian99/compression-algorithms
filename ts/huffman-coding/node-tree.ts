export default class Node {
  frequency: number;
  leftNode: Node | null = null;
  rightNode: Node | null = null;
  char: string;

  constructor(
    char: string,
    fre: number,
    leftNode: Node | null,
    rightNode: Node | null
  ) {
    this.leftNode = leftNode;
    this.rightNode = rightNode;
    this.frequency = fre;
    this.char = char;
  }
}
