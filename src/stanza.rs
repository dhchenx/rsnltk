use std::any::Any;
use std::collections::HashMap;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

///
/// This function expands a raw token into multiple syntactic words, which makes it easier to carry out Universal Dependencies analysis in some languages.
/// https://stanfordnlp.github.io/stanza/mwt.html
///
pub fn mwt_expand(text:&str,lang:&str)->Vec<Vec<HashMap<String,String>>>{
    match _mwt_expand(text,lang){
        Ok(t)=>{
            // println!("{:?}",t);
            t
        },
        Err(e)=>{
            println!("{:?}",e);
            Vec::new()
        }
    }
}


fn _mwt_expand(text:&str,lang:&str) -> PyResult<Vec<Vec<HashMap<String,String>>>> {

    let py_tokenize=concat!(
    "from nerkit.StanzaApi import StanzaWrapper\n",
    "def mwt_expand(text,lang):\n",
    "\tsw=StanzaWrapper()\n",
    "\treturn sw.mwt_expand(text=text,lang=lang)\n"
    );

    // "from nerkit.StanzaApi import StanzaWrapper\ndef getNER(text_en,a):\n\tsw=StanzaWrapper()\n\treturn sw.ner(text_en)"
    let kwargs = vec![
        ( "text", text),
        ("lang",lang)
    ];

    Python::with_gil(|py| {
        let results:Vec<Vec<HashMap<String,String>>>= PyModule::from_code(
            py,
            py_tokenize,
            "",
            "",
        )?.getattr("mwt_expand")?.call((),Some(kwargs.into_py_dict(py)))?.extract()?;

        // println!("{:?}",results);

        Ok(results)
    })
}

///
/// Download a list of necessary language models for the first time you use this toolkit!
///
pub fn download_langs(list_lang:Vec<&str>){
    for lang in list_lang{
        let flag=download_lang(lang);
        if flag{
            println!("Downloaded successfully!")
        }else{
            println!("Downloaded failed!")
        }
    }
}


///
/// Download a necessary language model for the first time you use this toolkit!
///
pub fn download_lang(lang:&str)->bool{
    match _download_lang(lang){
        Ok(t)=>{
            // println!("{:?}",t);
            t
        },
        Err(e)=>{
            println!("{:?}",e);
            false
        }
    }
}

fn _download_lang(lang:&str) -> PyResult<bool> {

    let py_tokenize=concat!(
    "from nerkit.StanzaApi import StanzaWrapper\n",
    "def download_lang(lang):\n",
    "\tflag=False\n",
    "\tsw=StanzaWrapper()\n",
    "\tsw.download(lang=lang)\n",
    "\treturn not False\n"
    );

    // "from nerkit.StanzaApi import StanzaWrapper\ndef getNER(text_en,a):\n\tsw=StanzaWrapper()\n\treturn sw.ner(text_en)"
    let kwargs = vec![
        ( "lang", lang)
    ];

    Python::with_gil(|py| {
        let downloaded:bool= PyModule::from_code(
            py,
            py_tokenize,
            "",
            "",
        )?.getattr("download_lang")?.call((),Some(kwargs.into_py_dict(py)))?.extract()?;

        // println!("{:?}",downloaded);

        Ok(downloaded)
    })
}


///
/// Detect the language of text and route texts of different languages to different language specific pipelines.
/// https://stanfordnlp.github.io/stanza/langid.html
///
pub fn lang(list_str:Vec<&str>)->Vec<HashMap<String,String>>{


    match _lang(list_str){
        Ok(t)=>{
            // println!("{:?}",t);
            t
        },
        Err(e)=>{
            println!("{:?}",e);
            Vec::new()
        }
    }
}

fn _lang(list_text:Vec<&str>) -> PyResult<Vec<HashMap<String,String>>> {

    let py_tokenize=concat!(
    "from nerkit.StanzaApi import StanzaWrapper\n",
    "def lang(list_text):\n",
    "\tsw=StanzaWrapper()\n",
    "\treturn sw.lang(list_text=list_text)\n"
    );

    // "from nerkit.StanzaApi import StanzaWrapper\ndef getNER(text_en,a):\n\tsw=StanzaWrapper()\n\treturn sw.ner(text_en)"
    let kwargs = vec![
        ( "list_text", list_text)
    ];

    Python::with_gil(|py| {
        let tags:Vec<HashMap<String,String>>= PyModule::from_code(
            py,
            py_tokenize,
            "",
            "",
        )?.getattr("lang")?.call((),Some(kwargs.into_py_dict(py)))?.extract()?;

        // println!("{:?}",tags);

        Ok(tags)
    })
}



