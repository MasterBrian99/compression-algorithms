import Node from "./node-tree.ts";

export default class Huffman {
  root: Node | null = null;
  getCode(input: string): Map<string, string> {
    const freqMap = this.buildFrequencyMap(input);
    const nodeQueue = this.sortByFrequence(freqMap);
    this.root = this.buildTree(nodeQueue);
    const codeMap = this.createHuffmanCode(this.root);
    return codeMap;
  }

  buildFrequencyMap(input: string): Map<string, number> {
    const map = new Map<string, number>();
    for (let i = 0; i < input.length; i++) {
      const ch = input.charAt(i);
      if (!map.has(ch)) {
        map.set(ch, 1);
      } else {
        const val = map.get(ch)!;
        map.set(ch, val + 1);
      }
    }
    return map;
  }
  sortByFrequence(map: Map<string, number>): Node[] {
    const queue: Node[] = [];
    for (const [ch, frequency] of map.entries()) {
      queue.push(new Node(ch, frequency, null, null));
    }
    queue.sort((a, b) => a.frequency - b.frequency);
    return queue;
  }

  buildTree(nodeQueue: Node[]): Node | null {
    while (nodeQueue.length > 1) {
      const node1 = nodeQueue.shift()!;
      const node2 = nodeQueue.shift()!;
      const parentNode = new Node(
        "",
        node1.frequency + node2.frequency,
        node1,
        node2
      );
      nodeQueue.push(parentNode);
      nodeQueue.sort((a, b) => a.frequency - b.frequency);
    }
    return nodeQueue.shift() || null;
  }
  createHuffmanCode(node: Node | null): Map<string, string> {
    const map = new Map<string, string>();
    this.createCodeRec(node, map, "");
    return map;
  }

  // Preorder of the tree using recursion
  createCodeRec(node: Node | null, map: Map<string, string>, s: string): void {
    if (!node) return;
    if (node.leftNode === null && node.rightNode === null) {
      map.set(node.char, s);
      return;
    }
    this.createCodeRec(node.leftNode, map, s + "0");
    this.createCodeRec(node.rightNode, map, s + "1");
  }

  // Step 5: Use Huffman code to encode the input string
  encode(codeMap: Map<string, string>, input: string): string {
    let encodedString = "";
    for (let i = 0; i < input.length; i++) {
      encodedString += codeMap.get(input.charAt(i))!;
    }
    return encodedString;
  }

  // Step 6: Decode the encoded string
  decode(coded: string): string {
    let decodedString = "";
    let curr = this.root;
    for (let i = 0; i < coded.length; i++) {
      curr = coded.charAt(i) === "1" ? curr?.rightNode! : curr?.leftNode!;
      if (curr?.leftNode === null && curr?.rightNode === null) {
        decodedString += curr.char;
        curr = this.root;
      }
    }
    return decodedString;
  }
}
