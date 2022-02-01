#[cfg(test)]
mod tests{
    use crate::api::natural::*;
    # [test]
    fn test_distance(){
        println!("lev = {}",lev_dist("kitten", "sitting"));
        println!("winkler = {}",jw_dist("dixon", "dicksonx"));
    }

    # [test]
    fn test_soundx(){
        println!("{}",is_soundex("robert","rupert"));
    }

    # [test]
    fn test_tokenize(){
        let str1="hello, world!";
        let srtr="My dog has fleas.";
        println!("{:?}",tokenize(str1));

    }

    # [test]
    fn test_ngrams(){
        // no padding
        let str1="hello my darling";
        let results=get_ngram(str1,2);
        for l in results{
            println!("{:?}",l);
        }
        println!();
        // with padding
        let results=get_ngram_with_padding(str1,2,"---");
        for l in results{
            println!("{:?}",l);
        }
    }

    # [test]
    fn test_classification(){
        let mut list_str=Vec::new();
        list_str.push("Hello World");
        list_str.push("Hello Chen's World");
        list_str.push("World is Amazing");
        let mut list_label=Vec::new();
        list_label.push("a");
        list_label.push("b");
        list_label.push("c");
        println!("guess = {}",nb_guess(list_str,list_label,"Hello"));
    }

    # [test]
    fn test_tf_idf(){
        let mut list_str=Vec::new();
        list_str.push("this document is about rust.");
        list_str.push("this document is about erlang.");
        list_str.push("this document is about erlang and rust.");
        list_str.push("this document is about rust. it has rust examples");

        println!("tf-idf value = {}",get_tf_idf(list_str,"rust"));

    }

}

extern crate natural;
use natural::distance::jaro_winkler_distance;
use natural::distance::levenshtein_distance;
use natural::phonetics::soundex;
use natural::classifier::NaiveBayesClassifier;
use natural::tf_idf::TfIdf;

pub fn lev_dist(str1:&str,str2:&str)->usize{
    return levenshtein_distance(str1, str2);
}

pub fn jw_dist(str1:&str,str2:&str)->f32{
    return jaro_winkler_distance(str1, str2);
}

pub fn is_soundex(str1:&str,str2:&str)->bool{
    let result=soundex(str1,str2);
    return result;
}

pub fn tokenize(str:&str)->Vec<&str>{
    natural::tokenize::tokenize(&str)
}


pub fn get_ngram(str:&str,n:usize)->Vec<Vec<&str>>{
    natural::ngram::get_ngram(str, n)
}

pub fn get_ngram_with_padding<'a>(str:&'a str,n:usize,padding:&'a str)->Vec<Vec<&'a str>>{
    let result = natural::ngram::get_ngram_with_padding(str, n,padding);
    result
}

pub fn nb_guess(train_strs:Vec<&str>,labels:Vec<&str>,str_guess:&str)->String{


    let mut nbc = NaiveBayesClassifier::new();
    let mut idx=0;
    for train_str in train_strs{
        nbc.train(train_str, labels[idx]);
        idx+=1;
    }

    nbc.guess(str_guess) //returns a label with the highest probability

}

pub fn get_tf_idf(strs:Vec<&str>,s:&str)->f32{

    let mut tf_idf=TfIdf::new();


    for str in strs{
        tf_idf.add(str);
    }

    tf_idf.get(s) //0.21859923
}


