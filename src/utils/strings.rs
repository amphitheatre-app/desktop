// Copyright 2023 The Amphitheatre Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use rand::{thread_rng, Rng};

pub fn generate_random_word(length: usize) -> String {
    let mut rng = thread_rng();
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

    // 生成随机单词
    let word: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    word
}

pub fn get_random_words(count: usize) -> Vec<String> {
    let mut rng = thread_rng();
    let mut random_words: Vec<String> = Vec::new();

    for _ in 0..count {
        let word_length = rng.gen_range(3..11);
        random_words.push(generate_random_word(word_length));
    }

    random_words
}

pub fn generate_random_words_string(range: std::ops::Range<usize>) -> String {
    let mut rng = thread_rng();
    let count = rng.gen_range(range);

    get_random_words(count).join(" ")
}