///
/// The Part-of-Speech (POS) & morphological features tagging module labels words with their universal POS (UPOS) tags, treebank-specific POS (XPOS) tags, and universal morphological features (UFeats)
/// https://stanfordnlp.github.io/stanza/pos.html
///
pub fn pos(text:&str,lang:&str)->Vec<HashMap<String,String>>{

    let tag_result;
    if lang=="zh"{
        tag_result=_pos_chinese(text,lang)
    }else{
        tag_result=_pos(text,lang)
    }

    match tag_result{
        Ok(t)=>{
            println!("{:?}",t);
            t
        },
        Err(e)=>{
            println!("{:?}",e);
            Vec::new()
        }
    }
}

fn _pos(text:&str,lang:&str) -> PyResult<Vec<HashMap<String,String>>> {

    let py_tokenize=concat!(
    "from nerkit.StanzaApi import StanzaWrapper\n",
    "def tag(text,lang):\n",
    "\tsw=StanzaWrapper()\n",
    "\treturn sw.tag(text=text,lang=lang)\n"
    );

    // "from nerkit.StanzaApi import StanzaWrapper\ndef getNER(text_en,a):\n\tsw=StanzaWrapper()\n\treturn sw.ner(text_en)"
    let kwargs = vec![
        ( "text", text),
        ( "lang", lang)
    ];

    Python::with_gil(|py| {
        let tags:Vec<HashMap<String,String>>= PyModule::from_code(
            py,
            py_tokenize,
            "",
            "",
        )?.getattr("tag")?.call((),Some(kwargs.into_py_dict(py)))?.extract()?;

        println!("{:?}",tags);

        Ok(tags)
    })
}

fn _pos_chinese(text:&str,lang:&str) -> PyResult<Vec<HashMap<String,String>>> {

    let py_tokenize=concat!(
    "from nerkit.StanzaApi import StanzaWrapper\n",
    "def tag_chinese(text,lang):\n",
    "\tsw=StanzaWrapper()\n",
    "\treturn sw.tag_chinese(text=text,lang=lang)\n"
    );

    // "from nerkit.StanzaApi import StanzaWrapper\ndef getNER(text_en,a):\n\tsw=StanzaWrapper()\n\treturn sw.ner(text_en)"
    let kwargs = vec![
        ( "text", text),
        ( "lang", lang)
    ];

    Python::with_gil(|py| {
        let tags:Vec<HashMap<String,String>>= PyModule::from_code(
            py,
            py_tokenize,
            "",
            "",
        )?.getattr("tag_chinese")?.call((),Some(kwargs.into_py_dict(py)))?.extract()?;

        println!("{:?}",tags);

        Ok(tags)
    })
}


///
/// The SentimentProcessor adds a label for sentiment to each Sentence. The existing models each support negative, neutral, and positive, represented by 0, 1, 2 respectively.
/// https://stanfordnlp.github.io/stanza/sentiment.html
///
pub fn sentiment(text:&str,lang:&str)->Vec<HashMap<String,String>>{
    match _sentiment(text,lang){
        Ok(t)=>{
            println!("{:?}",t);
            t
        },
        Err(e)=>{
            println!("{:?}",e);
            Vec::new()
        }
    }
}

fn _sentiment(text:&str,lang:&str) -> PyResult<Vec<HashMap<String,String>>> {

    let py_tokenize=concat!(
    "from nerkit.StanzaApi import StanzaWrapper\n",
    "def sentiment(text,lang):\n",
    "\tsw=StanzaWrapper()\n",
    "\treturn sw.sentiment(text=text,lang=lang)\n"
    );

    // "from nerkit.StanzaApi import StanzaWrapper\ndef getNER(text_en,a):\n\tsw=StanzaWrapper()\n\treturn sw.ner(text_en)"
    let kwargs = vec![
        ( "text", text),
        ( "lang", lang)
    ];

    Python::with_gil(|py| {
        let sentiments:Vec<HashMap<String,String>>= PyModule::from_code(
            py,
            py_tokenize,
            "",
            "",
        )?.getattr("sentiment")?.call((),Some(kwargs.into_py_dict(py)))?.extract()?;

        println!("{:?}",sentiments);

        Ok(sentiments)
    })
}

