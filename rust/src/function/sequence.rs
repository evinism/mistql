use super::args::ArgParser;
use crate::prefix::truthiness;
use crate::{expr, Error, Result, Value};
use itertools::Itertools;

pub fn sequence(arg_parser: ArgParser) -> Result<Value> {
    let mut args = arg_parser.at_least_n_args(3)?;
    let target_val = args.pop().unwrap().to_value(arg_parser.data)?;

    let target = match target_val {
        Value::Array(arr) => Ok(arr),
        _ => Err(Error::eval(format!(
            "target of sequence must be an array (got {:?}",
            target_val
        ))),
    }?;

    let mut truths: Vec<Vec<usize>> = Vec::new();
    for predicate_arg in args {
        let predicate = predicate_arg.to_pair()?;
        truths.push(
            target
                .iter()
                .map(|elt| expr::eval(predicate.clone(), elt, None))
                .collect::<Result<Vec<Value>>>()?
                .iter()
                .enumerate()
                .filter(|(_, elt)| truthiness(elt))
                .map(|(n, _)| n)
                .collect(),
        );
    }

    let sequences: Vec<Value> = truths
        .iter()
        .map(|row| row.iter())
        .multi_cartesian_product()
        .filter(|seq| is_sorted(seq))
        .map(|seq| seq.into_iter().map(|elt| target[*elt].clone()).collect())
        .map(|seq| Value::Array(seq))
        .collect();

    dbg!(&sequences);

    Ok(Value::Array(sequences))
}

// there is a standard library function for this but it's unstable
fn is_sorted(arr: &Vec<&usize>) -> bool {
    let mut itr = arr.into_iter();
    match itr.next() {
        None => true,
        Some(first) => itr
            .scan(first, |state, next| {
                let cmp = *state < next;
                *state = next;
                Some(cmp)
            })
            .all(|b| b),
    }
}

#[cfg(test)]
mod tests {
    use crate::query_value;

    #[test]
    fn sequence_sequences_with_two_predicates() {
        assert_eq!(
            query_value(
                "[1, 2, 3, 4] | sequence (@ % 2) ((@ + 1)% 2)".to_string(),
                serde_json::Value::Null
            )
            .unwrap(),
            serde_json::Value::from(vec![vec![1, 2], vec![1, 4], vec![3, 4]])
        );
    }

    #[test]
    fn sequence_sequences_with_three_predicates() {
        assert_eq!(
            query_value(
                "[1, 2, 3, 4, 5, 6] | sequence (@ % 2) ((@ + 1)% 2) ((@ % 2)  == 0)".to_string(),
                serde_json::Value::Null
            )
            .unwrap(),
            serde_json::Value::from(vec![
                vec![1, 2, 4],
                vec![1, 2, 6],
                vec![1, 4, 6],
                vec![3, 4, 6]
            ])
        );
    }
}
