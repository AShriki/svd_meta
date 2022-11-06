use roxmltree::Document;
use std::fs::read_to_string;

fn main() {
    match read_to_string("EFM32PG22C200F128IM32.svd"){
        Ok(fc) => {
            let doc = match Document::parse(&fc){
                Ok(document) => document,
                Err(e) => panic!("failed to parse tree: {}", e),
            };
            // println!("{:?}",doc);
        },
        Err(e) => println!("failed to get tree: {}", e),
    }
}
