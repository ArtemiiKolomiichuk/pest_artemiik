use pest_artemiik::*;
use pest::Parser;
    
#[cfg(test)]
mod tests{
    use super::*;
    mod parser{
        use super::*;
        #[test]
        fn first_field_iterates() -> anyhow::Result<()>{
            let data = Grammar::parse(Rule::field, "-44.43,-15")?.next().ok_or_else( || anyhow::anyhow!("no field"))?;
            print!("{:?}", data);
            assert_eq!(data.as_str(), "-44.43");
            assert_eq!(data.as_span().start(), 0);
            Ok(())
        }

        #[test]
        fn second_field_iterates() -> anyhow::Result<()> {
            let mut iter = Grammar::parse(Rule::record, "-44.43,-15")?;
            let record = iter.next().ok_or_else(|| anyhow::anyhow!("no record"))?;
            let mut inner_pairs = record.into_inner();
            inner_pairs.next().ok_or_else(|| anyhow::anyhow!("no first field"))?;
            let second_field = inner_pairs.next().ok_or_else(|| anyhow::anyhow!("no second field"))?;
            assert_eq!(second_field.as_str(), "-15");
            assert_eq!(second_field.as_span().start(), 7);
            Ok(())
        }

        #[test]
        fn record_iterates() -> anyhow::Result<()>{
            let mut iter = Grammar::parse(Rule::record, "-44.43,-15")?;
            let data = iter.next().ok_or_else( || anyhow::anyhow!("no field"))?;
            assert_eq!(data.as_str(), "-44.43,-15");
            assert_eq!(data.as_span().start(), 0);
            Ok(())
        }

        #[test]
        fn wrong_field_is_err() -> anyhow::Result<()>{
            let res = Grammar::parse(Rule::field, "--.0--.0..");
            assert!(res.is_err());
            Ok(())
        }
    }
    
}
