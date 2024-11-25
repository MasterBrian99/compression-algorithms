use std::{
    char,
    collections::{BinaryHeap, HashMap}
};

#[derive(Debug, Eq)]
pub struct Node {
    symbol: Option<char>,
    weight: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
        // TODO
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Huffman {
    root: Option<Box<Node>>,
}
impl Huffman {
    pub fn new() -> Self {
        Self { root: None }
    }
    pub fn generate_code(&mut self, input: &str)->HashMap<char, String> {
        let freq_map: HashMap<char, usize> = Self::count_freq(input);
        let mut queue: BinaryHeap<Node> = Self::create_queue(&freq_map);
        self.root=Self::create_tree_item(&mut queue);
        let mut mapped=HashMap::new();
        if let Some(ro)= &self.root{
            // do things.idk
            Self::build_map_code(ro,&mut mapped, String::new());
        }
        mapped
    }
    pub fn count_freq(input: &str) -> HashMap<char, usize> {
        let mut freq_map: HashMap<char, usize> = HashMap::new();

        for ch in input.chars() {
            *freq_map.entry(ch).or_insert(0) += 1;
        }
        freq_map
    }
    pub fn create_queue(map: &HashMap<char, usize>) -> BinaryHeap<Node> {
        let mut queue: BinaryHeap<Node> = BinaryHeap::new();
        for (&ch, &fre) in map {
            queue.push(Node {
                left: None,
                right: None,
                symbol: Some(ch),
                weight: fre,
            });
        }
        queue
    }

    pub fn create_tree_item(queue:&mut BinaryHeap<Node> )->Option<Box<Node>> {
        while queue.len()>1 { //fuck meeeeeeeeeeeeeeeeeeeeeeeeeeeeeee
            //take first to
            // 0,1,1,1,2
            let smallest1=queue.pop().unwrap();
            let smallest2=queue.pop().unwrap();

            let parent=Node{
                weight:smallest1.weight+smallest2.weight,
                left:Some(Box::new(smallest1)),
                right:Some(Box::new(smallest2)),
                symbol:None,
            };
            queue.push(parent);
        }

        queue.pop().map(Box::new)

    }
    pub fn build_map_code(node: &Node,map:&mut HashMap<char,String>,path:String){
        if let Some(symbol)=node.symbol {
            map.insert(symbol,path);
        }else{
            if let Some(left)=&node.left{
                Self::build_map_code(left, map,format!("{}0",path));
            }
            if let Some(right)=&node.right {
                Self::build_map_code(right, map, format!("{}1",path));
            }
        }

    }
    pub fn compression(&self,mapped:&HashMap<char,String>,text:&str)->String{

        text.chars().map(|char|mapped.get(&char).unwrap().clone()).collect()

    }
    pub fn decompress(&self,char_bits:&str)->String{
        let mut decoded_text=String::new();
        let mut current=self.root.as_deref();

        for bit in char_bits.chars(){
            match current {
                Some(node) => {
                    current=if bit=='0'{
                        node.left.as_deref()
                    }else{
                        node.right.as_deref()
                    };
                    if let Some(node)=current {
                        if let Some(symbol)=node.symbol {
                            decoded_text.push(symbol);
                            current=self.root.as_deref()
                        }
                    }
                },
                None =>break,
            }
        };
        decoded_text

    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_start() {
        println!("hello");
    }

    #[test]
    fn test_count_freq() {
        let input = "AABBCCD";
        let expected = HashMap::from([('A', 2), ('B', 2), ('C', 2), ('D', 1)]);
        let res = Huffman::count_freq(input);
        assert_eq!(expected, res);
    }
    #[test]
    fn test_create_queue() {
        let map = HashMap::from([('A', 2), ('B', 2), ('C', 2), ('D', 1)]);
        let queue = Huffman::create_queue(&map);
        println!("{:?}", queue);
    }
    #[test]
    fn test_compress(){
        let input="AAABB";
        let mut compresser=Huffman::new();
        let codes=compresser.generate_code(input);
        let compressed=compresser.compression(&codes, input);
        assert_eq!(compressed.len(),5);
    }

    #[test]
    fn test_decompress(){
        let input="AAABB";
        let mut compressor=Huffman::new();
        let codes=compressor.generate_code(input);
        let compressed=compressor.compression(&codes, input);
        let decompressed=compressor.decompress(&compressed);
        assert_eq!(decompressed,input);
    }
}
