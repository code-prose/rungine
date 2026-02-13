use std::collections::HashMap;
use std::intrinsics::log10f32;

type WordMap = HashMap<String, i64>;
type TfIdf = HashMap<String, Vec<(f32, DocPath)>>;

type DocPath = String;

pub struct Document {
    pub path: DocPath,
    pub num_words: i64,
    pub word_freqs: HashMap<String, i64>,
}

pub struct Indexer;

impl Indexer {
    pub fn create_map(
        text: Vec<String>,
        document_frequency: &mut HashMap<String, i64>,
    ) -> HashMap<String, i64> {
        let mut hmap = HashMap::new();
        for word in text.iter() {
            if hmap.contains_key(word) {
                let key_ref = hmap.get_mut(word).unwrap();
                *key_ref += 1;
            } else {
                hmap.insert(word.to_string(), 1);

                if document_frequency.contains_key(word) {
                    let key_ref = document_frequency.get_mut(word).unwrap();
                    *key_ref += 1;
                } else {
                    document_frequency.insert(word.to_string(), 1);
                }
            }
        }
        hmap
    }

    fn calc_tf_idf(num_docs: f32, num_docs_appear: f32, term_count: f32, word_count: f32) -> f32 {
        Indexer::calc_idf(num_docs, num_docs_appear) * Indexer::calc_tf(term_count, word_count)
    }

    fn calc_idf(num_docs: f32, num_docs_appear: f32) -> f32 {
        log10f32(num_docs / num_docs_appear)
    }

    fn calc_tf(term_count: f32, word_count: f32) -> f32 {
        term_count / word_count
    }

    pub fn create_tf_idfs(documents: &Vec<Document>, word_map: &WordMap) -> TfIdf {
        let mut tf_idfs: HashMap<String, Vec<(f32, DocPath)>> = HashMap::new();
        for doc in documents.iter() {
            for word in doc.word_freqs.keys() {
                if tf_idfs.contains_key(word) {
                    let key_ref = tf_idfs.get_mut(word).unwrap();
                    key_ref.push((
                        Indexer::calc_tf_idf(
                            documents.len() as f32,
                            word_map.get(word).unwrap().clone() as f32,
                            doc.word_freqs.get(word).unwrap().clone() as f32,
                            doc.num_words.clone() as f32,
                        ),
                        doc.path.clone(),
                    ));
                } else {
                    let mut t_vec: Vec<(f32, DocPath)> = Vec::new();
                    t_vec.push((
                        Indexer::calc_tf_idf(
                            documents.len() as f32,
                            word_map.get(word).unwrap().clone() as f32,
                            doc.word_freqs.get(word).unwrap().clone() as f32,
                            doc.num_words.clone() as f32,
                        ),
                        doc.path.clone(),
                    ));

                    tf_idfs.insert(word.to_string(), t_vec);
                }
            }
        }
        tf_idfs
    }

    pub fn create_word_map(documents: &Vec<Document>) -> WordMap {
        let mut word_map: HashMap<String, i64> = HashMap::new();
        for doc in documents.iter() {
            for word in doc.word_freqs.keys() {
                if word_map.contains_key(word) {
                    let key_ref = word_map.get_mut(word).unwrap();
                    *key_ref += 1;
                } else {
                    word_map.insert(word.clone().to_string(), 1);
                }
            }
        }
        word_map
    }
}
