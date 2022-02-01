# Rust-based Natural Language Toolkit (rsnltk)
A Rust library to support natural language processing with Python bindings

## Features
The `rsnltk` library integrates various existing Python-based NLP toolkits for powerful text analysis in Rust-based applications. 

## Current Features
This toolkit is based on the Python-based [Stanza](https://stanfordnlp.github.io/stanza/) and other important libraries.

A list of functions from Stanza we bind here include:
- Tokenize
- Sentence Segmentation
- Multi-Word Token Expansion
- Part-of-Speech & Morphological Features
- Named Entity Recognition
- Sentiment Analysis
- Language Identification

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

Or you can manually install those [language models](https://stanfordnlp.github.io/stanza/available_models.html) via the Python-based `ner-kit` package which provides more features in using Stanza. Go to: [ner-kit](https://pypi.org/project/ner-kit/)

If no error occur in the above example, then it works. Finally, you can try the following advanced example usage.

Currently, we tested the use of English and Chinese language models; however, other language models should work as well. 

## Examples

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

Example 4: Tokenize

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

## Credits

Thank [Stanford NLP Group](https://github.com/stanfordnlp/stanza) for their hard work in [Stanza](https://stanfordnlp.github.io/stanza/). 

## License
MIT

