use std::{collections::HashMap, fs::File, io::Read, iter::Sum};

fn main() -> std::io::Result<()> {
    let mut file = File::open("input1")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    //     let contents = "190: 10 19
    // 3267: 81 40 27
    // 83: 17 5
    // 156: 15 6
    // 7290: 6 8 6 15
    // 161011: 16 10 13
    // 192: 17 8 14
    // 21037: 9 7 18 13
    // 292: 11 6 16 20";
    let mut sum: u64 = 0;
    for l in contents.trim().split("\n") {
        let (target, nums) = l.split_once(": ").unwrap();
        let target = target.parse::<u64>().unwrap();
        let nums: Vec<_> = nums.split(" ").map(|n| n.parse::<u64>().unwrap()).collect();
        let cur = nums[0];
        let sl = &nums[1..];
        if let Some(outp) = recurse_calc(cur, sl, target) {
            sum = sum.checked_add(outp).unwrap();
        }
    }
    println!("{}", sum);

    Ok(())
}

fn recurse_calc(curr: u64, slice: &[u64], target: u64) -> Option<u64> {
    let next = slice[0];
    if slice.len() == 1 {
        if curr.checked_add(next).is_some_and(|v| v == target)
            || curr.checked_mul(next).is_some_and(|v| v == target)
            || curr * 10u64.pow(next.ilog10() + 1) + next == target
        {
            return Some(target);
        }
        return None;
    }

    let rest = &slice[1..];

    let nnext = curr.checked_add(next);
    let mut res = None;
    if let Some(nn) = nnext {
        if nn <= target {
            res = recurse_calc(nn, rest, target);
        }
    }
    let nnext = curr.checked_mul(next);
    if let Some(nn) = nnext {
        if nn <= target {
            res = res.or(recurse_calc(nn, rest, target));
        }
    }
    let nnext = (curr * 10u64.pow(next.ilog10() + 1)).checked_add(next);
    if let Some(nn) = nnext {
        if nn <= target {
            res = res.or(recurse_calc(nn, rest, target));
        }
    }
    res
}
