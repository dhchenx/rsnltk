# Rust-based Natural Language Toolkit (rsnltk)
A Rust library to support natural language processing with Python bindings

[Rust Docs](https://docs.rs/rsnltk/0.1.1) | [Crates Home Page](https://crates.io/crates/rsnltk) | [Tests](https://github.com/dhchenx/rsnltk/tree/main/tests) | [NER-Kit](https://pypi.org/project/ner-kit/)

## Features
The `rsnltk` library integrates various existing Python-written NLP toolkits for powerful text analysis in Rust-based applications. 

## Functions
This toolkit is based on the Python-written [Stanza](https://stanfordnlp.github.io/stanza/) and other important NLP crates.

A list of functions from Stanza and others we bind here include:
- Tokenize
- Sentence Segmentation
- Multi-Word Token Expansion
- Part-of-Speech & Morphological Features
- Named Entity Recognition
- Sentiment Analysis
- Language Identification
- Dependency Tree Analysis

Some amazing crates are also included in `rsnltk` but with simplified APIs for actual use:
- [word2vec](https://crates.io/crates/word2vec)
- [natural](https://crates.io/crates/natural), [yn](https://crates.io/crates/yn), [whatlang](https://crates.io/crates/whatlang). 

Additionally, we can calculate the similarity between words based on WordNet though the `semantic-kit` PyPI project via `pip install semantic-kit`.

## Installation

1. Make sure you install Python 3.6.6+ and PIP environment in your computer. Type `python -V` in the Terminal should print no error message;

2. Install our Python-based [ner-kit](https://pypi.org/project/ner-kit/) (version>=0.0.5a2) for binding the `Stanza` package via `pip install ner-kit==0.0.5a2`;

3. Then, Rust should be also installed in your computer. I use IntelliJ to develop Rust-based applications, where you can write Rust codes;

4. Create a simple Rust application project with a `main()` function. 

5. Add the `rsnltk` dependency to the `Cargo.toml` file, keep up the Latest version.

6. After you add the `rsnltk` dependency in the `toml file`, install necessary language models from Stanza using the following Rust code for the first time you use this package.

```rust
fn init_rsnltk_and_test(){
    // 1. first install the necessary language models using language codes
    let list_lang=vec!["en","zh"]; 
    //e.g. you install two language models, 
    // namely, for English and Chinese text analysis.
    download_langs(list_lang);
    // 2. then do test NLP tasks
    let text="I like Beijing!";
    let lang="en";
    // 2. Uncomment the below codes for Chinese NER
    // let text="我喜欢北京、上海和纽约！";
    // let lang="zh";
    let list_ner=ner(text,lang);
    for ner in list_ner{
        println!("{:?}",ner);
    }
}
```

Or you can manually install those [language models](https://stanfordnlp.github.io/stanza/available_models.html) via the Python-written `ner-kit` package which provides more features in using Stanza. Go to: [ner-kit](https://pypi.org/project/ner-kit/)

If no error occurs in the above example, then it works. Finally, you can try the following advanced example usage.

Currently, we tested the use of English and Chinese language models; however, other language models should work as well. 

## Examples with Stanza Bindings

Example 1: Part-of-speech Analysis

```rust
    fn test_pos(){
    //let text="我喜欢北京、上海和纽约！";
    //let lang="zh";
    let text="I like apple";
    let lang="en";

    let list_result=pos(text,lang);
    for word in list_result{
        println!("{:?}",word);
    }
}
```

Example 2: Sentiment Analysis
```rust
    fn test_sentiment(){
        //let text="I like Beijing!";
        //let lang="en";
        let text="我喜欢北京";
        let lang="zh";

        let sentiments=sentiment(text,lang);
        for sen in sentiments{
            println!("{:?}",sen);
        }
    }
```

Example 3: Named Entity Recognition

```rust
    fn test_ner(){
        // 1. for English NER
        let text="I like Beijing!";
        let lang="en";
        // 2. Uncomment the below codes for Chinese NER
        // let text="我喜欢北京、上海和纽约！";
        // let lang="zh";
        let list_ner=ner(text,lang);
        for ner in list_ner{
            println!("{:?}",ner);
        }
    }
```

Example 4: Tokenize for Multiple Languages

```rust
    fn test_tokenize(){

        let text="我喜欢北京、上海和纽约！";
        let lang="zh";

        let list_result=tokenize(text,lang);
        for ner in list_result{
            println!("{:?}",ner);
        }
    }
```

Example 5: Tokenize Sentence

```rust
    fn test_tokenize_sentence(){
        let text="I like apple. Do you like it? No, I am not sure!";
        let lang="en";
        let list_sentences=tokenize_sentence(text,lang);
        for sentence in list_sentences{
            println!("Sentence: {}",sentence);
        }
    }
```

Example 6: Language Identification

```rust
fn test_lang(){
    let list_text = vec!["I like Beijing!",
                         "我喜欢北京！", 
                         "Bonjour le monde!"];
    let list_result=lang(list_text);
    for lang in list_result{
        println!("{:?}",lang);
    }
}
```

Example 7: MWT expand

```rust
    fn test_mwt_expand(){
        let text="Nous avons atteint la fin du sentier.";
        let lang="fr";
        let list_result=mwt_expand(text,lang);
    }
```

Example 8: Estimate the similarity between words in WordNet

You need to firstly install `semantic-kit` PyPI package!

```rust
    fn test_wordnet_similarity(){
        let s1="dog.n.1";
        let s2="cat.n.2";
        let sims=wordnet_similarity(s1,s2);
        for sim in sims{
            println!("{:?}",sim);
        }
    }
```

Example 9: Obtain a dependency tree from a text
```rust
fn test_dependency_tree(){
    let text="I like you. Do you like me?";
    let lang="en";
    let list_results=dependency_tree(text,lang);
    for list_token in list_results{
        for token in list_token{
            println!("{:?}",token)
        }

    }
}
```

## Examples in Pure Rust

Example 1: Word2Vec similarity

```rust
fn test_open_wv_bin(){
    let wv_model=wv_get_model("GoogleNews-vectors-negative300.bin");
    let positive = vec!["woman", "king"];
    let negative = vec!["man"];
    println!("analogy: {:?}", wv_analogy(&wv_model,positive, negative, 10));
    println!("cosine: {:?}", wv_cosine(&wv_model,"man", 10));
}
```

Example 2: Text summarization

```rust
    use rsnltk::native::summarizer::*;
    fn test_summarize(){
        let text="Some large txt...";
        let stopwords=&[];
        let summarized_text=summarize(text,stopwords,5);
        println!("{}",summarized_text);
    }
```

Example 3: Get token list from English strings
```rust
use rsnltk::native::token::get_token_list;
fn test_get_token_list(){
        let s="Hello, Rust. How are you?";
        let result=get_token_list(s);
        for r in result{
            println!("{}\t{:?}",r.text,r);
        }
}
```

Example 4: Word segmentation for some language where no space exists between terms, e.g. Chinese text.

We implement three word segmentation methods in this version:

- Forward Maximum Matching (fmm), which is baseline method
- Backward Maximum Matching (bmm), which is considered better
- Bidirectional Maximum Matching (bimm), high accuracy but low speed

```rust
use rsnltk::native::segmentation::*;
fn test_real_word_segmentation(){
    let dict_path="30wdict.txt"; // empty if only for tokenizing
    let stop_path="baidu_stopwords.txt";// empty when no stop words
    let _sentence="美国太空总署希望，在深海的探险发现将有助于解开一些外太空的秘密，\
    同时也可以测试前往太阳系其他星球探险所需的一些设备和实验。";
    let meaningful_words=get_segmentation(_sentence,dict_path,stop_path, "bimm");
    // bimm can be changed to fmm or bmm. 
    println!("Result: {:?}",meaningful_words);
}
```

## Credits

Thank [Stanford NLP Group](https://github.com/stanfordnlp/stanza) for their hard work in [Stanza](https://stanfordnlp.github.io/stanza/). 

## License
MIT

