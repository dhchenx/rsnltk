

use std::*;
extern crate  unicode_segmentation;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;


///
/// Get word segmentation results from customized dictionaries and methods
///
/// Parameters:
///
///     _sentence: a string,
///
///     dict_path: the dictionary file path where each line is a term,
///
///     stopwords_path: the stopwords file path  where each line is a stop word,
///
///     method: if empty, use 'bimm', other optional values: fmm, bmm.
///
pub fn get_segmentation(_sentence:&str,dict_path:&str,stopwords_path:&str,method:&str)->Vec<String>{

    if dict_path.eq(""){
        let result= get_word_list(_sentence);
        let mut result_final:Vec<String>=Vec::new();
        for r in result{
            result_final.push(String::from(r));
        }
        return result_final;
    }

    // println!("loading common dictionary");
    let common_words=load_dictionary(dict_path);


    let stop_words:Vec<String>;
    if stopwords_path.eq(""){
        // println!("loading stopwords dictionary");
        stop_words=Vec::new();
    }else{
        stop_words=load_dictionary(stopwords_path);
    }


    // println!("doing segmentation tasks");

    let sentence=_sentence.graphemes(true).collect::<Vec<&str>>();
    let mut list_result:Vec<String>=Vec::new();

    if method=="bimm" || method==""{
        list_result=bimm(sentence,common_words);
    }else  if method=="fmm"{
        list_result=fmm(sentence,common_words);
    }else if method=="bmm"{
        list_result=bmm(sentence,common_words);
    }


    if !stopwords_path.eq(""){
        let mut meaningful_words:Vec<String>=Vec::new();
        // println!("removing stop words...");
        for word in list_result{
            if !stop_words.contains(&word){
                meaningful_words.push(word);
            }
        }
        return meaningful_words;
    }else{
        return list_result;
    }


}


///
/// Bidirection Maximum Matching Method
///
pub fn bimm(sentence:Vec<&str>, words_dict:Vec<String>)->Vec<String>{
    let s1=sentence.clone();
    let s2=sentence.clone();
    let dict1=words_dict.clone();
    let dict2=words_dict.clone();
    let forward =fmm(s1,dict1);
    let backward=bmm(s2,dict2);
    // println!("FMM: {:?}",forward);
    // println!("BMM: {:?}",backward);
    let mut f_single_word=0;
    let mut b_single_word=0;
    let mut tot_fmm=forward.len();
    let mut tot_bmm=backward.len();
    let mut oov_fmm=0;
    let mut oov_bmm=0;
    let mut score_fmm=0;
    let mut score_bmm=0;
    if forward==backward{
        return backward;
    }else{
        for each in forward.clone(){
            if each.len()==1{
                f_single_word+=1;
            }
        }
        for each in backward.clone(){
            if each.len()==1{
                b_single_word+=1;
            }
        }
        for each in forward.clone(){
            if !words_dict.contains(&each){
                oov_fmm+=1;
            }
        }
        for each in backward.clone(){
            if !words_dict.contains(&each){
                oov_bmm+=1;
            }
        }
        if oov_fmm>oov_bmm{
            score_bmm+=1;
        }
        if oov_fmm<oov_bmm{
            score_fmm+=1;
        }
        if tot_fmm>tot_bmm{
            score_bmm+=1;
        }else if tot_fmm<tot_bmm{
            score_fmm+=1;
        }

        if f_single_word>b_single_word{
            score_bmm+=1;
        }else if f_single_word<b_single_word{
            score_fmm+=1;
        }

        if score_fmm<score_bmm{
            return forward;
        }else{
            return backward;
        }

    }
}

///
///
/// Get meaningful word list from non-english text like Chinese with the use of unicode-segmentation. (Not char list)
///
pub fn get_word_list(str:&str)->Vec<&str>{
    // let s = "我喜欢吃苹果，也爱打羽毛球";
    let g = str.graphemes(true).collect::<Vec<&str>>();
    // println!("{:?}",g);
    return g;
}

///
/// Word Segmentation Based on Backward Maximum Matching
///
pub fn bmm(sentence:Vec<&str>,dict:Vec<String>)->Vec<String>{

    let mut list_words:Vec<String>=Vec::new();
    let mut index:i32=sentence.len() as i32;
    let window_size:i32=4;
    while index>0{
        let mut match_flag=false;
        let mut i=window_size.clone();
        // println!("i={i}");
        while i>=0{
            // println!("i={}",i);
            let a;
            if index-i<0{
                a=0 as usize;
            }else{
                a=(index-i) as usize;
            }
            // let a = (index-i) as usize;
            let b =index as usize;
            // println!("({},{})",a,b);
            let sub_str=sentence[a..b].concat();
            if dict.contains(&sub_str) {
                match_flag = true;
                list_words.push(sub_str);
                index -= i;
                break;
            }
            i-=1;
        }
        if match_flag==false{

            if index-1<0{
                index=1;
            }
            let a=(index-1) as usize;
            list_words.push(String::from(sentence[a]));
            index-=1;
        }
    }
    list_words.reverse();
    return list_words;

}

///
/// Word Segmentation Based on Forward Maximum Matching
///
pub fn fmm(sentence:Vec<&str>,dict:Vec<String>)->Vec<String>{
    let token_len=sentence.len() as i32;
    // println!("token len: {}",token_len);
    let mut index:i32=0;

    let mut list_words:Vec<String>=Vec::new();
    let window_size=4;
    /*
    for char in sentence.chars(){
        println!("{}",char);
    }
     */
    while index<token_len{
        let mut match_flag=false;
        let mut i=window_size.clone();
        while i>=0{
            //println!("i={}",i);

            let a=index as usize;
            let mut b=(index+i) as usize;
           // println!("({},{})",a,b);
            if b>(token_len) as usize{
                b=token_len as usize;
            }
            let sub_str=sentence[a..b].concat();
            //println!("sub_str: {}",sub_str);
            if dict.contains(&sub_str){
                match_flag=true;
                list_words.push(sub_str);
                index+=i;
                break;
            }
            //println!();
            i-=1;
        }
        if match_flag==false{
            let a=index as usize;
            let v=String::from(sentence[a]);
            list_words.push(v);
            index+=1;
        }

    }
    return list_words;
}

use std::io::{self, BufRead};
use std::fs::{File, read_dir};
use std::io::prelude::*;
use std::path::Path;

fn _read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

///
/// Read a list of lines from a file
///
pub fn load_dictionary(filepath:&str)->Vec<String>{
    // The output is wrapped in a Result to allow matching on errors
    // Returns an Iterator to the Reader of the lines of the file.

    let mut strings=Vec::new();

    if let Ok(lines) = _read_lines(filepath) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                // println!("{}", ip);
                //let word=line.replace("\n","").trim();
                strings.push(String::from(line.replace("\n","").trim()));
            }
        }
    }
    strings
}



