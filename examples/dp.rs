use std::vec;

fn main() {
    println!("{}", climb_stairs(5));
    let matrix = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    println!("{}", find_path_sum(matrix));
    let triangle = vec![vec![1], vec![5, 1], vec![4, 2, 1], vec![1, 2, 3, 9]];
    println!("{}", find_path_sum_in_triganle(triangle));
    let coins = vec![2, 8, 2];
    println!("{}", coin_change(&coins, 11));
    println!("{}", coin_change_dp(coins, 11));
    let v = vec![1, 2, 8, 1, 3, 5, 76, 2];

    println!("{}", find_increasing_array(v));

    println!("{}", min_distance("horse".to_string(), "ros".to_string()));
}

fn climb_stairs(n: i32) -> i32 {
    if n < 2 {
        return n;
    }
    let mut v = vec![0; (n + 1) as usize];
    v[1] = 1;
    v[2] = 2;
    for i in 3..v.len() {
        v[i] = v[i - 1] + v[i - 2];
    }
    v.pop().unwrap()
}

fn find_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    if m == 0 || n == 0 {
        return 0;
    }
    let mut status = vec![vec![0; n]; m];
    let mut sum = 0;
    for i in 0..n {
        sum += matrix[0][i];
        status[0][i] = sum;
    }
    sum = 0;
    for i in 0..m {
        sum += matrix[i][0];
        status[i][0] = sum;
    }
    for i in 1..m {
        for j in 1..n {
            status[i][j] = matrix[i][j] + status[i - 1][j].min(status[i][j - 1]);
        }
    }
    status[m - 1][n - 1]
}

fn find_path_sum_in_triganle(mut triangle: Vec<Vec<i32>>) -> i32 {
    let m = triangle.len();
    if m == 0 {
        return 0;
    }
    for i in (0..m - 1).rev() {
        for j in 0..triangle[i].len() {
            if j > 0 {
                triangle[i][j] = triangle[i][j]
                    + triangle[i + 1][j]
                        .min(triangle[i + 1][j + 1])
                        .min(triangle[i + 1][j - 1]);
            } else {
                triangle[i][j] = triangle[i][j] + triangle[i + 1][j].min(triangle[i + 1][j + 1]);
            }
        }
    }
    triangle[0][0]
}

fn coin_change(coins: &Vec<i32>, amount: i32) -> i32 {
    if amount < 0 {
        return -1;
    }
    if amount == 0 {
        return 0;
    }
    if coins.contains(&amount) {
        return 1;
    }
    let mut v = vec![];
    for i in coins {
        if amount > *i {
            let count = coin_change(coins, amount - *i);
            if count > 0 {
                v.push(count + 1)
            }
        }
    }
    if v.is_empty() {
        return -1;
    }
    let mut min = v[0];
    for i in v {
        min = min.min(i);
    }
    min
}

fn coin_change_dp(coins: Vec<i32>, amount: i32) -> i32 {
    if coins.is_empty() || amount <= 0 {
        return -1;
    }
    let mut status = vec![amount + 1; (amount + 1) as usize];
    status[0] = 0;
    for i in 1..=amount {
        //目标金额
        for j in coins.iter() {
            //可用金额
            if i >= *j {
                //目标额度大  能选用这张钱
                status[i as usize] = status[i as usize].min(status[i as usize - *j as usize] + 1);
            }
        }
    }
    let last = *status.last().unwrap();
    println! {"{:?}",status};
    if last > amount {
        -1
    } else {
        last
    }
}

fn find_increasing_array(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut status = vec![1; nums.len()];
    let mut max = 1;
    for i in 0..nums.len() {
        for j in 0..i {
            if nums[i] > nums[j] {
                status[i] = status[i].max(status[j] + 1);
            }
        }
        max = max.max(status[i]);
    }
    println!("{:?}", status);

    max
}

fn min_distance(word1: String, word2: String) -> i32 {
    if word1 == word2 {
        return 0;
    }
    let mut status = vec![vec![0; word1.len() + 1]; word2.len() + 1];
    let chars1: Vec<char> = word1.chars().collect();
    for i in 1..=chars1.len() {
        status[0][i] = i;
    }
    let chars2: Vec<char> = word2.chars().collect();
    for i in 1..=chars2.len() {
        status[i][0] = i;
    }
    println!("{:?}", status);

    for i in 1..=chars2.len() {
        for j in 1..=chars1.len() {
            if chars2[i - 1] == chars1[j - 1] {
                status[i][j] = status[i - 1][j - 1];
            } else {
                status[i][j] = status[i - 1][j - 1] //替换
                    .min(status[i - 1][j])//插入
                    .min(status[i][j - 1]) //删除
                    + 1;
            }
        }
    }
    for v in &status {
        println!("{:?}", v);
    }
    status[chars2.len()][chars1.len()] as i32
}
