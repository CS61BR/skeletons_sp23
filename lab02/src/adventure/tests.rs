use std::io::{BufRead, Error, Read};

use crate::{
    bee_counting_stage::play_bee_counting_stage, machine_stage::play_machine_stage,
    palindrome_stage::play_palindrome_stage, species_list_stage::play_species_list_stage,
};

/// A BufRead implementer that panics on EOF,
/// to prevent infinite loops in tests
struct LimitedBufReader<'a> {
    data: &'a [u8]
}

impl<'a> Read for LimitedBufReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.data.read(buf)
    }
}

impl<'a> BufRead for LimitedBufReader<'a> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        if self.data.len() == 0 {
            panic!("End of data reached");
        }
        Ok(self.data)
    }

    fn consume(&mut self, amt: usize) {
        self.data.consume(amt)
    }
}

fn test_stage<F>(stage: F, input: &str, expected_output: &str)
where
    F: Fn(&mut LimitedBufReader, &mut Vec<u8>) -> Result<(), Error>,
{
    let mut input = LimitedBufReader {
        data: input.as_bytes(),
    };
    let mut output = Vec::new();

    stage(&mut input, &mut output).expect("unexpected io error");
    let output = String::from_utf8(output).expect("output is not utf8");
    assert_eq!(output, expected_output)
}

#[test]
fn bee_counting() {
    test_stage(
        |r, w| play_bee_counting_stage(r, w),
        "3\n2\n6\ngo\n",
        "\
In Soda 326, you can find the computers known as \"The Hive\". It is a \
little-known fact that they are called this because they are home to \
(friendly) robotic bees. How many bees do you see?
 -.- -.- -.-
How about now?
 -.- -.-
How about now?
 -.- -.- -.- -.- -.- -.-
Those sure were some bees!
Phew, that was a lot of counting! It's time for Professor Hug's office hours! \
Let's head up to his office on the 7th floor.
>> [go]\n",
    );
}

#[test]
fn species_list_correct_inputs() {
    test_stage(
        |r, w| play_species_list_stage(r, w),
        "leopards, bison\nsquirrels, hummingbirds\n\ngo\n", 
        "\
Inside Professor Hug's office, you see some O'Reilly books. These books have \
cool animals on the covers. As a budding computer scientist, you should be \
able to identify all kinds of neat animals. Here's a few:
- These large felines with spots will teach you how to react quickly.
- This native american bovine can be found in the plains, and happens to be EXTREMELY good at Java.
Type their names into the terminal (separated by ',')
Woah! There are even more neat books here!
- These bushy-tailed friends are everywhere in and around the trees on campus, and know the best parts of Java.
- These tiny birds flap very fast, drink nectar, and know how to make simpler Java applications.
Type their names into the terminal (separated by ',')
Well, there's nothing left here! press enter to move.
Wow! That was pretty neat! We got to see so many neat animals! We should study now, so let's go to the Woz.
>> [go]\n");
}

#[test]
fn species_list_incorrect_inputs() {
    test_stage(
        |r, w| play_species_list_stage(r, w),
        "cheese, bison\nleopards, bison\nsquirrels, hummingbirds\n\ngo\n", 
        "\
Inside Professor Hug's office, you see some O'Reilly books. These books have \
cool animals on the covers. As a budding computer scientist, you should be \
able to identify all kinds of neat animals. Here's a few:
- These large felines with spots will teach you how to react quickly.
- This native american bovine can be found in the plains, and happens to be EXTREMELY good at Java.
Type their names into the terminal (separated by ',')
Try again! You got 1 animals correct!
Woah! There are even more neat books here!
- These bushy-tailed friends are everywhere in and around the trees on campus, and know the best parts of Java.
- These tiny birds flap very fast, drink nectar, and know how to make simpler Java applications.
Type their names into the terminal (separated by ',')
Well, there's nothing left here! press enter to move.
Wow! That was pretty neat! We got to see so many neat animals! We should study now, so let's go to the Woz.
>> [go]\n");
}

#[test]
fn palindrome() {
    test_stage(
        |r, w| play_palindrome_stage(r, w),
        "1112\n12345654321\nva cafe\ngo\n",
        "\
The Woz is a cool name, much better than \"Soda 430\". Soda 606 is a neat, \
symmetric conference room, like its number. We could rename The Woz to be \
palindromic, but it sounds so cool! Why not change the room number instead?
(Give a palindromic room number.)
That's not a palindrome! Try again.
Wow, nice room number!
Hmm, you're getting hungry. Where do you want to go?
>> [va cafe] [hearst food court]
Mm, tasty. Briefly, you wonder if free will is an illusion. You hear some \
people talking about a machine in Soda and decide to check it out.
>> [go]\n",
    );
}

#[test]
fn machine() {
    test_stage(
        |r, w| play_machine_stage(r, w),
        "1,5,4,-3,7,2\n-3,6,2,-5,11,1\nexperiment\n-1,3,-4,5,-100\n\
        -1,5,2,6,-100\nmove on\nzoom\n",
        "\
On the first (zeroth?) floor of Soda, below the labs, you find a mysterious \
machine. It has holes for two lists of ints of the same length, and a third \
hole that looks like it would output a number. The label reads:

'SumOfElementWiseMax-inator'

... Huh. You decide to experiment with the machine for a bit.
Enter a sequence of ints, separated by commas:
Enter a second sequence, with the same number of ints:
The machine whirrs briefly before outputting a slip of paper, reading 21
Do you want to continue experimenting?
>> [experiment] [move on]
Enter a sequence of ints, separated by commas:
Enter a second sequence, with the same number of ints:
The machine whirrs briefly before outputting a slip of paper, reading -88
Do you want to continue experimenting?
>> [experiment] [move on]
It's almost time for lecture! How are you attending?
>> [li ka shing] [zoom]
You find an open seat, set up your laptop, and join the Zoom.\n",
    );
}
