// Ref: https://towardsdatascience.com/a-simple-text-summarizer-written-in-rust-4df05f9327a5
// From Author: Charles Chan
#[cfg(test)]
mod tests{
    use crate::native::summarizer::*;
    # [test]
    fn test_summarize(){
        let text="As of Sunday, there were more than 58.2 million reported cases of COVID-19 worldwide, with more than 37.2 million of those cases listed as recovered, according to a COVID-19 tracking tool maintained by Johns Hopkins University. The global death toll stood at more than 1.3 million. In Asia, the daily tally of reported cases in Japan hit a record for the fourth day in a row, with 2,508 people confirmed infected, the Health Ministry said Sunday. A flurry of criticism has erupted, from opposition legislators and the public, slamming the government as having acted too slowly in halting its \"GoTo\" campaign, which encouraged travel and dining out with discounts. In Europe, French authorities ordered the culling of all minks at a farm after analysis showed a mutated version of the coronavirus was circulating among the animals. The move follows virus developments in mink farms in Denmark and other countries, including the Netherlands, Sweden and Greece. In the Americas, Chile says it will open its main border crossing and principal airport to foreign visitors on Monday after an eight-month pandemic shutdown. Arrivals will have to present evidence of a recent negative test for the novel coronavirus, as well as health insurance. They'll also have to report their whereabouts and health status for a two-week watch period. Those coming from high-risk countries will have to quarantine for 14 days. In Africa, Sudan's minister of cabinet affairs on Sunday tested positive for the coronavirus, the prime minister's office said, the latest in a string of senior officials to be infected as the country shows an increase of confirmed cases of COVID-19. Over the past month, acting ministers of finance and health, the central bank governor and two associates to Prime Minister Abdalla Hamdok have tested positive.";
        let stopwords=&[];
        let summarized_text=summarize(text,stopwords,5);
        println!("{}",summarized_text);
    }
}

use unicode_segmentation::UnicodeSegmentation;
use std::collections::BTreeSet;
use ndarray::{Array1, Array2};

///
/// Summarize text
///
pub fn summarize(text: &str, stop_words: &[&str], num_sentence: usize) -> String {
    let sentences = text.unicode_sentences().collect::<Vec<&str>>();
    if num_sentence >= sentences.len() {
        return text.to_string();
    }
    let mut sentences_and_words = vec![];
    sentences.iter().for_each(|&sentence| {
        let words = split_into_words(sentence);
        sentences_and_words.push(words);
    });
    let matrix = build_similarity_matrix(&sentences_and_words, stop_words);
    let ranks = calculate_sentence_rank(&matrix);
    let mut sorted_ranks = ranks.clone();
    sorted_ranks.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let least_rank = sorted_ranks[num_sentence + 1];
    let mut result: Vec<&str> = vec![];
    let mut included_count = 0;
    for i in 0..sentences.len() {
        if ranks[i] >= least_rank {
            included_count = included_count + 1;
            result.push(sentences[i]);
        }
        if included_count == num_sentence {
            break;
        }
    }
    result.join("")
}

fn get_all_words_lc<'a>(sentence1: &[&'a str], sentence2: &[&'a str]) -> BTreeSet<String> {
    let mut all_words: BTreeSet<String> = BTreeSet::new();

    sentence1.iter().for_each(|w| {
        all_words.insert(w.to_lowercase());
    });

    sentence2.iter().for_each(|w| {
        all_words.insert(w.to_lowercase());
    });
    return all_words;
}

///
/// Retrieve a sentence vector based on the frequency of words that appears in the all_words_lc set.
/// all_words_lc should be a sorted set of lower cased words
/// The size of the resulting vector is the same as the all_words_lc set
/// stop_words are skipped
///
fn get_sentence_vector(sentence: &[&str], all_words_lc: &BTreeSet<String>, stop_words: &[&str]) -> Vec<usize> {
    let mut vector: Vec<usize> = vec![0; all_words_lc.len()];
    for word in sentence {
        let word_lc = word.to_lowercase();
        if !stop_words.contains(&word_lc.as_str()) {
            let index = all_words_lc.iter().position(|x| x.eq(&word_lc)).unwrap();
            vector[index] += 1;
        }
    }
    return vector;
}

