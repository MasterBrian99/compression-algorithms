import { assertEquals } from "@std/assert";
import Huffman from "./index.ts";

Deno.test(function addTest() {
  const input = "AAAAAABBCCDDEEFFFFF";
  const huffman = new Huffman();
  const codeMap = huffman.getCode(input);
  const encoded = huffman.encode(codeMap, input);

  console.log(codeMap);
  console.log(encoded);
  const decode = huffman.decode(encoded);
  console.log("decoding string: " + decode);
  assertEquals(input, decode);
});
