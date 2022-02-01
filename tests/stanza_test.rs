
#[cfg(test)]
mod tests {
    use rsnltk::{download_lang, ner, tokenize, download_langs, tokenize_sentence, lang, sentiment, mwt_expand, pos, dependency_tree};
    use crate::*;

    # [test] // before use the rsnltk library, you need to download target language package from Stanza's website.
    fn test_download_langs(){
        // 1. first install the package
        let list_lang=vec!["en","zh"];
        download_langs(list_lang);
        // 2. then do NLP tasks
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

    #[test]
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
    # [test]
    fn test_tokenize(){

        let text="我喜欢北京、上海和纽约！";
        let lang="zh";

        let list_result=tokenize(text,lang);
        for ner in list_result{
            println!("{:?}",ner);
        }
    }
    # [test]
    fn test_tokenize_sentence(){
        let text="I like apple. Do you like it? No, I am not sure!";
        let lang="en";
        let list_sentences=tokenize_sentence(text,lang);
        for sentence in list_sentences{
            println!("Sentence: {}",sentence);
        }
    }
    # [test]
    fn test_lang(){
        let list_text = vec!["I like Beijing!","我喜欢北京！", "Bonjour le monde!"];
        let list_result=lang(list_text);
        for lang in list_result{
            println!("{:?}",lang);
        }
    }
    # [test]
    fn test_mwt_expand(){
        let text="Nous avons atteint la fin du sentier.";
        let lang="fr";
        let list_result=mwt_expand(text,lang);
    }
    # [test]
    fn test_tag(){
        //let text="我喜欢北京、上海和纽约！";
        //let lang="zh";
        let text="I like apple";
        let lang="en";

        let list_result=pos(text,lang);
        for word in list_result{
            println!("{:?}",word);
        }
    }
    # [test]
    fn test_sentiment(){
        //let text="I like Beijing!";
        //let lang="en";
        let text="我讨厌北京";
        let lang="zh";

        let sentiments=sentiment(text,lang);
        for sen in sentiments{
            println!("{:?}",sen);
        }
    }

    # [test]
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

}