///
/// Calculates the cosine distance between two vectors
/// Refer to [YouTube](https://www.youtube.com/watch?v=3X0wLRwU_Ws)
///
fn cosine_distance(vec1: &Vec<usize>, vec2: &Vec<usize>) -> f64 {
    let dot_product = dot_product(vec1, vec2);
    let root_sum_square1 = root_sum_square(vec1);
    let root_sum_square2 = root_sum_square(vec2);
    return dot_product as f64 / (root_sum_square1 * root_sum_square2);
}

fn root_sum_square(vec: &Vec<usize>) -> f64 {
    let mut sum_square = 0;
    for i in 0..vec.len() {
        sum_square += vec[i] * vec[i];
    }
    (sum_square as f64).sqrt()
}

fn dot_product(vec1: &Vec<usize>, vec2: &Vec<usize>) -> usize {
    let delta = vec1.len() - vec2.len();
    let shortest_vec = match delta {
        d if d < 0 => vec1,
        d if d > 0 => vec2,
        _ => vec1
    };
    let mut dot_product = 0;
    for i in 0..shortest_vec.len() {
        dot_product += vec1[i] * vec2[i];
    }
    dot_product
}

fn sentence_similarity(s1: &[&str], s2: &[&str], stop_words: &[&str]) -> f64 {
    let all_words = get_all_words_lc(s1, s2);
    let v1 = get_sentence_vector(s1, &all_words, stop_words);
    let v2 = get_sentence_vector(s2, &all_words, stop_words);
    1.0 - cosine_distance(&v1, &v2)
}

///
/// Calculate a similarity matrix for the given sentences.
/// Returns a 2-D array M_i,j such that for all 'j', sum(i, M_i,j) = 1
/// We take a leap of faith here and assume that cosine similarity is similar to the probability
/// that a sentence is important for summarization
///
fn build_similarity_matrix(sentences: &Vec<Vec<&str>>, stop_words: &[&str]) -> Array2<f64> {
    let len = sentences.len();
    let mut matrix = Array2::<f64>::zeros((len, len));
    let mut sum_column: Vec<f64> = vec![0.0; len];
    for i in 0..len {
        for j in 0..len {
            if i == j {
                continue;
            }
            matrix[[i, j]] = sentence_similarity(sentences[i].as_slice(), sentences[j].as_slice(), stop_words);
        }
    }
    // at this point we have the cosine similarity of each sentence.
    // take a leap of faith and assume that the cosine similarity is the probability that a sentence
    // is important for summarization.
    // We do this by normalizing the matrix along the column. The column values should add up to 1.
    for j in 0..len {
        let mut sum: f64 = 0.0;
        for i in 0..len {
            if i == j {
                continue;
            }
            sum += matrix[[i, j]];
        }
        sum_column[j] = sum;
    }
    for i in 0..len {
        for j in 0..len {
            if i == j {
                continue;
            }
            matrix[[i, j]] = matrix[[i, j]] / sum_column[j];
        }
    }
    matrix
}

///
/// Calculate a sentence rank similar to a page rank.
/// Please refer to [PageRank](https://en.wikipedia.org/wiki/PageRank) for more details.
///
fn calculate_sentence_rank(similarity_matrix: &Array2<f64>) -> Vec<f64> {
    let num_sentence = similarity_matrix.shape()[1];
    let threshold = 0.001;
    // Initialize a vector with the same value 1/number of sentences. Uniformly distributed across
    // all sentences. NOTE: perhaps we can make some sentences more important than the rest?
    let initial_vector: Vec<f64> = vec![1.0 / num_sentence as f64; num_sentence];
    let mut result = Array1::from(initial_vector);
    let mut prev_result = result.clone();
    let damping_factor = 0.85;
    let initial_m = damping_factor * similarity_matrix + (1.0 - damping_factor) / num_sentence as f64;
    loop {
        result = initial_m.dot(&result);
        let delta = &result - &prev_result;
        let mut converged = true;
        for i in 0..delta.len() {
            if delta[i] > threshold {
                converged = false;
                break;
            }
        }
        if converged {
            break;
        }
        prev_result = result.clone();
    }
    result.into_raw_vec()
}

fn split_into_words(sentence: &str) -> Vec<&str> {
    let mut result = vec![];
    let words = sentence.unicode_words();
    for word in words {
        result.push(word);
    }
    result
}