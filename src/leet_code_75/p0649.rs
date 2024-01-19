pub fn predict_party_victory(senate: String) -> String {

        let mut current = senate;
        let mut next = String::new();
        let mut votes = vec![];

        loop {
            for c in current.chars() {
                if votes.is_empty() {
                    votes.push(c);
                    next.push(c);
                } else if votes[votes.len() - 1] == c {
                    votes.push(c);
                    next.push(c);
                } else {
                    votes.pop();
                }
            }
            current = next;
            next = String::new();
            if !current.contains('R') {
                return "Dire".to_string();
            } else if !current.contains('D') {
                return "Radiant".to_string();
            }
        }

       
}

#[test]
pub fn test(){
    println!("{}", predict_party_victory("RDD".to_string()));
}