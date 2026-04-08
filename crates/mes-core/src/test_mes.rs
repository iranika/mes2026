
/*
#[cfg(test)]
mod mes_unit_tests {
    
    #[test]
    fn test_counter(){
        let text = std::fs::read_to_string("tests/SampleMimeyScript.txt").unwrap();
        let result = crate::mes::countDialogueWordToJson(&text);
        println!("{}",result);
    }
    #[test]
    fn test_flatDialogue(){
        let text = std::fs::read_to_string("tests/AfterTheMonday/CommonScript.txt").unwrap();
        let mut rawMedo = crate::mes::parseRawMedo(&text);
        rawMedo.toflat_dialogue();
        let res = rawMedo.parse_to_medo();
        //println!("{:?}", res.body.pieces);
        
        for ele in res.body.pieces {
            println!("\"{}\",\"{}\",\"{}\",\"{}\"", ele.charactor,ele.dialogue,ele.comments,ele.sound_note);            
        }   
    }
    #[test]
    fn test_parseRawMedo(){
        //let text = std::fs::read_to_string("tests/AfterTheMonday/CommonScript.txt").unwrap();
        let text = std::fs::read_to_string("tests/SampleCommonScript.txt").unwrap();
        let mut rawMedo = crate::mes::parseRawMedo(&text);

        println!("header: {:?}", rawMedo.header);
    }

    #[test]
    fn test_countDialogueWordToJsonWithConf(){
        let mut text: String = std::fs::read_to_string("tests/IgnoreStringSample.txt").unwrap();
        let text2 = text.clone();
        let result = crate::mes::countDialogueWordToJson(&text);
        let iresult = crate::mes::countDialogueWordToJsonWithConf(text2, crate::mes::get_default_config());
        println!("result{}", result);
        println!("iresult{}", iresult);
        
    }

}
 */