use super::{Tensor, NeuralNetwork};
use std::collections::{HashMap, HashSet};
use anyhow::{Result, anyhow};

/// Text tokenizer for natural language processing
#[derive(Debug, Clone)]
pub struct Tokenizer {
    vocab: HashMap<String, usize>,
    reverse_vocab: HashMap<usize, String>,
    special_tokens: HashMap<String, usize>,
    max_length: usize,
}

impl Tokenizer {
    pub fn new(max_length: usize) -> Self {
        let mut tokenizer = Self {
            vocab: HashMap::new(),
            reverse_vocab: HashMap::new(),
            special_tokens: HashMap::new(),
            max_length,
        };
        
        // Add special tokens
        tokenizer.add_special_token("[PAD]", 0);
        tokenizer.add_special_token("[UNK]", 1);
        tokenizer.add_special_token("[CLS]", 2);
        tokenizer.add_special_token("[SEP]", 3);
        tokenizer.add_special_token("[MASK]", 4);
        
        tokenizer
    }
    
    fn add_special_token(&mut self, token: &str, id: usize) {
        self.special_tokens.insert(token.to_string(), id);
        self.vocab.insert(token.to_string(), id);
        self.reverse_vocab.insert(id, token.to_string());
    }
    
    /// Build vocabulary from a corpus of texts
    pub fn build_vocab(&mut self, texts: &[String], min_freq: usize) {
        let mut word_counts = HashMap::new();
        
        // Count word frequencies
        for text in texts {
            let words = self.basic_tokenize(text);
            for word in words {
                *word_counts.entry(word).or_insert(0) += 1;
            }
        }
        
        // Add words that meet minimum frequency threshold
        let mut next_id = self.special_tokens.len();
        for (word, count) in word_counts {
            if count >= min_freq && !self.vocab.contains_key(&word) {
                self.vocab.insert(word.clone(), next_id);
                self.reverse_vocab.insert(next_id, word);
                next_id += 1;
            }
        }
    }
    
    /// Basic word tokenization
    fn basic_tokenize(&self, text: &str) -> Vec<String> {
        text.to_lowercase()
            .split_whitespace()
            .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }
    
    /// Encode text to token IDs
    pub fn encode(&self, text: &str) -> Vec<usize> {
        let words = self.basic_tokenize(text);
        let mut tokens = vec![self.special_tokens["[CLS]"]];
        
        for word in words {
            let token_id = self.vocab.get(&word)
                .copied()
                .unwrap_or(self.special_tokens["[UNK]"]);
            tokens.push(token_id);
            
            if tokens.len() >= self.max_length - 1 {
                break;
            }
        }
        
        tokens.push(self.special_tokens["[SEP]"]);
        
        // Pad to max length
        while tokens.len() < self.max_length {
            tokens.push(self.special_tokens["[PAD]"]);
        }
        
        tokens
    }
    
    /// Decode token IDs to text
    pub fn decode(&self, token_ids: &[usize]) -> String {
        token_ids.iter()
            .filter_map(|&id| self.reverse_vocab.get(&id))
            .filter(|&token| !["[PAD]", "[CLS]", "[SEP]"].contains(&token.as_str()))
            .cloned()
            .collect::<Vec<_>>()
            .join(" ")
    }
    
    /// Get vocabulary size
    pub fn vocab_size(&self) -> usize {
        self.vocab.len()
    }
}

/// Word embeddings
#[derive(Debug, Clone)]
pub struct WordEmbeddings {
    embeddings: Tensor,
    vocab_size: usize,
    embedding_dim: usize,
}

impl WordEmbeddings {
    pub fn new(vocab_size: usize, embedding_dim: usize) -> Self {
        let embeddings = Tensor::random(vec![vocab_size, embedding_dim]);
        Self {
            embeddings,
            vocab_size,
            embedding_dim,
        }
    }
    
    /// Get embedding for a token ID
    pub fn get_embedding(&self, token_id: usize) -> Result<Tensor> {
        if token_id >= self.vocab_size {
            return Err(anyhow!("Token ID {} out of vocabulary range", token_id));
        }
        
        // Extract row from embeddings matrix
        let start_idx = token_id * self.embedding_dim;
        let end_idx = start_idx + self.embedding_dim;
        let embedding_data = self.embeddings.data()[start_idx..end_idx].to_vec();
        
        Tensor::new(embedding_data, vec![self.embedding_dim])
    }
    
    /// Get embeddings for a sequence of token IDs
    pub fn get_sequence_embeddings(&self, token_ids: &[usize]) -> Result<Tensor> {
        let mut sequence_embeddings = Vec::new();
        
        for &token_id in token_ids {
            let embedding = self.get_embedding(token_id)?;
            sequence_embeddings.extend_from_slice(embedding.data());
        }
        
        Tensor::new(sequence_embeddings, vec![token_ids.len(), self.embedding_dim])
    }
}

