#[cfg(test)]
mod tests{
    use rsnltk::native::segmentation::*;
    extern crate  unicode_segmentation;
    use unicode_segmentation::UnicodeSegmentation;
    # [test]
    fn test_utf8(){
        let s = "我喜欢吃苹果，也爱打羽毛球";
        let g = s.graphemes(true).collect::<Vec<&str>>();
        println!("{:?}",g);
    }

    # [test]
    fn test_bmm(){
        let sss=String::from("我喜欢吃苹果，也爱打羽毛球");
        let sentence = sss.graphemes(true).collect::<Vec<&str>>();

        let dict=vec!["我".to_string(),"喜欢".to_string(),"苹果".to_string(),"羽毛球".to_string(),"爱".to_string()];

        let results=bmm(sentence,dict);

        println!("{:?}",results);
    }

    # [test]
    fn test_fmm(){
        let sss=String::from("我喜欢吃苹果，也爱打羽毛球");
        let sentence = sss.graphemes(true).collect::<Vec<&str>>();

        let dict=vec!["我".to_string(),"喜欢".to_string(),"苹果".to_string(),"羽毛球".to_string(),"爱".to_string()];

        let result=fmm(sentence,dict);

        println!("{:?}",result)

    }

    # [test]
    fn test_bimm(){
        let sss=String::from("我喜欢吃苹果，也爱打羽毛球");
        let sentence = sss.graphemes(true).collect::<Vec<&str>>();

        let dict=vec!["我".to_string(),"喜欢".to_string(),"苹果".to_string(),"羽毛球".to_string(),"爱".to_string()];

        let result=bimm(sentence,dict);

        println!("{:?}",result)
    }

    # [test]
    fn test_real_word_segmentation(){
        let dict_path="D:\\GitHub\\rsnltk\\experiments\\rsnltk-experiment\\examples\\data\\dicts\\30wdict.txt";
        let stop_path="D:\\GitHub\\rsnltk\\experiments\\rsnltk-experiment\\examples\\data\\dicts\\stopwords\\baidu_stopwords.txt";

        // let dict_path="";
        // let stop_path="";

        let _sentence="美国太空总署希望，在深海的探险发现将有助于解开一些外太空的秘密，同时也可以测试前往太阳系其他星球探险所需的一些设备和实验。";
        let meaningful_words=get_segmentation(_sentence,dict_path,stop_path, "");

        println!("Result: {:?}",meaningful_words);
    }

    # [test]
    fn test_segmentation_performance(){
        use std::time::{Duration, Instant};
        // set a dictionary
        let dict_path="D:\\GitHub\\rsnltk\\experiments\\rsnltk-experiment\\examples\\data\\dicts\\30wdict.txt";
        let stop_path="D:\\GitHub\\rsnltk\\experiments\\rsnltk-experiment\\examples\\data\\dicts\\stopwords\\baidu_stopwords.txt";
        // target sentence
        let _sentence="美国太空总署希望，在深海的探险发现将有助于解开一些外太空的秘密，同时也可以测试前往太阳系其他星球探险所需的一些设备和实验。";
        // start to time recording
        let mut start = Instant::now();
        let bimm_result=get_segmentation(_sentence,dict_path,stop_path, "bimm");
        println!("bimm's time cost: {:?}",start.elapsed());
        start = Instant::now();
        let fmm_result=get_segmentation(_sentence,dict_path,stop_path, "fmm");
        println!("fmm's time cost: {:?}",start.elapsed());
        start = Instant::now();
        let bmm_result=get_segmentation(_sentence,dict_path,stop_path, "bmm");
        println!("bmm's time cost: {:?}",start.elapsed());

    }
}