fn main() {
    let n = std::time::Instant::now();
    let word = "If you wish to make an apple pie from scratch, you must first invent the universe.";
    let result = irc::encode(10, word);
    println!("{:?}", result);
    println!("{:?}", n.elapsed())
}

mod irc {
    use itertools::Itertools;
    use std::iter::FromIterator;

    pub fn encode(n: u32, qoute: &str) -> String {
        let mut message = qoute.replace('\n', "0"); //

        // get the spaces

        // remove spaces
        for _ in 1..=n {
            let spaces = message.char_indices().filter(|(_, c)| c == &' ');
            let mut no_spaces: Vec<char> = message.replace(" ", "").chars().collect();
            // rotate the string to the right by 10
            let _rotate = if n as usize > no_spaces.len() {
                n as usize % no_spaces.len()
            } else {
                n as usize
            };
            no_spaces.rotate_right(n as usize);

            // place back the spaces
            spaces.for_each(|(ind, s)| {
                no_spaces.insert(ind, s);
            });
            //

            message = String::from_iter(no_spaces)
                .split_ascii_whitespace()
                .map(|s| get_shifted_str_right(s, _rotate, true))
                .join(" ");
            println!("{}", message);
        }
        format!("{}", format_args!("{} {}", n, message.replace('0', "\n")))
    }

    #[allow(dead_code, unused_variables)]
    #[inline]
    pub fn get_shifted_str_right(sum: &str, right: usize, to_right: bool) -> String {
        // get an ar
        let mut arr: Vec<_> = sum.chars().collect();

        let right = if right > arr.len() && arr.len() > 0 {
            right % arr.len()
        } else {
            right
        };
        if to_right {
            arr.rotate_right(right);
        } else {
            arr.rotate_left(right);
        }

        arr.iter().collect()
    }

    #[allow(dead_code, unused_variables)]
    #[inline]
    pub fn decode(encoded: &str) -> String {
        // extract n from str
        let mut arr = encoded.split(' ');
        // num
        let n = arr.nth(0).unwrap().parse::<usize>().unwrap();
        // message

        let mut message = arr.join(" ");
        let mut count = 1;
        while count <= n {
            message = message
                .split(' ')
                .map(|e| get_shifted_str_right(e, n, false))
                .join(" ");
            // get the space and indices
            let spaces = message.char_indices().filter(|(_, c)| c == &' ');
            // chars without spaces
            // remove spaces
            let mut no_spaces: Vec<char> = message.replace(" ", "").chars().collect();
            // rotate no_spaces to the left by n
            let rotate = if n as usize > no_spaces.len() {
                n as usize % no_spaces.len()
            } else {
                n as usize
            };
            no_spaces.rotate_left(rotate);
            // replace the spaces back

            spaces.for_each(|(ind, s)| no_spaces.insert(ind, s));
            message = no_spaces.iter().collect();
            //println!("{}",message);
            //
            count += 1;
        }
        message
    }
}

#[cfg(test)]
mod example_tests {
    use super::*;

    #[test]
    fn example_tests() {
        let encode_examples: Vec<(u32,&str)> = vec![
             (10,"If you wish to make an apple pie from scratch, you must first invent the universe."),
             (14,"True evil is a mundane bureaucracy."),
             (22,"There is nothing more atrociously cruel than an adored child."),
             (36,"As I was going up the stair\nI met a man who wasn't there!\nHe wasn't there again today,\nOh how I wish he'd go away!"),
             (29,"I avoid that bleak first hour of the working day during which my still sluggish senses and body make every chore a penance.\nI find that in arriving later, the work which I do perform is of a much higher quality.")
         ];
        let decode_examples = vec![
             "10 hu fmo a,ys vi utie mr snehn rni tvte .ysushou teI fwea pmapi apfrok rei tnocsclet",
             "14 daue ilev is a munbune Traurecracy.",
             "22 tareu oo iucnaTr dled oldthser.hg hiarm nhcn se rliyet oincoa",
             "36 ws h weA dgIaa ug owh n!asrit git \n msm phw teaI'e tanantwhe reos\ns ther! aHeae 'gwadin\nt haw n htoo ,I'i sy aohOy",
             "29 a r.lht niou gwryd aoshg gIsi mk lei adwhfci isd seensn rdohy mo kleie oltbyhes a\naneu p.n rndr tehh irnne yifav t eo,raclhtc frpw IIti im gwkaidhv aicufh ima doea eruhi y io qshhcoa kr ef l btah gtrrse otnvugrt"
         ];

        for ((n, s1), s2) in encode_examples.iter().zip(decode_examples.iter()) {
            assert_eq!(&irc::encode(*n, s1), s2);
            assert_eq!(&irc::decode(s2), s1);
        }
    }
}