/// Positional encoding for transformer models
#[derive(Debug, Clone)]
pub struct PositionalEncoding {
    encodings: Tensor,
    max_length: usize,
    d_model: usize,
}

impl PositionalEncoding {
    pub fn new(max_length: usize, d_model: usize) -> Self {
        let mut encodings = vec![0.0; max_length * d_model];
        
        for pos in 0..max_length {
            for i in (0..d_model).step_by(2) {
                let angle = pos as f32 / 10000.0_f32.powf(i as f32 / d_model as f32);
                encodings[pos * d_model + i] = angle.sin();
                if i + 1 < d_model {
                    encodings[pos * d_model + i + 1] = angle.cos();
                }
            }
        }
        
        let encodings_tensor = Tensor::new(encodings, vec![max_length, d_model]).unwrap();
        
        Self {
            encodings: encodings_tensor,
            max_length,
            d_model,
        }
    }
    
    /// Add positional encoding to input embeddings
    pub fn add_encoding(&self, embeddings: &Tensor) -> Result<Tensor> {
        if embeddings.shape().len() != 2 {
            return Err(anyhow!("Expected 2D embeddings tensor"));
        }
        
        let seq_len = embeddings.shape()[0];
        if seq_len > self.max_length {
            return Err(anyhow!("Sequence length exceeds maximum"));
        }
        
        // Extract positional encodings for the sequence length
        let pos_data = &self.encodings.data()[..seq_len * self.d_model];
        let pos_encodings = Tensor::new(pos_data.to_vec(), vec![seq_len, self.d_model])?;
        
        embeddings.add(&pos_encodings)
    }
}

/// Text classification model
pub struct TextClassifier {
    tokenizer: Tokenizer,
    embeddings: WordEmbeddings,
    model: NeuralNetwork,
    num_classes: usize,
}

impl TextClassifier {
    pub fn new(vocab_size: usize, embedding_dim: usize, hidden_dim: usize, num_classes: usize, max_length: usize) -> Self {
        let tokenizer = Tokenizer::new(max_length);
        let embeddings = WordEmbeddings::new(vocab_size, embedding_dim);
        
        // Create a simple classification model
        let mut model = NeuralNetwork::new();
        model.add_layer(Box::new(super::Dense::new(max_length * embedding_dim, hidden_dim)));
        model.add_layer(Box::new(super::neural_networks::Dropout::new(0.3)));
        model.add_layer(Box::new(super::Dense::new(hidden_dim, hidden_dim)));
        model.add_layer(Box::new(super::Dense::new(hidden_dim, num_classes)));
        
        Self {
            tokenizer,
            embeddings,
            model,
            num_classes,
        }
    }
    
    /// Train the classifier on labeled data
    pub fn train(&mut self, texts: &[String], labels: &[usize], epochs: usize) -> Result<()> {
        // Build vocabulary from training texts
        self.tokenizer.build_vocab(texts, 2);
        
        // Prepare training data
        let mut inputs = Vec::new();
        let mut targets = Vec::new();
        
        for (text, &label) in texts.iter().zip(labels.iter()) {
            let token_ids = self.tokenizer.encode(text);
            let embeddings = self.embeddings.get_sequence_embeddings(&token_ids)?;
            
            // Flatten embeddings for the model
            let flattened = embeddings.reshape(vec![embeddings.data().len()])?;
            inputs.push(flattened);
            
            // One-hot encode labels
            let mut target = vec![0.0; self.num_classes];
            target[label] = 1.0;
            targets.push(Tensor::new(target, vec![self.num_classes])?);
        }
        
        self.model.train(&inputs, &targets, epochs)
    }
    
    /// Predict class for a text
    pub fn predict(&self, text: &str) -> Result<usize> {
        let token_ids = self.tokenizer.encode(text);
        let embeddings = self.embeddings.get_sequence_embeddings(&token_ids)?;
        let flattened = embeddings.reshape(vec![embeddings.data().len()])?;
        
        let output = self.model.forward(&flattened)?;
        
        // Find class with highest probability
        let mut max_idx = 0;
        let mut max_val = output.data()[0];
        
        for (i, &val) in output.data().iter().enumerate() {
            if val > max_val {
                max_val = val;
                max_idx = i;
            }
        }
        
        Ok(max_idx)
    }
    
    /// Get prediction probabilities for all classes
    pub fn predict_proba(&self, text: &str) -> Result<Vec<f32>> {
        let token_ids = self.tokenizer.encode(text);
        let embeddings = self.embeddings.get_sequence_embeddings(&token_ids)?;
        let flattened = embeddings.reshape(vec![embeddings.data().len()])?;
        
        let output = self.model.forward(&flattened)?;
        let probabilities = output.softmax(0)?;
        
        Ok(probabilities.data().to_vec())
    }
}

