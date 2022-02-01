#[cfg(test)]
mod tests {
    use rsnltk::native::token::*;
    use rsnltk::native::summarizer::*;
    # [test]
    fn test_get_token_list(){
        let s="Hello, Rust. How are you?";
        let result=get_token_list(s);
        for r in result{
            println!("{}\t{:?}",r.text,r);
        }
    }

    # [test]
    fn test_summarize(){
        let text="As of Sunday, there were more than 58.2 million reported cases of COVID-19 worldwide, with more than 37.2 million of those cases listed as recovered, according to a COVID-19 tracking tool maintained by Johns Hopkins University. The global death toll stood at more than 1.3 million. In Asia, the daily tally of reported cases in Japan hit a record for the fourth day in a row, with 2,508 people confirmed infected, the Health Ministry said Sunday. A flurry of criticism has erupted, from opposition legislators and the public, slamming the government as having acted too slowly in halting its \"GoTo\" campaign, which encouraged travel and dining out with discounts. In Europe, French authorities ordered the culling of all minks at a farm after analysis showed a mutated version of the coronavirus was circulating among the animals. The move follows virus developments in mink farms in Denmark and other countries, including the Netherlands, Sweden and Greece. In the Americas, Chile says it will open its main border crossing and principal airport to foreign visitors on Monday after an eight-month pandemic shutdown. Arrivals will have to present evidence of a recent negative test for the novel coronavirus, as well as health insurance. They'll also have to report their whereabouts and health status for a two-week watch period. Those coming from high-risk countries will have to quarantine for 14 days. In Africa, Sudan's minister of cabinet affairs on Sunday tested positive for the coronavirus, the prime minister's office said, the latest in a string of senior officials to be infected as the country shows an increase of confirmed cases of COVID-19. Over the past month, acting ministers of finance and health, the central bank governor and two associates to Prime Minister Abdalla Hamdok have tested positive.";
        let stopwords=&[];
        let summarized_text=summarize(text,stopwords,5);
        println!("{}",summarized_text);
    }

}