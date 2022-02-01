extern crate word2vec;

use word2vec::wordclusters::WordClusters;
use word2vec::wordvectors::WordVector;

#[cfg(test)]
mod tests{
    use std::borrow::Borrow;
    use crate::native::word2vec::*;

    # [test]
    fn test_word2vec(){
            //this is an issue to fix
            let clusters=wv_clusters_create("D:\\UIBEResearch\\classes.txt");
            let index=wv_get_cluster_from_clusters(clusters,"problem");
            println!("index = {}",index);
    }

    # [test]
    fn test_open_wv_bin(){
        let wv_model=wv_get_model("D:\\UIBEResearch\\GoogleNews-vectors-negative300.bin\\GoogleNews-vectors-negative300.bin");
        let positive = vec!["woman", "king"];
        let negative = vec!["man"];
        println!("analogy: {:?}", wv_analogy(&wv_model,positive, negative, 10));
        println!("cosine: {:?}", wv_cosine(&wv_model,"man", 10));
    }

    # [test]
    fn test_origin(){
/*
    let model = word2vec::wordvectors::WordVector::load_from_binary(

        "D:\\UIBEResearch\\GoogleNews-vectors-negative300.bin\\GoogleNews-vectors-negative300.bin").expect("Unable to load word vector model");
    println!("{:?}", model.cosine("snow", 10));
    let positive = vec!["woman", "king"];
    let negative = vec!["man"];
    println!("{:?}", model.analogy(positive, negative, 10));
*/


        let clusters = word2vec::wordclusters::WordClusters::load_from_file(
            "D:\\UIBEResearch\\classes1.txt").expect("Unable to load word clusters");
        println!("{:?}", clusters.get_cluster("belarus"));
        println!("{:?}", clusters.get_words_on_cluster(6));


    }

}

pub fn wv_get_model(bin_path:&str)->WordVector{
    let model = word2vec::wordvectors::WordVector::load_from_binary(
        bin_path).expect("Unable to load word vector model");
    return model
}


///
///     let model = word2vec::wordvectors::WordVector::load_from_binary(
///         "vectors.bin").expect("Unable to load word vector model");
///     println!("{:?}", model.cosine("snow", 10));
///
///
pub fn wv_cosine(model:&WordVector,word:&str,n:usize)->Vec<(String,f32)>{

    let ret=model.cosine(word,n);
    match ret {
        Some(r)=>{
            r
        },
        None=>{
            Vec::new()
        },
    }
}

///
///     let positive = vec!["woman", "king"];
///     let negative = vec!["man"];
///     println!("{:?}", model.analogy(positive, negative, 10));
///
pub fn wv_analogy(model:&WordVector, positive:Vec<&str>,negative:Vec<&str>,n:usize)->Vec<(String,f32)>{
    let re=model.analogy(positive, negative, n);
    // println!("{:?}",re );
    match re{
        Some(v)=>v,
        None=>{
            eprintln!("error");
            Vec::new()
        }
    }
}

///
///
/// let clusters = word2vec::wordclusters::WordClusters::load_from_file(
///         "classes.txt").expect("Unable to load word clusters");
///     println!("{:?}", clusters.get_cluster("belarus"));
///     println!("{:?}", clusters.get_words_on_cluster(6));
///
///
pub fn wv_clusters_create(filepath:&str)->WordClusters{
    let clusters = word2vec::wordclusters::WordClusters::load_from_file(
        filepath).expect("Unable to load word clusters");
    return clusters;
}

///
///  println!("{:?}", clusters.get_words_on_cluster(6));
///
pub fn wv_get_cluster_from_clusters(clusters:WordClusters,word:&str)->i32{
    match clusters.get_cluster(word){
        Some(&v)=>{
            v
        },
        None=>{
            println!("error");
            -1
        }
    }
}

///
/// println!("{:?}", clusters.get_cluster("belarus"));
///
pub fn wv_get_cluster_string(clusters:WordClusters,index:i32)->Vec<String>{
    match clusters.get_words_on_cluster(index){
        Some(v)=>{
            v.clone()
        },
        None=>{
            println!("error");
            Vec::new()
        }
    }
}


fn main(){

}