use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;


fn main() -> anyhow::Result<()>{
    let res = Grammar::parse(Rule::file, "-44.43,-6,-9\n-87.0,-5,6.7,8\n")?;
    println!("{:?}", res);
    Ok(())
}