///
/// It performs tokenization and sentence segmentation at the same time.
/// https://stanfordnlp.github.io/stanza/tokenize.html
///
pub fn tokenize(text:&str,lang:&str)->Vec<Vec<HashMap<String,String>>>{
    match _tokenize(text,lang){
        Ok(t)=>{
            println!("{:?}",t);
            t
        },
        Err(e)=>{
            println!("{:?}",e);
            Vec::new()
        }
    }
}



fn _tokenize(text:&str,lang:&str) -> PyResult<Vec<Vec<HashMap<String,String>>>> {

    let py_tokenize=concat!(
    "from nerkit.StanzaApi import StanzaWrapper\n",
    "def tokenize(text,lang):\n",
    "\tsw=StanzaWrapper()\n",
    "\treturn sw.tokenize(text=text,lang=lang)\n"
    );

    // "from nerkit.StanzaApi import StanzaWrapper\ndef getNER(text_en,a):\n\tsw=StanzaWrapper()\n\treturn sw.ner(text_en)"
    let kwargs = vec![
        ( "text", text)
        , ("lang", lang)
    ];

    Python::with_gil(|py| {
        let list_result:Vec<Vec<HashMap<String,String>>> = PyModule::from_code(
            py,
            py_tokenize,
            "",
            "",
        )?.getattr("tokenize")?.call((),Some(kwargs.into_py_dict(py)))?.extract()?;

        println!("{:?}",list_result);

        Ok(list_result)
    })
}

///
/// You can perform tokenization without sentence segmentation, as long as the sentences are split by two continuous newlines (\n\n) in the raw text.
///
pub fn tokenize_sentence(text:&str,lang:&str)->Vec<String>{
    match _tokenize_sentence(text,lang){
        Ok(t)=>{
            println!("{:?}",t);
            t
        },
        Err(e)=>{
            println!("{:?}",e);
            Vec::new()
        }
    }
}

fn _tokenize_sentence(text:&str,lang:&str) -> PyResult<Vec<String>> {

    let py_tokenize=concat!(
    "from nerkit.StanzaApi import StanzaWrapper\n",
    "def tokenize(text,lang):\n",
    "\tsw=StanzaWrapper()\n",
    "\treturn sw.tokenize_sentence(text=text,lang=lang)\n"
    );

    // "from nerkit.StanzaApi import StanzaWrapper\ndef getNER(text_en,a):\n\tsw=StanzaWrapper()\n\treturn sw.ner(text_en)"
    let kwargs = vec![
        ( "text", text)
        , ("lang", lang)
    ];

    Python::with_gil(|py| {
        let list_result:Vec<String> = PyModule::from_code(
            py,
            py_tokenize,
            "",
            "",
        )?.getattr("tokenize_sentence")?.call((),Some(kwargs.into_py_dict(py)))?.extract()?;

        println!("{:?}",list_result);

        Ok(list_result)
    })
}

///
/// The named entity recognition (NER) module recognizes mention spans of a particular entity type (e.g., Person or Organization) in the input sentence.
/// https://stanfordnlp.github.io/stanza/ner.html
///
pub fn ner(text:&str,lang:&str)->Vec<HashMap<String,String>>{
    match _ner(text,lang){
        Ok(t)=>{
            //println!("{:?}",t);
            t
        },
        Err(e)=>{
            println!("{:?}",e);
            Vec::new()
        }
    }
}

fn _ner(text:&str,lang:&str) -> PyResult<Vec<HashMap<String,String>>> {

    let py_ner=concat!(
    "from nerkit.StanzaApi import StanzaWrapper\n",
    "def getNER(text,lang):\n",
    "\tsw=StanzaWrapper()\n",
    "\treturn sw.ner(text=text,lang=lang)\n"
    );

    // "from nerkit.StanzaApi import StanzaWrapper\ndef getNER(text_en,a):\n\tsw=StanzaWrapper()\n\treturn sw.ner(text_en)"
    let kwargs = vec![(
                          "text", text)
                      , ("lang", lang)
    ];

    Python::with_gil(|py| {
        let list_result:Vec<HashMap<String,String>> = PyModule::from_code(
            py,
            py_ner,
            "",
            "",
        )?.getattr("getNER")?.call((),Some(kwargs.into_py_dict(py)))?.extract()?;

        println!("{:?}",list_result);

        Ok(list_result)
    })
}