/// Named Entity Recognition (NER) model
pub struct NERModel {
    tokenizer: Tokenizer,
    embeddings: WordEmbeddings,
    model: NeuralNetwork,
    label_map: HashMap<String, usize>,
    reverse_label_map: HashMap<usize, String>,
}

impl NERModel {
    pub fn new(vocab_size: usize, embedding_dim: usize, hidden_dim: usize, labels: &[String], max_length: usize) -> Self {
        let tokenizer = Tokenizer::new(max_length);
        let embeddings = WordEmbeddings::new(vocab_size, embedding_dim);
        
        // Create label mappings
        let mut label_map = HashMap::new();
        let mut reverse_label_map = HashMap::new();
        
        for (i, label) in labels.iter().enumerate() {
            label_map.insert(label.clone(), i);
            reverse_label_map.insert(i, label.clone());
        }
        
        // Create sequence labeling model
        let mut model = NeuralNetwork::new();
        model.add_layer(Box::new(super::neural_networks::LSTM::new(embedding_dim, hidden_dim)));
        model.add_layer(Box::new(super::Dense::new(hidden_dim, labels.len())));
        
        Self {
            tokenizer,
            embeddings,
            model,
            label_map,
            reverse_label_map,
        }
    }
    
    /// Extract named entities from text
    pub fn extract_entities(&self, text: &str) -> Result<Vec<(String, String, usize, usize)>> {
        let words = self.tokenizer.basic_tokenize(text);
        let token_ids = self.tokenizer.encode(text);
        let embeddings = self.embeddings.get_sequence_embeddings(&token_ids)?;
        
        // Get predictions for each token
        let mut entities = Vec::new();
        let mut current_entity: Option<(String, usize, usize)> = None;
        
        for (i, word) in words.iter().enumerate() {
            if i >= embeddings.shape()[0] {
                break;
            }
            
            // Extract embedding for this position
            let start_idx = i * embeddings.shape()[1];
            let end_idx = start_idx + embeddings.shape()[1];
            let token_embedding = Tensor::new(
                embeddings.data()[start_idx..end_idx].to_vec(),
                vec![embeddings.shape()[1]]
            )?;
            
            let output = self.model.forward(&token_embedding)?;
            
            // Find predicted label
            let mut max_idx = 0;
            let mut max_val = output.data()[0];
            
            for (j, &val) in output.data().iter().enumerate() {
                if val > max_val {
                    max_val = val;
                    max_idx = j;
                }
            }
            
            if let Some(label) = self.reverse_label_map.get(&max_idx) {
                if label != "O" { // Not "Outside" label
                    if let Some((entity_type, start_pos, end_pos)) = &mut current_entity {
                        if label == entity_type {
                            // Continue current entity
                            *end_pos = i + word.len();
                        } else {
                            // End current entity and start new one
                            entities.push((entity_type.clone(), text[*start_pos..*end_pos].to_string(), *start_pos, *end_pos));
                            current_entity = Some((label.clone(), i, i + word.len()));
                        }
                    } else {
                        // Start new entity
                        current_entity = Some((label.clone(), i, i + word.len()));
                    }
                } else {
                    // End current entity if any
                    if let Some((entity_type, start_pos, end_pos)) = current_entity.take() {
                        entities.push((entity_type, text[start_pos..end_pos].to_string(), start_pos, end_pos));
                    }
                }
            }
        }
        
        // Add final entity if any
        if let Some((entity_type, start_pos, end_pos)) = current_entity {
            entities.push((entity_type, text[start_pos..end_pos].to_string(), start_pos, end_pos));
        }
        
        Ok(entities)
    }
}

/// Sentiment analysis model
pub struct SentimentAnalyzer {
    classifier: TextClassifier,
}

impl SentimentAnalyzer {
    pub fn new(vocab_size: usize, embedding_dim: usize, hidden_dim: usize, max_length: usize) -> Self {
        // 3 classes: negative (0), neutral (1), positive (2)
        let classifier = TextClassifier::new(vocab_size, embedding_dim, hidden_dim, 3, max_length);
        
        Self { classifier }
    }
    
    /// Analyze sentiment of text
    pub fn analyze(&self, text: &str) -> Result<(String, f32)> {
        let probabilities = self.classifier.predict_proba(text)?;
        
        let labels = ["negative", "neutral", "positive"];
        let mut max_idx = 0;
        let mut max_prob = probabilities[0];
        
        for (i, &prob) in probabilities.iter().enumerate() {
            if prob > max_prob {
                max_prob = prob;
                max_idx = i;
            }
        }
        
        Ok((labels[max_idx].to_string(), max_prob))
    }
    
    /// Train the sentiment analyzer
    pub fn train(&mut self, texts: &[String], sentiments: &[usize], epochs: usize) -> Result<()> {
        self.classifier.train(texts, sentiments, epochs)
    }
}
