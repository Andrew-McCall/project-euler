/* p31: Coin Sums */
pub fn solution(coins: &[u64], target: u64) -> u64 {
    let mut coins = coins.to_vec();
    coins.sort();
    coins.reverse();

    let mut result: u64 = 0;
    let mut stack = vec![(0usize, target)];

    while let Some((index, remaining)) = stack.pop() {
        if remaining == 0 {
            result += 1;
            continue;
        }
        if index == coins.len() {
            continue;
        }

        let coin = coins[index];
        let mut r = remaining;
        while r as i32 >= 0 {
            stack.push((index + 1, r));
            if r < coin {
                break;
            }
            r -= coin;
        }
    }

    result
}

#[test]
fn test() {
    let coins = [1, 5, 2, 10, 20, 50, 100, 200];
    assert_eq!(solution(&coins, 200), 73682);
}
